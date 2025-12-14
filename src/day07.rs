use std::collections::{hash_set, HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};
use crate::Day;

pub struct Day07;

pub struct BitArray {
    values: Vec<u64>,
}

impl BitArray {
    pub fn new(values: Vec<bool>) -> BitArray {
        let width = values.len();
        let width_with_padding = width;  // add left padding
        let words_per_row = (width_with_padding / 64) + 1;
        let mut data = vec![0u64; words_per_row];
        let mut x = 0;
        for i in values {
            if i {
                let actual_x = x; // Offset by 1 for left padding
                let idx = actual_x / 64;
                let bit = actual_x % 64;
                data[idx] |= 1u64 << bit;
            }
            x += 1;
        }

        BitArray { values: data }
    }

    pub fn new_empty(width: usize) -> BitArray {
        let width_with_padding = width;  // add left padding
        let words_per_row = (width_with_padding / 64) + 1;
        BitArray { values: vec![0u64; words_per_row] }
    }

    pub fn set(&mut self, index: usize) {
        let idx = index / 64;
        let bit = index % 64;
        self.values[idx] |= 1 << bit;
    }

    pub fn and(&self, other: &BitArray) -> BitArray {
        BitArray { values: self.values.iter()
            .zip(other.values.iter())
            .map(|(a, b)| a & b)
            .collect::<Vec<_>>() }
    }

    pub fn or(&self, other: &BitArray) -> BitArray {
        BitArray {
            values: self.values.iter()
                .zip(other.values.iter())
                .map(|(a, b)| a | b)
                .collect::<Vec<_>>()
        }
    }

    pub fn xor(&self, other: &BitArray) -> BitArray {
        BitArray {
            values: self.values.iter()
                .zip(other.values.iter())
                .map(|(a, b)| a ^ b)
                .collect::<Vec<_>>()
        }
    }

    pub fn shift_left(&self) -> BitArray {
        let mut result = vec![0u64; self.values.len()];

        for i in 0..self.values.len() {
            // Shift current word left by 1
            result[i] = self.values[i] << 1;

            // Carry the highest bit from the previous word
            if i > 0 {
                result[i] |= self.values[i - 1] >> 63;
            }
        }

        BitArray { values: result }
    }

    pub fn shift_right(&self) -> BitArray {
        let mut result = vec![0u64; self.values.len()];

        for i in 0..self.values.len() {
            // Shift current word right by 1
            result[i] = self.values[i] >> 1;

            // Carry the lowest bit from the next word
            if i + 1 < self.values.len() {
                result[i] |= self.values[i + 1] << 63;
            }
        }

        BitArray { values: result }
    }

    pub fn count_ones(&self) -> u32 {
        self.values.iter().map(|&a| a.count_ones()).sum()
    }
}

impl Debug for BitArray {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        let total_bits = self.values.len() * 64;
        for i in 0..total_bits {
            let idx = i / 64;
            let bit = i % 64;
            let bit_value = (self.values[idx] >> bit) & 1;
            write!(f, "{}", bit_value)?;

            // Optional: add space every 8 bits for readability
            // if (i + 1) % 8 == 0 && i + 1 < total_bits {
            //     write!(f, " ")?;
            // }
        }

        write!(f, "]")
    }
}


impl Day<Vec<Vec<bool>>, usize> for Day07 {
    fn parse_input(&self, input: &str) -> Vec<Vec<bool>> {
        input
            .lines()
            .map(|l| l.chars().map(|c| c == '^').collect::<Vec<bool>>())
            .collect::<Vec<Vec<bool>>>()
    }
    
    fn part1(&self, input: &Vec<Vec<bool>>) -> usize {
        let width = input[0].len();

        let test = input
            .iter()
            .map(|l| BitArray::new(l.clone()))
            .collect::<Vec<BitArray>>();

        let mut splits = 0;

        let mut scan_line = BitArray::new_empty(width);

        scan_line.set(width / 2);

        for line in test {
            // First part: how many are hitting
            // sl:     010
            // li:     010 &
            // result: 010
            let hits = line.and(&scan_line);

            splits += hits.count_ones() as usize;

            // Shift left and right all hits
            let shifted_left = hits.shift_left();
            let shifted_right = hits.shift_right();

            // OR both shifted
            scan_line = scan_line
                .or(&shifted_left)
                .or(&shifted_right)
                .xor(&hits); // XOR with hits to get new scan_line
        }

        splits // 1672
    }
    
    fn part2(&self, input: &Vec<Vec<bool>>) -> usize {
        let mut input = input.clone();
        let height = input.len();
        let width = input[0].len();

        let y = 0;
        let x = width / 2;



        let mut cache: HashMap<(usize, usize), usize> = HashMap::new();

        calc_splits(&input, (y, x), &mut cache)
    }
}

fn calc_splits(grid: &Vec<Vec<bool>>, (y, x): (usize, usize), cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if y >= grid.len() {
        1
    } else if grid[y][x] {
        if let Some(value) = cache.get(&(y, x)) {
            *value
        } else {
            let left = if x + 1 < grid[y].len() { calc_splits(grid, (y, x + 1), cache) } else { 0 };
            let right = if x > 0 { calc_splits(grid, (y, x - 1), cache) } else { 0 };

            cache.insert((y, x), left + right);

            left + right
        }
    } else {
        calc_splits(grid, (y + 1, x), cache)
    }
}
