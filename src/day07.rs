use std::collections::{hash_set, HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};
use crate::Day;

pub struct Day07;

const WIDTH: usize = 141;
const BIT_ARRAY_SIZE: usize = (WIDTH / u64::BITS as usize) + 1;

pub struct BitArray {
    values: [u64; BIT_ARRAY_SIZE],
}

impl BitArray {
    pub fn new(values: Vec<bool>) -> BitArray {
        let mut data = [0u64; BIT_ARRAY_SIZE];

        for (x, &i) in values.iter().enumerate() {
            if i {
                let idx = x / 64;
                let bit = x % 64;
                data[idx] |= 1u64 << bit;
            }
        }

        BitArray { values: data }
    }

    pub fn new_empty() -> BitArray {
        BitArray { values: [0u64; BIT_ARRAY_SIZE] }
    }

    pub fn set(&mut self, index: usize) {
        let idx = index / 64;
        let bit = index % 64;
        self.values[idx] |= 1 << bit;
    }

    pub fn get(&self, index: usize) -> bool {
        let idx = index / 64;
        let bit = index % 64;
        self.values[idx] & (1 << bit) != 0
    }

    pub fn and(&self, other: &BitArray) -> BitArray {
        let mut result = [0u64; BIT_ARRAY_SIZE];
        for i in 0..BIT_ARRAY_SIZE {
            result[i] = self.values[i] & other.values[i];
        }
        BitArray { values: result }
    }

    pub fn or(&self, other: &BitArray) -> BitArray {
        let mut result = [0u64; BIT_ARRAY_SIZE];
        for i in 0..BIT_ARRAY_SIZE {
            result[i] = self.values[i] | other.values[i];
        }
        BitArray { values: result }
    }

    pub fn xor(&self, other: &BitArray) -> BitArray {
        let mut result = [0u64; BIT_ARRAY_SIZE];
        for i in 0..BIT_ARRAY_SIZE {
            result[i] = self.values[i] ^ other.values[i];
        }
        BitArray { values: result }
    }

    pub fn shift_left(&self) -> BitArray {
        let mut result = [0u64; BIT_ARRAY_SIZE];

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
        let mut result = [0u64; BIT_ARRAY_SIZE];

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

    // Maak een nieuwe BitArray met dezelfde size als een andere
    pub fn with_capacity_like(other: &BitArray) -> BitArray {
        BitArray {
            values: [0u64; BIT_ARRAY_SIZE]
        }
    }

    // Copy data from another BitArray
    pub fn copy_from(&mut self, other: &BitArray) {
        self.values.copy_from_slice(&other.values);
    }

    // In-place AND, result goes into self
    pub fn and_into(&mut self, a: &BitArray, b: &BitArray) {
        for i in 0..BIT_ARRAY_SIZE {
            self.values[i] = a.values[i] & b.values[i];
        }
    }

    // Combined operation: self = (self | shift_left(hits) | shift_right(hits)) ^ hits
    pub fn update_scan_line_inplace(&mut self, hits: &BitArray) {
        for i in 0..BIT_ARRAY_SIZE {
            let shifted_left = if i > 0 {
                (hits.values[i] << 1) | (hits.values[i - 1] >> 63)
            } else {
                hits.values[i] << 1
            };

            let shifted_right = if i + 1 < BIT_ARRAY_SIZE {
                (hits.values[i] >> 1) | (hits.values[i + 1] << 63)
            } else {
                hits.values[i] >> 1
            };

            self.values[i] = (self.values[i] | shifted_left | shifted_right) ^ hits.values[i];
        }
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

impl Day<Vec<BitArray>, usize> for Day07 {
    fn parse_input(&self, input: &str) -> Vec<BitArray> {
        input
            .lines()
            .map(|l| BitArray::new(l.chars().map(|c| c == '^').collect::<Vec<bool>>()))
            .collect::<Vec<_>>()
    }
    
    fn part1(&self, input: &Vec<BitArray>) -> usize {
        let mut splits = 0;
        let mut scan_line = BitArray::new_empty();
        let mut hits = BitArray::new_empty();  // Pre-allocate buffer

        scan_line.set(WIDTH / 2);

        for line in input {
            // Reuse hits buffer
            hits.and_into(line, &scan_line);

            splits += hits.count_ones() as usize;

            // Update scan_line in-place
            scan_line.update_scan_line_inplace(&hits);
        }

        splits
    }
    
    fn part2(&self, input: &Vec<BitArray>) -> usize {
        let mut acc = [1usize; WIDTH];

        for line in input.iter().rev() {
            for i in 1..WIDTH - 1 {
                let op = line.get(i);
                acc[i] = (op as usize * (acc[i - 1] + acc[i + 1])) + ((!op) as usize * acc[i]);
            }
        }

        acc[WIDTH / 2]
    }
}
