use crate::Day;

pub struct Day12;

pub struct Grid {
    width: usize,
    height: usize,
    amount_of_shapes: Vec<usize>,
}

type ParsingOutput = (Vec<[u64; 3]>, Vec<Grid>);

impl Day<ParsingOutput, usize> for Day12 {
    fn parse_input(&self, input: &str) -> ParsingOutput {
        let lines = input.lines().collect::<Vec<&str>>();
        let index_first_grid = lines.iter().position(|line| line.contains('x')).unwrap();
        let (shape_lines, grid_lines) = lines.split_at(index_first_grid);
        let shapes = shape_lines
            .chunks(5)
            .map(|lines| {
                [
                    lines[1].as_bytes()
                        .iter()
                        .fold(0u64, |acc, &c| (acc << 1) | ((c == b'#') as u64)),
                    lines[2].as_bytes()
                        .iter()
                        .fold(0u64, |acc, &c| (acc << 1) | ((c == b'#') as u64)),
                    lines[3].as_bytes()
                        .iter()
                        .fold(0u64, |acc, &c| (acc << 1) | ((c == b'#') as u64)),
                    // [lines[1].as_bytes()[0] == b'#', lines[1].as_bytes()[1] == b'#', lines[1].as_bytes()[2] == b'#'],
                    // [lines[2].as_bytes()[0] == b'#', lines[2].as_bytes()[1] == b'#', lines[2].as_bytes()[2] == b'#'],
                    // [lines[3].as_bytes()[0] == b'#', lines[3].as_bytes()[1] == b'#', lines[3].as_bytes()[2] == b'#'],
                ]
            })
            .collect::<Vec<_>>();

        let grids = grid_lines
            .iter()
            .map(|line| {
                let (grid_size_str, amount_of_shapes_str) = line.split_once(": ").unwrap();
                let (width, height) = grid_size_str.split_once("x").unwrap();
                let width = width.parse::<usize>().unwrap();
                let height = height.parse::<usize>().unwrap();
                assert!(width <= usize::BITS as usize);

                let amount_of_shapes = amount_of_shapes_str
                    .split(" ")
                    .map(|shapes_str| shapes_str.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                    // .as_slice();

                Grid {
                    width,
                    height,
                    amount_of_shapes,
                }
            })
            .collect::<Vec<_>>();
        // println!("{:?}", shape_lines.chunks(4));
        // println!("{:?}", (shape_lines, grid_lines));

        (shapes, grids)
    }
    
    fn part1(&self, input: &ParsingOutput) -> usize {

        input
            .1
            .iter()
            .map(|grid| {
                let width = grid.width;
                let height = grid.height;
                let shapes_stack = grid.amount_of_shapes
                    .iter()
                    .enumerate()
                    .map(|(shape_id, amount)| vec![shape_id; *amount])
                    .flatten()
                    .collect::<Vec<_>>();
                let mut grid = vec![!(u64::MAX << (u64::BITS as usize - width)); height];
                for x in &grid {
                    println!("{:064b}", x);
                }
                let cursor_x = 0;
                let cursor_y = 0;

                let max_height = height - 2; // -2 omdat shape 3 hoog is
                let max_width = width - 2;

                bfs_prob(&input.0, max_width, max_height, &mut grid, &shapes_stack, 0) as usize
            })
            .sum::<usize>()
    }
    
    fn part2(&self, _input: &ParsingOutput) -> usize {
        // TODO: Implement part 2 solution
        0
    }
}

fn bfs_prob(
    shapes: &Vec<[u64; 3]>,
    max_width: usize,
    max_height: usize,
    grid: &mut Vec<u64>,
    shapes_needed: &Vec<usize>,
    shape_index: usize
) -> bool {
    // Early return: alle shapes geplaatst!
    if shape_index >= shapes_needed.len() {
        return true;
    }

    let current_shape_id = shapes_needed[shape_index];
    let shape = shapes[current_shape_id];

    // let max_height = grid.len() - 2; // -2 omdat shape 3 hoog is
    // let max_width = u64::BITS as usize - grid[0].leading_ones() as usize - 2;

    // println!("{max_width}\t{max_height}");

    // Probeer alle posities
    for y in 0..max_height {
        for x in 0..max_width {
            // Probeer alle rotaties/flips van deze shape
            for rotated_shape in get_all_orientations(&shape) {
                if can_place(grid, &rotated_shape, x, y) {
                    // Plaats de shape
                    place_shape(grid, &rotated_shape, x, y);

                    // Recursief verder met volgende shape
                    if bfs_prob(shapes, max_width, max_height, grid, shapes_needed, shape_index + 1) {
                        return true; // Gevonden!
                    }

                    // Backtrack: verwijder shape weer
                    remove_shape(grid, &rotated_shape, x, y);
                }
            }
        }
    }

    false // Geen oplossing gevonden
}

fn can_place(grid: &Vec<u64>, shape: &[u64; 3], x: usize, y: usize) -> bool {
    let mask_offset = u64::BITS as usize - 3 - x;
    (grid[y] & (shape[0] << mask_offset)) == 0
        && (grid[y + 1] & (shape[1] << mask_offset)) == 0
        && (grid[y + 2] & (shape[2] << mask_offset)) == 0
}

fn place_shape(grid: &mut Vec<u64>, shape: &[u64; 3], x: usize, y: usize) {
    let mask_offset = u64::BITS as usize - 3 - x;
    grid[y] |= shape[0] << mask_offset;
    grid[y + 1] |= shape[1] << mask_offset;
    grid[y + 2] |= shape[2] << mask_offset;
}

fn remove_shape(grid: &mut Vec<u64>, shape: &[u64; 3], x: usize, y: usize) {
    let mask_offset = u64::BITS as usize - 3 - x;
    grid[y] &= !(shape[0] << mask_offset);
    grid[y + 1] &= !(shape[1] << mask_offset);
    grid[y + 2] &= !(shape[2] << mask_offset);
}

fn rotate_90(shape: &[u64; 3]) -> [u64; 3] {
    // Van links-naar-rechts wordt boven-naar-beneden
    let mut result = [0u64; 3];

    for col in 0..3 {
        for row in 0..3 {
            let bit = (shape[row] >> (2 - col)) & 1;
            result[col] |= bit << (2 - row);
        }
    }

    result
}

fn rotate_180(shape: &[u64; 3]) -> [u64; 3] {
    [
        reverse_3bits(shape[2]),
        reverse_3bits(shape[1]),
        reverse_3bits(shape[0]),
    ]
}

fn rotate_270(shape: &[u64; 3]) -> [u64; 3] {
    // 3x 90° = 270°, of gewoon inverse van 90°
    let mut result = [0u64; 3];

    for col in 0..3 {
        for row in 0..3 {
            let bit = (shape[row] >> (2 - col)) & 1;
            result[2 - col] |= bit << row;
        }
    }

    result
}

fn flip_horizontal(shape: &[u64; 3]) -> [u64; 3] {
    // Spiegel elke rij
    [
        reverse_3bits(shape[0]),
        reverse_3bits(shape[1]),
        reverse_3bits(shape[2]),
    ]
}

fn flip_vertical(shape: &[u64; 3]) -> [u64; 3] {
    // Rijen omdraaien
    [shape[2], shape[1], shape[0]]
}

fn reverse_3bits(bits: u64) -> u64 {
    // Draai 3 bits om: 101 -> 101, 110 -> 011, etc.
    ((bits & 0b100) >> 2) | (bits & 0b010) | ((bits & 0b001) << 2)
}

fn get_all_orientations(shape: &[u64; 3]) -> Vec<[u64; 3]> {
    let mut orientations = Vec::new();
    let mut seen = std::collections::HashSet::new();

    // Alle transformaties
    let transforms = [
        *shape,                          // 0°
        rotate_90(shape),                // 90°
        rotate_180(shape),               // 180°
        rotate_270(shape),               // 270°
        flip_horizontal(shape),          // horizontaal gespiegeld
        flip_vertical(shape),            // verticaal gespiegeld
        rotate_90(&flip_horizontal(shape)),  // gespiegeld + 90°
        rotate_90(&flip_vertical(shape)),    // gespiegeld + 90°
    ];

    // Alleen unieke orientaties toevoegen
    for t in transforms {
        if seen.insert(t) {
            orientations.push(t);
        }
    }

    orientations
}
