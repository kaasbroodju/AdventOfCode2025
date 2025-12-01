use std::ops::{Add, Rem, Sub};
use crate::Day;

pub enum Direction {
    Left = -1,
    Right = 1,
}

pub struct Day01;

const CLOCKSIZE: i32 = 100;
const STARTING_POSITION: i32 = 50;


impl Day<Vec<(i32, i32)>, i32> for Day01 {
    fn parse_input(&self, input: &str) -> Vec<(i32, i32)> {
        // Example: parse lines as integers
        input.lines()
            .map(|line| {
                let direction = if line[0..1].eq("L") {
                    -1i32
                } else {
                    1i32
                };

                let count = line[1..].parse::<i32>().unwrap();

                (direction, count)
            })
            .collect()
    }
    
    fn part1(&self, input: &Vec<(i32, i32)>) -> i32 {
        let mut position = STARTING_POSITION;
        let mut total = 0;
        for (direction, count) in input {
            position = (position + direction * (count % CLOCKSIZE)).rem_euclid(CLOCKSIZE);
            
            // Branchless: add 1 if position == 0, else add 0
            total += (position == 0) as i32;
        }

        total
    }

    fn part2(&self, input: &Vec<(i32, i32)>) -> i32 {
        let mut position = STARTING_POSITION;
        let mut total = 0;
        let mut rotational_total = 0;

        for (direction, count) in input {
            let previous_position_is_zero = position == 0;
            // Count full rotations from the count
            total += count / CLOCKSIZE;

            // Calculate new position
            position = position + direction * (count % CLOCKSIZE);

            // Count if we landed on zero
            total += (position == 0) as i32;

            // Check if we crossed zero during the move
            // For Left: if new_position < 0 and previous wasn't zero, we crossed
            // For Right: if new_position >= CLOCKSIZE and previous wasn't zero, we crossed
            let is_crossing_boundary = ((position < 0) && (*direction == -1)) ||
                 ((position >= CLOCKSIZE) && (*direction == 1));

            total += (is_crossing_boundary && !previous_position_is_zero) as i32;

            // Wrap position using rem_euclid
            position = position.rem_euclid(CLOCKSIZE);
        }

        total
    }
}
