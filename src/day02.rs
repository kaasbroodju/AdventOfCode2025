use rayon::iter::ParallelIterator;
use std::time::Instant;
use rayon::prelude::ParallelBridge;
use crate::Day;

pub struct Day02;

impl Day<Vec<(usize, usize)>, usize> for Day02 {
    fn parse_input(&self, input: &str) -> Vec<(usize, usize)> {
        input
            .split(",")
            .map(|line| line.split_once("-").unwrap())
            .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
            .collect::<Vec<_>>()
    }

    fn part1(&self, input: &Vec<(usize, usize)>) -> usize {

        input
            .iter()
            .par_bridge()
            .map(|&(encoded_start, encoded_end)| {
                let mut total = 0;

                let mut encoded = encode_digits(encoded_start);
                let mut n = encoded_start;
                let mut digit_count = (n.ilog10() + 1) as usize;

                // Pre-compute wanneer digit_count verandert
                let mut next_boundary = 10usize.pow(digit_count as u32);

                while n <= encoded_end {
                    // Check of we een digit boundary passeren
                    if n == next_boundary {
                        digit_count += 1;
                        next_boundary *= 10;
                    }

                    // Gebruik gecachte digit_count
                    let half = digit_count / 2;
                    let half_bits = half * 4;
                    let mask = (1u64 << half_bits) - 1;

                    let right_half = encoded & mask;
                    let left_half = (encoded >> half_bits) & mask;

                    total += (digit_count % 2 == 0 && left_half == right_half) as usize * n;

                    n += 1;
                    encoded = increment_encoded(encoded, digit_count + 1);
                }

                total
            })
            .sum::<usize>()
    }

    fn part2(&self, input: &Vec<(usize, usize)>) -> usize {

        input
            .iter()
            .par_bridge()
            .map(|&(range_start, range_end)| {
                let mut total = 0;

                let mut encoded = encode_digits(range_start);
                let mut n = range_start;
                let mut digit_count = (n.ilog10() + 1) as usize;

                // Pre-compute wanneer digit_count verandert
                let mut next_boundary = 10usize.pow(digit_count as u32);

                while n <= range_end {
                    // Check of we een digit boundary passeren
                    if n == next_boundary {
                        digit_count += 1;
                        next_boundary *= 10;
                        // Re-encode bij boundary cross (nieuw cijfer erbij)
                        encoded = encode_digits(n);
                    }

                    // Gebruik gecachte digit_count
                    for &pattern_size in divisors_of(digit_count) {
                        // Stap 1: Extract pattern
                        let pattern_bits = pattern_size * 4;
                        let pattern_mask = (1u64 << pattern_bits) - 1;
                        let pattern = encoded & pattern_mask;

                        // Stap 2: Herhaal pattern over hele lengte
                        let mut replicated = 0u64;
                        let total_bits = digit_count * 4;

                        for shift in (0..total_bits).step_by(pattern_bits) {
                            replicated |= pattern << shift;
                        }

                        // Stap 3: Vergelijk met origineel
                        let full_mask = (1u64 << total_bits) - 1;
                        let result = (encoded & full_mask) == (replicated & full_mask);

                        if result {
                            total += n;
                            break;  // Match gevonden, stop met divisors zoeken
                        }
                    }

                    n += 1;

                    // Alleen increment als we niet over boundary gaan
                    if n < next_boundary {
                        encoded = increment_encoded(encoded, digit_count + 1);
                    }
                }

                total
            })
            .sum::<usize>()
    }
}

fn encode_digits(mut n: usize) -> u64 {
    let mut result = 0u64;
    let mut shift = 0;

    // Extract digits van rechts naar links
    while n > 0 {
        let digit = (n % 10) as u64;
        result |= digit << shift;
        shift += 4;
        n /= 10;
    }

    result
}

fn _divisors(n: usize) -> Vec<usize> {
    if n == 0 { return vec![]; }
    if n == 1 { return vec![1]; }

    let mut divs = Vec::new();
    let sqrt_n = (n as f64).sqrt() as usize;

    for i in 1..=sqrt_n {
        if n % i == 0 {
            divs.push(i);

            // Voeg ook de "partner" deler toe
            let partner = n / i;
            if partner != i {  // Vermijd duplicaten bij perfecte kwadraten
                divs.push(partner);
            }
        }
    }

    divs.sort_unstable();
    divs
}

const fn divisors_of(n: usize) -> &'static [usize] {
    match n {
        1 => &[],
        2 => &[1],
        3 => &[1],
        4 => &[1, 2],
        5 => &[1],
        6 => &[1, 2, 3],
        7 => &[1],
        8 => &[1, 2, 4],
        9 => &[1, 3],
        10 => &[1, 2, 5],
        11 => &[1],
        12 => &[1, 2, 3, 4, 6],
        13 => &[1],
        14 => &[1, 2, 7],
        15 => &[1, 3, 5],
        16 => &[1, 2, 4, 8],
        _ => &[],
    }
}

fn increment_encoded(encoded: u64, digit_count: usize) -> u64 {
    let lowest = encoded & 0xF;
    if lowest < 9 {
        return encoded + 1;
    }

    // Slow path: carry propagation
    let mut result = 0u64;
    let mut carry = 1u64;

    for pos in 0..digit_count {
        let shift = pos * 4;
        let digit = ((encoded >> shift) & 0xF) + carry;

        if digit >= 10 {
            result |= 0 << shift;
            carry = 1;
        } else {
            result |= digit << shift;
            carry = 0;

            if carry == 0 && pos + 1 < digit_count {
                let remaining_mask = !((1u64 << (shift + 4)) - 1);
                result |= encoded & remaining_mask;
                return result;
            }
        }
    }

    result
}