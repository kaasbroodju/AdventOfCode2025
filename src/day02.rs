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
            for i in a..=b {
                let s = i.to_string();
                let middle_point = s.len() / 2;
                let first_half = &s[..middle_point];
                let second_half = &s[middle_point..];
                if first_half.eq(second_half) {
                    total += i;
                }
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
