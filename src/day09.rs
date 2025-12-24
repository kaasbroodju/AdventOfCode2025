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
                    if !has_collision(&lines, (ax, ay), (bx, by)) {
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
        max
    }
}

fn has_collision(
    polygon: &Vec<((usize, usize), (usize, usize))>,
    a: (usize, usize),
    b: (usize, usize)
) -> bool {
    let min_x = a.0.min(b.0);
    let max_x = a.0.max(b.0);
    let min_y = a.1.min(b.1);
    let max_y = a.1.max(b.1);

    let (horizontal, vertical): (Vec<((usize, usize), (usize, usize))>, Vec<((usize, usize), (usize, usize))>) = polygon
        .into_iter()
        .partition(|((_, a), (_, b))| a == b);

    for x in min_x..=max_x { // horizontal sweep, aka vertical lines
        for (from, to) in &horizontal {
            if crosses_horizontal((x, min_y), (x, max_y), *from, *to) {
                return true;
            }
        }
    }

    for y in min_y..=max_y {
        for (from, to) in &vertical {
            if crosses_vertical((min_x, y), (max_x, y), *from, *to) {
                return true;
            }
        }
    }

    // // Stap 1: Bereken het midden
    // let mid_x = (min_x + max_x) / 2;
    // let mid_y = (min_y + max_y) / 2;
    // let center = (mid_x, mid_y);
    //
    // // Stap 2 & 3: Trek lijnen van midden naar 4 hoeken en check kruisingen
    // let corners = [
    //     (min_x, min_y),  // top-left
    //     (max_x, min_y),  // top-right
    //     (min_x, max_y),  // bottom-left
    //     (max_x, max_y),  // bottom-right
    // ];
    //
    // for &corner in &corners {
    //     if line_crosses_polygon(&polygon, &center, &corner) {
    //         return false;
    //     }
    // }
    //
    // // Stap 4: Ga de rand af en check lijnen naar midden
    // // Top rand
    // for x in min_x+1..max_x {
    //     if line_crosses_polygon(polygon, &center, &(x, min_y)) {
    //         return false;
    //     }
    // }
    //
    // // Bottom rand
    // for x in min_x+1..max_x {
    //     if line_crosses_polygon(polygon, &center, &(x, max_y)) {
    //         return false;
    //     }
    // }
    //
    // // Left rand
    // for y in min_y+1..max_y {
    //     if line_crosses_polygon(polygon, &center, &(min_x, y)) {
    //         return false;
    //     }
    // }
    //
    // // Right rand
    // for y in min_y+1..max_y {
    //     if line_crosses_polygon(polygon, &center, &(max_x, y)) {
    //         return false;
    //     }
    // }

    false
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


fn segments_intersect(
    from: (usize, usize),
    to: (usize, usize),
    from_polyglot: (usize, usize),
    to_polyglot: (usize, usize)
) -> bool {
    let (ax1, ay1) = (from.0 as i64, from.1 as i64);
    let (ax2, ay2) = (to.0 as i64, to.1 as i64);
    let (bx1, by1) = (from_polyglot.0 as i64, from_polyglot.1 as i64);
    let (bx2, by2) = (to_polyglot.0 as i64, to_polyglot.1 as i64);

    let d1 = direction(bx1, by1, bx2, by2, ax1, ay1);
    let d2 = direction(bx1, by1, bx2, by2, ax2, ay2);
    let d3 = direction(ax1, ay1, ax2, ay2, bx1, by1);
    let d4 = direction(ax1, ay1, ax2, ay2, bx2, by2);

    // Strikte kruising: beide segmenten moeten elkaar doorkruisen
    if ((d1 > 0 && d2 < 0) || (d1 < 0 && d2 > 0)) &&
        ((d3 > 0 && d4 < 0) || (d3 < 0 && d4 > 0)) {
        return true;
    }

    // Collinear cases: check of segmenten overlappen
    if d1 == 0 && on_segment(bx1, by1, bx2, by2, ax1, ay1) { return true; }
    if d2 == 0 && on_segment(bx1, by1, bx2, by2, ax2, ay2) { return true; }
    if d3 == 0 && on_segment(ax1, ay1, ax2, ay2, bx1, by1) { return true; }
    if d4 == 0 && on_segment(ax1, ay1, ax2, ay2, bx2, by2) { return true; }

    false
}

fn on_segment(x1: i64, y1: i64, x2: i64, y2: i64, x3: i64, y3: i64) -> bool {
    x3 >= x1.min(x2) && x3 <= x1.max(x2) &&
        y3 >= y1.min(y2) && y3 <= y1.max(y2)
}

fn direction(x1: i64, y1: i64, x2: i64, y2: i64, x3: i64, y3: i64) -> i64 {
    (x3 - x1) * (y2 - y1) - (y3 - y1) * (x2 - x1)
}