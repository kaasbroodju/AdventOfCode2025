use crate::Day;

pub struct Day01;

impl Day<Vec<i32>, i32> for Day01 {
    fn parse_input(&self, input: &str) -> Vec<i32> {
        // Example: parse lines as integers
        input.lines()
            .filter_map(|line| line.trim().parse().ok())
            .collect()
    }
    
    fn part1(&self, input: &Vec<i32>) -> i32 {
        // Example: sum all numbers
        input.iter().sum()
    }
    
    fn part2(&self, input: &Vec<i32>) -> i32 {
        // Example: product of all numbers
        input.iter().product()
    }
}
