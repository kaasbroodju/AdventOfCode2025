use std::cmp::min;
use std::collections::HashSet;
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

        // distances.sort();

        // TODO: Implement part 1 solution
        max
    }
    
    fn part2(&self, input: &Vec<(usize, usize)>) -> usize {

        let mut lines = vec![];
        let mut maxes = vec![];

        // let mut from = input[0];
        let mut cursor = input[0];
        for &to in input.iter().skip(1) {
            // walk from a cordinates to b cordinates
            lines.push((cursor, to));
            cursor = to;
        }

        lines.push((cursor, input[0]));

        let length = input.len();
        let mut max = 0;
        let mut idk = 1.0;

        let ceiling = (length * (length + 1) / 2) as f64;

        for i in 0..length {
            for j in i + 1..length {
                // println!("{}", idk / ceiling * 100.0);
                let (ax, ay) = input[i];
                let (bx, by) = input[j];

                let area = (ax.abs_diff(bx) + 1) * (ay.abs_diff(by) + 1);

                if area > max {
                    if is_in_grid(&lines, (ax, ay), (bx, by)) {
                        println!("new MAX: {area}");
                        println!("idk: {:?}", ((ax, ay), (bx, by)));
                        maxes.push(area);
                        max = area;
                    }

                }

                idk += 1.0;
            }
        }

        println!("{:?}", maxes);

        // 4575309256 too high
        // 2377856740 too high
        // 458052838 too low
        // 116002341 stuck
        max
    }
}

fn is_in_grid(
    polygon: &Vec<((usize, usize), (usize, usize))>,
    a: (usize, usize),
    b: (usize, usize)
) -> bool {
    let min_x = a.0.min(b.0);
    let max_x = a.0.max(b.0);
    let min_y = a.1.min(b.1);
    let max_y = a.1.max(b.1);

    // Stap 1: Bereken het midden
    let mid_x = (min_x + max_x) / 2;
    let mid_y = (min_y + max_y) / 2;
    let center = (mid_x, mid_y);

    // Stap 2 & 3: Trek lijnen van midden naar 4 hoeken en check kruisingen
    let corners = [
        (min_x, min_y),  // top-left
        (max_x, min_y),  // top-right
        (min_x, max_y),  // bottom-left
        (max_x, max_y),  // bottom-right
    ];

    for &corner in &corners {
        if line_crosses_polygon(&polygon, &center, &corner) {
            return false;
        }
    }

    // Stap 4: Ga de rand af en check lijnen naar midden
    // Top rand
    for x in min_x+1..max_x {
        if line_crosses_polygon(polygon, &center, &(x, min_y)) {
            return false;
        }
    }

    // Bottom rand
    for x in min_x+1..max_x {
        if line_crosses_polygon(polygon, &center, &(x, max_y)) {
            return false;
        }
    }

    // Left rand
    for y in min_y+1..max_y {
        if line_crosses_polygon(polygon, &center, &(min_x, y)) {
            return false;
        }
    }

    // Right rand
    for y in min_y+1..max_y {
        if line_crosses_polygon(polygon, &center, &(max_x, y)) {
            return false;
        }
    }

    true
}

// fn line_crosses_polygon(polygon: &Vec<((usize, usize), (usize, usize))>, center: &(usize, usize), coordinate: &(usize, usize)) -> bool {
//     !polygon
//         .iter()
//         .all(|&(from, to)| {
//
//         })
// }

fn line_crosses_polygon(
    polygon: &Vec<((usize, usize), (usize, usize))>,
    center: &(usize, usize),
    border_coordinate: &(usize, usize)
) -> bool {
    polygon.iter().any(|&(poly_start, poly_end)| {
        segments_intersect_strict(*center, *border_coordinate, poly_start, poly_end)
    })
}

fn segments_intersect_strict(
    center: (usize, usize),
    border_coordinate: (usize, usize),
    line_start: (usize, usize),
    line_end: (usize, usize)
) -> bool {
    // This line is nearly always diagonal expect when the x or y are the same
    let (center_x, center_y) = (center.0 as i64, center.1 as i64);
    let (border_coordinate_x, border_coordinate_y) = (border_coordinate.0 as i64, border_coordinate.1 as i64);

    // These are always strict horizontal or strict vertical
    let (line_start_x, line_start_y) = (line_start.0 as i64, line_start.1 as i64);
    let (line_end_x, line_end_y) = (line_end.0 as i64, line_end.1 as i64);



    if line_start_y == line_end_y {
        // horizontal line

        if center_y == line_start_y {
            return true;
        }

        if line_start_y == border_coordinate_y {
            return false // line points exact on the line, so it does not interact
        }

        let slope = (center_y - border_coordinate_y) as f64 / (center_x - border_coordinate_x) as f64;

        // get (slope * x) + b = y
        let bias = slope * border_coordinate_x as f64;
        let bias = bias - border_coordinate_y as f64;

        // slope * x = y - b
        // (y - b) / slope = x
        let crossing_x = (line_start_y as f64 - bias) / slope;

        let min_x = line_start_x.min(line_end_x) as f64;
        let max_x = line_start_x.max(line_end_x) as f64;

        if crossing_x < min_x || crossing_x > max_x { // line does not cross between x1 and x2
            return false
        } else {
            // line does cross but does the line go through the y coordinates
            let min_y = border_coordinate_y.min(center_y);
            let max_y = border_coordinate_y.max(center_y);

            let crossing_y = line_start_y;

            return crossing_y > min_y && crossing_y < max_y;
        }

    } else {
        // vertical line
        assert_eq!(line_start_x, line_end_x);

        if center_x == line_start_x {
            return true
        }

        if line_start_x == border_coordinate_x {
            return false
        }


        let slope = (center_y - border_coordinate_y) as f64 / (center_x - border_coordinate_x) as f64;

        // y = slope * x + b
        // b = y - (slope * x)
        let bias = border_coordinate_y as f64 - (slope * border_coordinate_x as f64);

        // y = slope * x + b
        let crossing_y = slope * line_start_x as f64 + bias;

        let min_y = line_start_y.min(line_end_y) as f64;
        let max_y = line_start_y.max(line_end_y) as f64;

        if crossing_y < min_y || crossing_y > max_y { // line does not cross between y1 and y2
            return false
        } else {
            // line does cross but does the line go through the x coordinates
            let min_x = border_coordinate_x.min(center_x);
            let max_x = border_coordinate_x.max(center_x);

            let crossing_x = line_start_x;

            return crossing_x > min_x && crossing_x < max_x;
        }
    }



    false
}

fn on_segment_exclusive(x1: i64, y1: i64, x2: i64, y2: i64, x3: i64, y3: i64) -> bool {
    // Check of punt (x3, y3) op segment (x1,y1)-(x2,y2) ligt, MAAR niet op eindpunten
    if (x3 == x1 && y3 == y1) || (x3 == x2 && y3 == y2) {
        return false;  // Op eindpunt = toegestaan
    }

    // Check of binnen segment
    x3 >= x1.min(x2) && x3 <= x1.max(x2) &&
        y3 >= y1.min(y2) && y3 <= y1.max(y2)
}

fn direction(x1: i64, y1: i64, x2: i64, y2: i64, x3: i64, y3: i64) -> i64 {
    (x3 - x1) * (y2 - y1) - (y3 - y1) * (x2 - x1)
}