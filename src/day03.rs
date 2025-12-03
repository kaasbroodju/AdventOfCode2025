use crate::Day;

pub struct Day03;

impl Day<Vec<Vec<u32>>, usize> for Day03 {
    fn parse_input(&self, input: &str) -> Vec<Vec<u32>> {
        input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }
    
    fn part1(&self, input: &Vec<Vec<u32>>) -> usize {
        let mut total = 0;


        for line in input {

            let mut lef_largest_number = 0;
            let mut left_largest_number_pos = 0;
            for (pos, x) in line.iter().rev().skip(1).rev().enumerate() {
                if *x > lef_largest_number {
                    lef_largest_number = *x;
                    left_largest_number_pos = pos;
                }
            }

            let mut right_largest_number = 0;
            for x in line.iter().skip(left_largest_number_pos + 1) {
                if *x > right_largest_number {
                    right_largest_number = *x;
                }
            }

            total += (lef_largest_number * 10) + right_largest_number;

        }

        total as usize
    }
    
    fn part2(&self, input: &Vec<Vec<u32>>) -> usize {
        const BUFFER_SIZE: usize = 12usize;
        let mut total = 0;

        for line in input {
            let mut numbers = [0; BUFFER_SIZE];

            let mut left_largest_number_pos = 0;
            for (pos, x) in line.iter().rev().skip(BUFFER_SIZE).rev().enumerate() {
                if *x > numbers[0] {
                    numbers[0] = *x;
                    left_largest_number_pos = pos;
                }
            }

            for i in 1..BUFFER_SIZE {
                for (pos, x) in line.iter().enumerate().rev().skip(BUFFER_SIZE - i - 1).rev().skip(left_largest_number_pos + 1) {
                    if *x > numbers[i] {
                        numbers[i] = *x;
                        left_largest_number_pos = pos;
                    }
                }
            }

            let parsed_number = format!("{:?}", numbers)
                .chars()
                .filter(char::is_ascii_digit)
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
            total += parsed_number;

        }

        total
    }
}
