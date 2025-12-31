use crate::Day;

pub struct Day12;

const AMOUNT_OF_SHAPES: usize = 6;

pub struct Grid {
    width: usize,
    height: usize,
    amount_of_shapes: [usize; AMOUNT_OF_SHAPES],
}

type ParsingOutput = ([usize; AMOUNT_OF_SHAPES], Vec<Grid>);

impl Day<ParsingOutput, usize> for Day12 {
    fn parse_input(&self, input: &str) -> ParsingOutput {
        let mut shape_areas = [0usize; AMOUNT_OF_SHAPES];
        let mut lines = input.lines();
        for i in 0..AMOUNT_OF_SHAPES {
            let _ = lines.next();
            let line = lines.next().unwrap().as_bytes();
            shape_areas[i] += (line[0] == b'#') as usize + (line[1] == b'#') as usize + (line[2] == b'#') as usize;
            let line = lines.next().unwrap().as_bytes();
            shape_areas[i] += (line[0] == b'#') as usize + (line[1] == b'#') as usize + (line[2] == b'#') as usize;
            let line = lines.next().unwrap().as_bytes();
            shape_areas[i] += (line[0] == b'#') as usize + (line[1] == b'#') as usize + (line[2] == b'#') as usize;
            let _ = lines.next();
        }

        let mut grids = Vec::with_capacity(1000);
        while let Some(line) = lines.next() {
            let left_side = line[0..2].parse::<usize>().unwrap();
            let right_side = line[3..5].parse::<usize>().unwrap();

            let count_per_shape = [
                line[7..9].parse::<usize>().unwrap(),
                line[10..12].parse::<usize>().unwrap(),
                line[13..15].parse::<usize>().unwrap(),
                line[16..18].parse::<usize>().unwrap(),
                line[19..21].parse::<usize>().unwrap(),
                line[22..24].parse::<usize>().unwrap(),
            ];

            grids.push(Grid {
                width: left_side,
                height: right_side,
                amount_of_shapes: count_per_shape,
            });
        }

        (shape_areas, grids)
    }

    fn part1(&self, (shapes_area, grids): &ParsingOutput) -> usize {
        grids
            .iter()
            .map(|grid| {
                let total_area_shapes: usize = grid
                    .amount_of_shapes
                    .iter()
                    .zip(shapes_area)
                    .map(|(a, b)| a * b)
                    .sum();

                if grid.width * grid.height < total_area_shapes {
                    return 0;
                } else if (grid.width / 3) * (grid.height / 3) >= grid.amount_of_shapes.iter().sum() {
                    return 1;
                } else {
                    panic!()
                }
            })
            .sum::<usize>()
    }
    
    fn part2(&self, _input: &ParsingOutput) -> usize {
        "ggwp"
            .bytes()
            .fold(0usize, |acc, value| (acc << 8) | value as usize)
    }
}