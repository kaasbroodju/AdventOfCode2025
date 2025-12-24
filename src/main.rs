// #![feature(portable_simd)]
// use std::simd::*;

use std::fs;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

use day01::Day01;
use day02::Day02;
use day03::Day03;
use day04::Day04;
use day05::Day05;
use day06::Day06;
use day07::Day07;
use day08::Day08;
use day09::Day09;
use day10::Day10;
use day11::Day11;
use day12::Day12;

/// Generic trait for Advent of Code day solutions
pub trait Day<T, U: std::fmt::Debug> {
    /// Parse file input into the problem's data structure
    fn parse_input(&self, input: &str) -> T;
    
    /// Solve part 1 of the day's challenge
    fn part1(&self, input: &T) -> U;
    
    /// Solve part 2 of the day's challenge
    fn part2(&self, input: &T) -> U;

    /// Execute the complete solution: parse, solve both parts, and print results
    fn solve(&self, file_path: &str) {
        let input = fs::read_to_string(file_path)
            .expect(&format!("Failed to read file: {}", file_path));

        let parsed = self.parse_input(&input);

        let timer = std::time::Instant::now();
        let result1 = self.part1(&parsed);
        println!("Part 1: {:?} (took {:?})", result1, timer.elapsed());

        let timer = std::time::Instant::now();
        let result2 = self.part2(&parsed);
        println!("Part 2: {:?} (took {:?})", result2, timer.elapsed());
    }
}

fn main() {
    const DAY: u8 = 10;
    
    
    // let DAY: u8 = args[1].parse().expect("Day must be a number between 1 and 12");
    
    if DAY < 1 || DAY > 12 {
        eprintln!("Day must be between 1 and 12");
        std::process::exit(1);
    }
    
    println!("Advent of Code 2025 - Day {}", DAY);
    println!("---");
    
    let file_path = format!("inputs/day{:02}", DAY);
    
    match DAY {
        1 => Day01.solve(&file_path),
        2 => Day02.solve(&file_path),
        3 => Day03.solve(&file_path),
        4 => Day04.solve(&file_path),
        5 => Day05.solve(&file_path),
        6 => Day06.solve(&file_path),
        7 => Day07.solve(&file_path),
        8 => Day08.solve(&file_path),
        9 => Day09.solve(&file_path),
        10 => Day10.solve(&file_path),
        11 => Day11.solve(&file_path),
        12 => Day12.solve(&file_path),
        _ => unreachable!(),
    }
}
