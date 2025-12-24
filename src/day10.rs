use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Display, Formatter};
use rand::prelude::SliceRandom;
use crate::Day;

pub struct Day10;

pub struct Machine {
    target_lights: u16,
    buttons: Vec<u16>,
    voltage: Vec<usize>
}

impl Machine {
    pub fn new(l: &str) -> Machine {
        let (target_light_str, l) = l.split_once(" ").unwrap();


        let target_light_str = &target_light_str[1..target_light_str.len() - 1];
        let target_lights = target_light_str
            .chars()
            .enumerate()
            .fold(0u16, |acc, (i, c)| {
                if c == '#' {
                    return acc | (1 << i)
                }
                acc
            });
        let (buttons, voltage) = l.rsplit_once(" ").unwrap();

        let buttons = buttons
            .split(" ")
            .map(|button_str| {
                let button_str = &button_str[1..button_str.len() - 1];
                button_str
                    .split(',')
                    .fold(0u16, |acc, button| {
                        acc | (1 << button.parse::<u16>().unwrap())
                    })
            })
            .collect::<Vec<_>>();

        let voltage = &voltage[1..voltage.len() - 1];
        let voltage = voltage
            .split(',')
            .map(|voltage_str| voltage_str.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Machine { target_lights, buttons, voltage }
    }
}

impl Debug for Machine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n{:016b}\ttarget_light", self.target_lights)?;
        for x in &self.buttons {
            writeln!(f, "{:016b}", x)?;
        }
        Ok(write!(f, "")?)

    }
}

impl Day<Vec<Machine>, usize> for Day10 {
    fn parse_input(&self, input: &str) -> Vec<Machine> {
        input
            .lines()
            .map(|l| Machine::new(l))
            .collect::<Vec<Machine>>()
    }
    
    fn part1(&self, input: &Vec<Machine>) -> usize {
        input
            .iter()
            .map(|m| {
                get_minimum_bfs(m.target_lights, m.buttons.as_slice())
            })
            .sum::<usize>()
    }
    
    fn part2(&self, _input: &Vec<Machine>) -> usize {
        // TODO: Implement part 2 solution
        0
    }
}

fn get_minimum_bfs(start: u16, buttons: &[u16]) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();

    queue.push_back((start, 0u16, 0usize)); // (state, last_button, steps)

    while let Some((current, last_button, steps)) = queue.pop_front() {
        if current == 0 {
            return steps;
        }

        if steps > 10 {
            continue;
        }

        let key = (current, last_button);
        if visited.contains_key(&key) {
            continue;
        }
        visited.insert(key, steps);

        for &button in buttons {
            if button != last_button {
                queue.push_back((current ^ button, button, steps + 1));
            }
        }
    }

    usize::MAX
}