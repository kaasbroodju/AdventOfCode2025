use std::cmp::min;
use std::collections::HashSet;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use crate::Day;

pub struct LinkedListItem {

}

pub struct Day09;

impl Day<Vec<(usize, usize)>, usize> for Day09 {
    fn parse_input(&self, input: &str) -> Vec<(usize, usize)> {
        input
            .lines()
            .map(|line| line.split_once(",").unwrap())
            .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
            .collect::<Vec<(usize, usize)>>()
    }
    
    fn part1(&self, input: &Vec<(usize, usize)>) -> usize {
        // let mut distances = vec![];
        let length = input.len();
        let mut max = 0;

        for i in 0..length {
            for j in i + 1..length {
                let (ax, ay) = input[i];
                let (bx, by) = input[j];

                let distance = (ax.abs_diff(bx) + 1) * (ay.abs_diff(by) + 1);
                if distance > max {
                    max = distance;
                }
            }
        }

        max
    }
    
    fn part2(&self, input: &Vec<(usize, usize)>) -> usize {
        let mut lines = vec![];

        let mut cursor = input[0];
        for &to in input.iter().skip(1) {
            lines.push((cursor, to));
            cursor = to;
        }
        lines.push((cursor, input[0]));

        let (horizontal, vertical): (Vec<_>, Vec<_>) = lines
            .into_iter()
            .partition(|((_, a), (_, b))| a == b);

        let length = input.len();

        let mut ranges = (0..length)
            .flat_map(|i| (i + 1..length)
                .map(move |j| (i, j, (input[i].0.abs_diff(input[j].0) + 1) * (input[i].1.abs_diff(input[j].1) + 1))))
            .collect::<Vec<_>>();

        ranges.sort_by(|(_, _, a), (_, _, b)| b.cmp(a)); // Reversed sort voor descending

        // Parallel zoeken naar eerste zonder collision
        ranges
            .par_iter()
            .find_first(|&&(i, j, _)| !has_collision(&horizontal, &vertical, input[i], input[j]))
            .map(|&(_, _, area)| area)
            .unwrap_or(0)
    }
}

fn has_collision(
    horizontal: &Vec<((usize, usize), (usize, usize))>,
    vertical: &Vec<((usize, usize), (usize, usize))>,
    a: (usize, usize),
    b: (usize, usize)
) -> bool {
    let min_x = a.0.min(b.0);
    let max_x = a.0.max(b.0);
    let min_y = a.1.min(b.1);
    let max_y = a.1.max(b.1);

    // Filter alleen relevante horizontale lijnen
    for (from, to) in horizontal.iter()
        .filter(|((x1, y), (x2, _))| {
            let line_y = *y;
            let line_min_x = x1.min(x2);
            let line_max_x = x1.max(x2);
            line_y >= min_y && line_y <= max_y && line_max_x >= &min_x && line_min_x <= &max_x
        })
    {
        for x in min_x..=max_x {
            if crosses_horizontal((x, min_y), (x, max_y), *from, *to) {
                return true;
            }
        }
    }

    // Filter alleen relevante verticale lijnen
    for (from, to) in vertical.iter()
        .filter(|((x, y1), (_, y2))| {
            let line_x = *x;
            let line_min_y = y1.min(y2);
            let line_max_y = y1.max(y2);
            line_x >= min_x && line_x <= max_x && line_max_y >= &min_y && line_min_y <= &max_y
        })
    {
        for y in min_y..=max_y {
            if crosses_vertical((min_x, y), (max_x, y), *from, *to) {
                return true;
            }
        }
    }

    false
}

// fn has_collision(
//     horizontal: &Vec<((usize, usize), (usize, usize))>,
//     vertical: &Vec<((usize, usize), (usize, usize))>,
//     a: (usize, usize),
//     b: (usize, usize)
// ) -> bool {
//     let min_x = a.0.min(b.0);
//     let max_x = a.0.max(b.0);
//     let min_y = a.1.min(b.1);
//     let max_y = a.1.max(b.1);
//
//     for x in min_x..=max_x { // horizontal sweep, aka vertical lines
//         for (from, to) in horizontal {
//             if crosses_horizontal((x, min_y), (x, max_y), *from, *to) {
//                 return true;
//             }
//         }
//     }
//
//     for y in min_y..=max_y {
//         for (from, to) in vertical {
//             if crosses_vertical((min_x, y), (max_x, y), *from, *to) {
//                 return true;
//             }
//         }
//     }
//
//     false
// }

fn crosses_horizontal(
    from: (usize, usize),
    to: (usize, usize),
    from_polyglot: (usize, usize),
    to_polyglot: (usize, usize)
) -> bool {
    // ployglot is horizontal
    // from and to are vertical

    let crossing_x = from.0;
    let min_x = from_polyglot.0.min(to_polyglot.0);
    let max_x = from_polyglot.0.max(to_polyglot.0);

    let crossing_y = from_polyglot.1;
    let min_y = from.1.min(to.1);
    let max_y = from.1.max(to.1);

    // crossing_x < min_x || crossing_x > max_x
    return (crossing_x > min_x && crossing_x < max_x)
        && (crossing_y > min_y && crossing_y < max_y)
}

fn crosses_vertical(
    from: (usize, usize),
    to: (usize, usize),
    from_polyglot: (usize, usize),
    to_polyglot: (usize, usize)
) -> bool {
    // ployglot is vertical
    // from and to are horizontal
    let crossing_x = from_polyglot.0;
    let min_x = from.0.min(to.0);
    let max_x = from.0.max(to.0);

    let crossing_y = from.1;
    let min_y = from_polyglot.1.min(to_polyglot.1);
    let max_y = from_polyglot.1.max(to_polyglot.1);



    // crossing_x < min_x || crossing_x > max_x
    return (crossing_x > min_x && crossing_x < max_x)
        && (crossing_y > min_y && crossing_y < max_y)
}