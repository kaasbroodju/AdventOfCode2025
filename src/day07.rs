use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::Day;

pub struct Day07;

impl Day<Vec<Vec<char>>, usize> for Day07 {
    fn parse_input(&self, input: &str) -> Vec<Vec<char>> {
        input
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>()
    }
    
    fn part1(&self, input: &Vec<Vec<char>>) -> usize {
        let mut input = input.clone();
        let height = input.len();
        let width = input[0].len();

        let mut splits = 0;

        for y in 0..height {
            for x in 0..width {
                let c = input[y][x];
                let up = input.get(y.wrapping_sub_signed(1)).and_then(|l| l.get(x));

                if c == 'S' {
                    input[y + 1][x] = '|';
                } else if c == '^' && matches!(up, Some('|')) {
                    splits += 1;
                    if x + 1 < width {
                        if input[y][x + 1] != '|' {
                            input[y][x + 1] = '|';
                            // splits += 1;
                        }

                    }
                    if x > 0 {
                        if input[y][x - 1] != '|' {
                            input[y][x - 1] = '|';
                            // splits += 1;
                        }

                    }
                } else if c == '.' && matches!(up, Some('|')) {
                    input[y][x] = '|';
                }
            }
        }
        // for line in input {
        //     println!("{:?}", line);
        // }
        // TODO: Implement part 1 solution
        splits // 1672
    }
    
    fn part2(&self, input: &Vec<Vec<char>>) -> usize {
        let mut input = input.clone();
        let height = input.len();
        let width = input[0].len();

        let y = 0;
        let x = width / 2;



        let mut cache: HashMap<(usize, usize), usize> = HashMap::new();

        calc_splits(&input, (y, x), &mut cache)
    }
}

fn calc_splits(grid: &Vec<Vec<char>>, (y, x): (usize, usize), cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if y >= grid.len() {
        1
    } else if grid[y][x] == '^' {
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
