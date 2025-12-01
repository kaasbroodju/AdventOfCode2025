use crate::Day;

pub enum Direction {
    Left, Right,
}

pub struct Day01;

impl Day<Vec<(Direction, i32)>, i32> for Day01 {
    fn parse_input(&self, input: &str) -> Vec<(Direction, i32)> {
        // Example: parse lines as integers
        input.lines()
            .map(|line| {
                let direction = if line[0..1].eq("L") {
                    Direction::Left
                } else {
                    Direction::Right
                };

                let count = line[1..].parse::<i32>().unwrap();

                (direction, count)
            })
            .collect()
    }
    
    fn part1(&self, input: &Vec<(Direction, i32)>) -> i32 {
        let mut position = 50i32;
        let mut total = 0;
        for (direction, count) in input {
            match direction {
                Direction::Left => position -= count % 100,
                Direction::Right => position += count % 100,
            }

            if position < 0 {
                position = 100 - position.abs()
            } else if position > 99 {
                position = (100 - position).abs()
            }

            if position == 0 {
                total += 1;
            }
        }

        total
    }
    
    fn part2(&self, input: &Vec<(Direction, i32)>) -> i32 {
        let mut position = 50i32;
        let mut total = 0;
        let mut rotational_total = 0;
        for (direction, count) in input {
            let previous_position_is_zero = position == 0;
            match direction {
                Direction::Left => position -= count % 100,
                Direction::Right => position += count % 100,
            }

            rotational_total += count / 100;

            if position == 0 || position == 100 {
                position = 0;
                total += 1;
            } else if position < 0 {
                if !previous_position_is_zero {
                    rotational_total += 1
                }
                position = 100 - position.abs()
            } else if position > 99 {
                if !previous_position_is_zero {
                    rotational_total += 1
                }
                position = (100 - position).abs()
            }
        }

       total + rotational_total
    }
}
