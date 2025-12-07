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
        if index >= GRID_SIZE {
            return false;
        }
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

                for (offset, bit) in OFFSETS {
                    if x & (1u8 << bit) != 0 { to_check.push(i.wrapping_add_signed(offset)); }
                }
            }
        }


        while let Some(i) = to_check.pop() {
            if !grid.get(i) { continue; }

            let x = check_neighbours(&grid, i);

            if x.count_ones() < 4 {
                total += 1;
                grid.set(i, false);

                for (offset, bit) in OFFSETS {
                    if x & (1u8 << bit) != 0 { to_check.push(i.wrapping_add_signed(offset)); }
                }
            }
        }

        total
    }
}

#[inline]
fn check_neighbours(grid: &BitArray, i: usize) -> u8 {
    let col = i % WIDTH;
    let at_left = (col == 0) as u8;
    let at_right = (col == WIDTH - 1) as u8;

    let left_mask = 0b10010100u8;   // Bits that need left-clear
    let right_mask = 0b00101001u8;  // Bits that need right-clear

    let valid_mask = u8::MAX
        & !(at_left * left_mask)
        & !(at_right * right_mask);

    let x =
        ((grid.get(i.wrapping_add_signed(-(WIDTH as isize) - 1)) as u8) << 7) |
        ((grid.get(i.wrapping_add_signed(-(WIDTH as isize))) as u8) << 6) |
        ((grid.get(i.wrapping_add_signed(-(WIDTH as isize) + 1)) as u8) << 5) |
        ((grid.get(i.wrapping_add_signed(-1)) as u8) << 4) |
        ((grid.get(i.wrapping_add_signed(1)) as u8) << 3) |
        ((grid.get(i.wrapping_add_signed(WIDTH as isize - 1)) as u8) << 2) |
        ((grid.get(i.wrapping_add_signed(WIDTH as isize)) as u8) << 1) |
        (grid.get(i.wrapping_add_signed(WIDTH as isize + 1)) as u8);

    x & valid_mask
}

const OFFSETS: [(isize, u8); 8] = [
    (-(WIDTH as isize) - 1, 7),  // top-left
    (-(WIDTH as isize),     6),  // top
    (-(WIDTH as isize) + 1, 5),  // top-right
    (-1,                    4),  // left
    (1,                     3),  // right
    (WIDTH as isize - 1,    2),  // bottom-left
    (WIDTH as isize,        1),  // bottom
    (WIDTH as isize + 1,    0),  // bottom-right
];
