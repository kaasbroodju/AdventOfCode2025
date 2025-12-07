use crate::Day;

pub struct Day04;

const WIDTH: usize = 137;

const GRID_SIZE: usize = WIDTH * WIDTH;
const CHUNK_SIZE: usize = usize::BITS as usize;
const BIT_ARRAY_SIZE: usize = (GRID_SIZE / CHUNK_SIZE) + 1;

#[derive(Debug, Clone)]
pub struct BitArray {
    bits: [usize; BIT_ARRAY_SIZE]
}

impl BitArray {
    fn new() -> Self {
        Self { bits: [0usize; BIT_ARRAY_SIZE] }
    }

    fn set(&mut self, index: usize, value: bool) {
        let chunk_idx = index / CHUNK_SIZE;
        let bit_pos = index % CHUNK_SIZE;

        if value {
            self.bits[chunk_idx] |= 1 << bit_pos;  // Set bit to 1
        } else {
            self.bits[chunk_idx] &= !(1 << bit_pos);  // Clear bit to 0
        }
    }

    fn get(&self, index: usize) -> bool {
        self.bits[index / CHUNK_SIZE] & (1 << (index % CHUNK_SIZE)) != 0
    }
}

impl Day<BitArray, usize> for Day04 {
    fn parse_input(&self, input: &str) -> BitArray {
        let mut array = BitArray::new();
        let mut i = 0usize;

        for char in input.chars() {
            if char == '\n' || char == '\r' { continue; }
            array.set(i, char == '@');
            i += 1;
        }

        array
    }
    
    fn part1(&self, input: &BitArray) -> usize {
        let mut total = 0;

        for i in 0..GRID_SIZE {
            if !input.get(i) { continue; }

            let x = check_neighbours(&input, i);

            total += (x.count_ones() < 4) as usize
        }

        total
    }
    
    fn part2(&self, input: &BitArray) -> usize {
        let mut grid = input.clone();
        let mut total = 0;
        let mut to_check: Vec<usize> = vec![];

        for i in 0..GRID_SIZE {
            if !grid.get(i) { continue; }

            let x = check_neighbours(&grid, i);

            if x.count_ones() < 4 {
                total += 1;
                grid.set(i, false);

                if x & (1u8 << 7u8) != 0 { to_check.push(i - WIDTH - 1); }
                if x & (1u8 << 6u8) != 0 { to_check.push(i - WIDTH); }
                if x & (1u8 << 5u8) != 0 { to_check.push(i - WIDTH + 1); }

                if x & (1u8 << 4u8) != 0 { to_check.push(i - 1); }
                if x & (1u8 << 3u8) != 0 { to_check.push(i + 1); }

                if x & (1u8 << 2u8) != 0 { to_check.push(i + WIDTH - 1); }
                if x & (1u8 << 1u8) != 0 { to_check.push(i + WIDTH); }
                if x & 1u8 != 0 { to_check.push(i + WIDTH + 1); }
            }
        }


        while let Some(i) = to_check.pop() {
            if !grid.get(i) { continue; }

            let x = check_neighbours(&grid, i);

            if x.count_ones() < 4 {
                total += 1;
                grid.set(i, false);

                if x & (1u8 << 7u8) != 0 { to_check.push(i - WIDTH - 1); }
                if x & (1u8 << 6u8) != 0 { to_check.push(i - WIDTH); }
                if x & (1u8 << 5u8) != 0 { to_check.push(i - WIDTH + 1); }

                if x & (1u8 << 4u8) != 0 { to_check.push(i - 1); }
                if x & (1u8 << 3u8) != 0 { to_check.push(i + 1); }

                if x & (1u8 << 2u8) != 0 { to_check.push(i + WIDTH - 1); }
                if x & (1u8 << 1u8) != 0 { to_check.push(i + WIDTH); }
                if x & 1u8 != 0 { to_check.push(i + WIDTH + 1); }
            }
        }

        total
    }
}

#[inline]
fn check_neighbours(grid: &BitArray, i: usize) -> u8 {
    let mut x = 0u8;
    let empty_left_side = i % WIDTH == 0;
    let empty_right_side = (i + 1) % WIDTH == 0;

    let p_l_u = i as isize - WIDTH as isize - 1;
    let p_m_u = i as isize - WIDTH as isize;
    let p_r_u = i as isize - WIDTH as isize + 1;

    x |= if p_l_u >= 0 && !empty_left_side { (grid.get(p_l_u as usize) as u8 * (1u8 << 7u8)) } else { 0 };
    x |= if p_m_u >= 0 { (grid.get(p_m_u as usize) as u8 * (1u8 << 6u8)) } else { 0 };
    x |= if p_r_u > 0 && !empty_right_side { (grid.get(p_r_u as usize) as u8 * (1u8 << 5u8)) } else { 0 };

    let p_l_m = i as isize - 1;
    let p_r_m = i + 1;
    x |= if p_l_m >= 0 && !empty_left_side { (grid.get(p_l_m as usize) as u8 * (1u8 << 4u8)) } else { 0 };
    x |= if p_r_m < GRID_SIZE && !empty_right_side { (grid.get(p_r_m) as u8 * (1u8 << 3u8)) } else { 0 };

    let p_l_d = i + WIDTH - 1;
    let p_m_d = i + WIDTH;
    let p_r_d = i + WIDTH + 1;

    x |= if p_l_d < GRID_SIZE && !empty_left_side { (grid.get(p_l_d) as u8 * (1u8 << 2u8)) } else { 0 };
    x |= if p_m_d < GRID_SIZE { (grid.get(p_m_d) as u8 * (1u8 << 1)) } else { 0 };
    x |= if p_r_d < GRID_SIZE && !empty_right_side { (grid.get(p_r_d) as u8) } else { 0 };
    x
}

