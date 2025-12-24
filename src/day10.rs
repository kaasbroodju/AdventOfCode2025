use std::collections::HashMap;
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

impl Day<Vec<Machine>, i32> for Day10 {
    fn parse_input(&self, input: &str) -> Vec<Machine> {
        input
            .lines()
            .map(|l| Machine::new(l))
            .collect::<Vec<Machine>>()
    }
    
    fn part1(&self, input: &Vec<Machine>) -> i32 {
        let length = input.len() as f64;
        let results = input
            .iter()
            .enumerate()
            .map(|(i, m)| {
                println!("{}", i as f64 / length * 100.0);
                get_minimum(&mut HashMap::new(), m.target_lights, 0u16, &m.buttons, 0)
            })
            .collect::<Vec<usize>>();

        println!("{:?}", results);

        results
            .iter()
            .sum::<usize>() as i32
        // println!("{:?}", input);
        // // TODO: Implement part 1 solution
        // 0
    }
    
    fn part2(&self, _input: &Vec<Machine>) -> i32 {
        // TODO: Implement part 2 solution
        0
    }
}

// fn get_minimum(cache: &mut HashMap<(u16, u16), usize>, current: u16, button_i_pressed: u16, buttons: &Vec<u16>, step: usize) -> usize {
//     if current == 0 {
//         cache.insert((current, button_i_pressed), step);
//         step
//     } else if step > 10 {
//         step
//     } else if let Some(&x) = cache.get(&(current, button_i_pressed)) {
//         step + x
//     } else {
//         buttons
//             .iter()
//             .filter(|&&b| b != button_i_pressed)
//             .map(|&b| get_minimum(cache, current ^ b, b, &buttons, step + 1))
//             .min()
//             .unwrap()
//     }
// }

fn get_minimum(
    cache: &mut HashMap<(u16, u16), usize>,
    current: u16,
    button_i_pressed: u16,
    buttons: &Vec<u16>,
    step: usize
) -> usize {
    // Base case
    if current == 0 {
        return step;
    }

    // Max depth pruning
    if step > 10 {
        return usize::MAX;
    }

    // Check cache BEFORE recursie
    let cache_key = (current, button_i_pressed);
    if let Some(&cached_steps) = cache.get(&cache_key) {
        return step + cached_steps;
    }

    // Bereken minimum
    let min_steps = buttons
        .iter()
        .filter(|&&b| b != button_i_pressed) // 2 keer xor resulteert in hetzelfde staat als voorheen, skip dus.
        .map(|&b| get_minimum(cache, current ^ b, b, buttons, step + 1))
        .min()
        .unwrap_or(usize::MAX);

    // Cache het resultaat (relatief tot huidige step)
    if min_steps != usize::MAX {
        cache.insert(cache_key, min_steps - step);
    }

    min_steps
}
