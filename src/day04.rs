// #![feature(portable_simd)]
// use std::simd::*;

use crate::Day;

pub struct Day04;

const WIDTH: usize = 137;

const GRID_SIZE: usize = WIDTH * WIDTH;
const CHUNK_SIZE: usize = usize::BITS as usize;
const BIT_ARRAY_SIZE: usize = (GRID_SIZE / CHUNK_SIZE) + 1 + 4;

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

    // #[inline]
    // unsafe fn get_simd_chunks(&self, index: usize) -> Simd<usize, 4> {
    //     // Read 4 consecutive usize chunks from the bits array
    //     // let ptr = self.bits.as_ptr();
    //     // let chunks_ptr = ptr.unchecked_add(index);
    //
    //     let mut result = [0usize; 4];
    //     // std::ptr::copy_nonoverlapping(chunks_ptr, result.as_mut_ptr(), 4);
    //
    //     for offset in 0..64 * 4 {
    //         if !(index.wrapping_add(offset) >= GRID_SIZE) {
    //             if self.get(index.wrapping_add(offset)) {
    //                 result[offset / 64] |= (1 << (offset % 64))
    //             }
    //         }
    //     }
    //
    //     for x in result {
    //         println!("{x:064b}");
    //     }
    //
    //     Simd::from_array(result)
    // }

    // #[inline]
    // unsafe fn get_simd_chunks(&self, index: usize) -> Simd<usize, 4> {
    //     // Read 4 consecutive usize chunks from the bits array
    //     let ptr = self.bits.as_ptr();
    //     let chunks_ptr = ptr.add(index); // hier gaat het fout, volgens de documentatie is index * 64
    //     // ik wil dat het stappen van index neemt bijvoorbeeld 3 bits
    //     // maar wat het doet is index * 64 bits (64 bits ivm usize)
    //
    //     let mut result = [0usize; 4];
    //     std::ptr::copy_nonoverlapping::<usize>(chunks_ptr, result.as_mut_ptr(), 4);
    //     println!("{:?}", result);
    //     for offset in 0..64 * 4 {
    //         if index.wrapping_add(offset) >= GRID_SIZE {
    //             result[offset / 64] &= !(1 << (offset % 64));
    //         }
    //     }
    //
    //     Simd::from_array(result)
    // }

    // #[inline]
    // unsafe fn get_simd_chunks(&self, index: usize) -> Simd<usize, 4> {
    //     // Convert to byte pointer for fine-grained control
    //     let byte_ptr = self.bits.as_ptr() as *const u8;
    //
    //     // Calculate byte offset from bit index
    //     let byte_offset = index / 8;  // How many bytes to skip
    //     let bit_offset = index % 8;   // Remaining bit offset within byte
    //
    //     // Read 32 bytes (= 4 usize on 64-bit, = 256 bits)
    //     let bytes_ptr = byte_ptr.add(byte_offset);
    //     let mut bytes = [0u8; 32];
    //     std::ptr::copy_nonoverlapping(bytes_ptr, bytes.as_mut_ptr(), 32);
    //
    //     // Convert bytes back to usize array
    //     let mut result = [0usize; 4];
    //     for i in 0..4 {
    //         result[i] = usize::from_ne_bytes([
    //             bytes[i*8], bytes[i*8+1], bytes[i*8+2], bytes[i*8+3],
    //             bytes[i*8+4], bytes[i*8+5], bytes[i*8+6], bytes[i*8+7],
    //         ]);
    //     }
    //
    //     // TODO: Handle bit_offset shifting if needed
    //
    //     // Clear out-of-bounds bits
    //     for offset in 0..CHUNK_SIZE * 4 {
    //         if index.wrapping_add(offset) >= GRID_SIZE {
    //             result[offset / CHUNK_SIZE] &= !(1 << (offset % CHUNK_SIZE));
    //         }
    //     }
    //
    //     Simd::from_array(result)
    // }

    // #[inline]
    // unsafe fn get_simd_chunks(&self, bit_index: usize) -> Simd<usize, 4> {
    //     let byte_ptr = self.bits.as_ptr() as *const u8;
    //     let byte_offset = bit_index / 8;
    //     let bit_offset = bit_index % 8;
    // 
    //     // Read 33 bytes total (enough for 256 bits + 7 bits misalignment)
    //     let mut bytes = [0u8; 33];
    // 
    //     let max_byte = (GRID_SIZE + 7) / 8;
    //     let bytes_to_read = (byte_offset + 33).min(max_byte).saturating_sub(byte_offset);
    // 
    //     if bytes_to_read > 0 {
    //         unsafe {
    //             std::ptr::copy_nonoverlapping(
    //                 byte_ptr.add(byte_offset),
    //                 bytes.as_mut_ptr(),
    //                 bytes_to_read
    //             );
    //         }
    //     }
    // 
    //     let mut result = [0usize; 4];
    // 
    //     if bit_offset == 0 {
    //         // Byte-aligned: direct conversion
    //         for i in 0..4 {
    //             let base = i * 8;
    //             result[i] = usize::from_ne_bytes([
    //                 bytes[base], bytes[base+1], bytes[base+2], bytes[base+3],
    //                 bytes[base+4], bytes[base+5], bytes[base+6], bytes[base+7],
    //             ]);
    //         }
    //     } else {
    //         // Bit-misaligned: need shifting
    //         for i in 0..4 {
    //             let byte_base = i * 8;
    // 
    //             // Read 8 bytes + 1 extra for the shift spillover
    //             // Build two usize values and combine them
    //             let mut low = 0u64;
    //             let mut high = 0u64;
    // 
    //             // Read 8 bytes for low part
    //             for j in 0..8 {
    //                 if byte_base + j < 33 {
    //                     low |= (bytes[byte_base + j] as u64) << (j * 8);
    //                 }
    //             }
    // 
    //             // Read 1 byte for high part (the spillover)
    //             if byte_base + 8 < 33 {
    //                 high = bytes[byte_base + 8] as u64;
    //             }
    // 
    //             // Shift and combine
    //             // We want bits [bit_offset .. bit_offset+64) from the 72-bit value
    //             result[i] = ((low >> bit_offset) | (high << (64 - bit_offset))) as usize;
    //         }
    //     }
    // 
    //     // Clear out-of-bounds bits
    //     for offset in 0..CHUNK_SIZE * 4 {
    //         if bit_index.wrapping_add(offset) >= GRID_SIZE {
    //             result[offset / CHUNK_SIZE] &= !(1 << (offset % CHUNK_SIZE));
    //         }
    //     }
    // 
    //     Simd::from_array(result)
    // }
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
        // unsafe { println!("{:?}", input.get_simd_chunks(0usize.wrapping_add_signed(-20))); }  // Should work
        // unsafe { println!("{:?}", input.get_simd_chunks(137)); }  // Row 2
        // unsafe { println!("{:?}", input.get_simd_chunks(0)); }  // Should work
        // unsafe { println!("{:?}", input.get_simd_chunks(WIDTH)); }  // Row 2
        // unsafe { println!("{:?}", input.get_simd_chunks(GRID_SIZE + WIDTH + 1)); }
        // unsafe { println!("{:?}", input.get_simd_chunks(0usize.wrapping_add_signed(-32))); }
        let mut total = 0;

        for i in 0..GRID_SIZE {
            // if !input.get(i) { continue; }

            let x = check_neighbours(&input, i);

            total += (input.get(i) && x.count_ones() < 4) as usize
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

    // unsafe { println!("{:?}", grid.get_simd_chunks(i.wrapping_add_signed(WIDTH as isize + 1))); }  // Row 2

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
