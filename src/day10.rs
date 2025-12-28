use rayon::iter::ParallelIterator;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Display, Formatter};
use rayon::prelude::IntoParallelRefIterator;
use crate::Day;

pub struct Day10;

pub struct Machine {
    target_lights: u16,
    buttons: Vec<u16>,
    voltage: u128,
}

const BITS_PER_BUTTON: usize = 12;

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
            .enumerate()
            .fold(0u128, |acc, (i, voltage_str)| acc | (voltage_str.parse::<u128>().unwrap() << (i * BITS_PER_BUTTON)));

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
    
    fn part2(&self, input: &Vec<Machine>) -> usize {
        input
            .par_iter()
            .map(|m| {
                let mut cache = HashMap::new();
                solve_recursive(&m.voltage, m.buttons.as_slice(), &mut cache)
            })
            .sum::<usize>()
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


fn solve_recursive(target: &u128, buttons: &[u16], cache: &mut HashMap<u128, usize>) -> usize {
    // Base case
    if *target == 0 {
        return 0;
    }

    // Check cache
    if let Some(&result) = cache.get(target) {
        return result;
    }

    // Stap 1: Bepaal de pariteit (welke lampjes moeten aan?)
    let parity_target = (0..10)
        .fold(0u16, |acc, i| {
            let val = get_value_at_position(target, i);
            if val % 2 == 1 {
                acc | (1 << i)
            } else {
                acc
            }
        });

    // Stap 2: Vind ALLE button combinaties die deze pariteit geven
    let valid_combos = find_all_parity_combos(parity_target, buttons);

    if valid_combos.is_empty() {
        // Onmogelijk
        cache.insert(*target, usize::MAX);
        return usize::MAX;
    }

    let mut best = usize::MAX;

    // Stap 3: Voor elke combo
    for (combo, num_buttons) in valid_combos {
        // Bereken wat overblijft na deze buttons 1x te drukken
        let mut remaining = [0u16; 10];
        let mut all_even = true;

        for i in 0..10 {
            let target_val = get_value_at_position(target, i);
            let mut pressed = 0;

            // Tel hoeveel keer deze position wordt verhoogd door de combo
            for button_idx in &combo {
                if (buttons[*button_idx] & (1 << i)) != 0 {
                    pressed += 1;
                }
            }

            if target_val < pressed {
                all_even = false;
                break;
            }

            remaining[i] = target_val - pressed;

            if remaining[i] % 2 != 0 {
                all_even = false;
                break;
            }
        }

        if !all_even {
            continue;
        }

        // Stap 4: Halveer en recurseer
        let halved = (0..10)
            .fold(0u128, |acc, i| {
                set_value_at_position(&acc, i, remaining[i] / 2)
            });

        let sub_result = solve_recursive(&halved, buttons, cache);

        if sub_result != usize::MAX {
            let total = 2 * sub_result + num_buttons;
            best = best.min(total);
        }
    }

    cache.insert(*target, best);
    best
}

fn find_all_parity_combos(target_parity: u16, buttons: &[u16]) -> Vec<(Vec<usize>, usize)> {
    let mut results = Vec::new();
    let n = buttons.len();

    // Probeer alle 2^n combinaties
    for mask in 0..(1 << n) {
        let mut current_parity = 0u16;
        let mut combo = Vec::new();

        for i in 0..n {
            if (mask & (1 << i)) != 0 {
                current_parity ^= buttons[i];
                combo.push(i);
            }
        }

        if current_parity == target_parity {
            results.push((combo.clone(), combo.len()));
        }
    }

    results
}

const MASK: u128 = (1 << BITS_PER_BUTTON) - 1; // 0b111111111111

fn get_value_at_position(state: &u128, position: usize) -> u16 {
    ((state >> (position * BITS_PER_BUTTON)) & MASK) as u16
}

fn set_value_at_position(state: &u128, position: usize, value: u16) -> u128 {
    let shift = position * BITS_PER_BUTTON;
    // Clear de oude waarde en set de nieuwe
    (state & !(MASK << shift)) | ((value as u128) << shift)
}