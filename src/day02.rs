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
        let mut total = 0;

        for &(a, b) in input {
            for n in a..=b {

                // n = "2121"
                let encoded_number = encode_digits(n);
                // encoded_number = 0010 0001 0010 0001

                // Tel hoeveel cijfers n heeft
                let digit_count = (n.ilog10() + 1) as usize;

                // hoeveel bits neem ik mee voor de vergelijking?
                let half = digit_count / 2;
                let half_bits = half * 4;
                let mask = (1u64 << half_bits) - 1;
                // mask = 0000 0000 1111 1111

                // dmv de mask neem ik de rechter helft alleen mee.
                let right_half = encoded_number & mask;
                // right_half = 0010 0001

                // verplaats de linker helft naar de rechterkant en herhaal de vergelijking.
                let left_half = (encoded_number >> half_bits) & mask;

                // branchless programming: als het aantal cijfers even is en de linker en rechter helft overeenkomen, dan mag i opgeteld worden.
                total += (digit_count % 2 == 0 && left_half == right_half) as usize * n;
            }
        }

        total
    }
    
    fn part2(&self, input: &Vec<(usize, usize)>) -> usize {
        let mut total = 0;

        for &(a, b) in input {
            for i in a..=b {
                let s = i.to_string();
                let upperbound = s.len();
                let middle_point = upperbound / 2;

                let mut is_matching = false;

                'cursor_size_loop: for cursor_size in 1..=middle_point {
                    let pattern = &s[0..cursor_size];

                    for pos in (0..upperbound-cursor_size).step_by(cursor_size) {
                        let cursor_pos = pos + cursor_size;

                        // skip of cursor overflows
                        if cursor_pos + cursor_size > upperbound {
                            continue 'cursor_size_loop;
                        }

                        // continue and increase cursor size when pattern doesn't match
                        let possible_match = &s[cursor_pos..cursor_pos + cursor_size];
                        if pattern != possible_match {
                            continue 'cursor_size_loop;
                        }
                    }

                    // loop finished without breaking, so pattern matches
                    is_matching = true;
                    break;
                }

                if is_matching {
                    total += i;
                }
            }
        }

        total
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
