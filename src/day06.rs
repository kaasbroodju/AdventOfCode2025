use std::cmp::PartialEq;
use std::ops::Index;
use crate::Day;

#[derive(Debug, Eq, PartialEq)]
pub enum Op {
    Addition,
    Multiplication,
}

pub struct Day06;


impl Day<String, usize> for Day06 {
    fn parse_input(&self, input: &str) -> String {
        input.to_string()
    }
    
    fn part1(&self, input: &String) -> usize {
        let mut grid = vec![];
        let lines = input.lines().collect::<Vec<&str>>();
        let line_iter = lines.iter();
        let last = line_iter.clone().last().unwrap();
        let first = line_iter.clone().next().unwrap();
        let number_lines = line_iter.clone().skip(1).rev().skip(1).rev();

        for number in first.split(" ") {
            if number.is_empty() { continue; }
            let number = number.parse::<usize>().unwrap();
            grid.push(vec![number]);
        }

        for line in number_lines {
            let mut j = 0;
            for number in line.split(" ") {
                if number.is_empty() { continue; }
                let number = number.parse::<usize>().unwrap();
                grid.get_mut(j).unwrap().push(number);
                j += 1;
            }
        }

        let mut results = vec![];

        let mut i = 0;
        for op in last.split(" ") {
            if op.is_empty() { continue; }
            let op = if op[0..1].eq("+") { Op::Addition } else { Op::Multiplication };
            results.push((op, grid.get(i).unwrap().to_owned()));
            i += 1;
        }

        results
            .iter()
            .map(|(op, numbers)| {
                match op {
                    Op::Addition => {numbers.iter().fold(0, |acc, i| acc + i)}
                    Op::Multiplication => {numbers.iter().fold(1, |acc, i| acc * i)}
                }
            })
            .sum::<usize>()
    }

    fn part2(&self, input: &String) -> usize {
        let mut input = input.chars().as_str();
        let max_lines = input.len();
        let mut char_cursor = 0usize;
        let width = 3770;
        let height = 4;

        let mut result = 0usize;
        let mut op_cursor = max_lines - 1;
        while op_cursor > ((width + 1) * height) - 1 {

            let mut parsing_width = 0;
            while op_cursor > ((width + 1) * height) - 1 && !(input[op_cursor..=op_cursor].eq("+") || input[op_cursor..=op_cursor].eq("*")) {
                op_cursor -= 1;
                parsing_width += 1;
            }
            let op = if input[op_cursor..=op_cursor].eq("+") { Op::Addition } else { Op::Multiplication };

            parsing_width += 1;
            op_cursor += parsing_width;

            let mut acc = if op.eq(&Op::Addition) { 0 } else { 1 };
            for base in 0..parsing_width {
                let mut number = String::new();
                for i in (1..=height).rev() {
                    let x = op_cursor - ((width + 1) * i) - 1 - base;

                    let slice = &input[x..=x];

                    if !slice.eq(" ") { number.push_str(slice); }

                }
                let number = number.parse::<usize>().unwrap();
                acc = match op {
                    Op::Addition => { acc + number }
                    Op::Multiplication => { acc * number }
                };

            }

            result += acc;
            op_cursor -= (parsing_width + 2);
        }




        result
    }
}
