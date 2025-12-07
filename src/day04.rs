use crate::Day;

pub struct Day04;

impl Day<Vec<Vec<bool>>, i32> for Day04 {
    fn parse_input(&self, input: &str) -> Vec<Vec<bool>>
    {
        input.lines().map(|line| line.chars().map(|c| c == '@').collect::<Vec<_>>()).collect::<Vec<_>>()

    }
    
    fn part1(&self, input: &Vec<Vec<bool>>) -> i32 {

        let mut total = 0;

        let height = input.len();
        let width = input[0].len();

        let mut y = 0;
        while y < height {
            let mut x = 0;

            while x < width {
                if !input[y][x] { x += 1; continue; }

                let mut surrounding_paper_rolls = 0;
                // check upper_layer
                let has_upper_layer = y as isize - 1 >= 0;
                if has_upper_layer && x as isize - 1 >= 0 { surrounding_paper_rolls += input[y - 1][x - 1] as usize } // upper left
                if has_upper_layer { surrounding_paper_rolls += input[y - 1][x] as usize } // upper middle
                if has_upper_layer && x + 1 < width { surrounding_paper_rolls += input[y - 1][x + 1] as usize } // upper right

                // check middle_layer
                if x as isize - 1 >= 0 { surrounding_paper_rolls += input[y][x - 1] as usize } // middle left
                if x + 1 < width { surrounding_paper_rolls += input[y][x + 1] as usize } // middle right

                // check lower_layer
                if y + 1 < height && x as isize - 1 >= 0 { surrounding_paper_rolls += input[y + 1][x - 1] as usize } // down left
                if y + 1 < height { surrounding_paper_rolls += input[y + 1][x] as usize } // down middle
                if y + 1 < height && x + 1 < width { surrounding_paper_rolls += input[y + 1][x + 1] as usize } // down right

                if surrounding_paper_rolls < 4 {
                    total += 1;
                }


                x += 1;
            }

            y += 1;
        }


        total
    }
    
    fn part2(&self, input: &Vec<Vec<bool>>) -> i32 {
        let mut input = input.clone();
        let mut total = 0;

        let height = input.len();
        let width = input[0].len();

        loop {
            let mut total_round = 0;

            let mut y = 0;
            while y < height {
                let mut x = 0;

                while x < width {
                    if !input[y][x] { x += 1; continue; }

                    let mut surrounding_paper_rolls = 0;
                    // check upper_layer
                    let has_upper_layer = y as isize - 1 >= 0;
                    if has_upper_layer && x as isize - 1 >= 0 { surrounding_paper_rolls += input[y - 1][x - 1] as usize } // upper left
                    if has_upper_layer { surrounding_paper_rolls += input[y - 1][x] as usize } // upper middle
                    if has_upper_layer && x + 1 < width { surrounding_paper_rolls += input[y - 1][x + 1] as usize } // upper right

                    // check middle_layer
                    if x as isize - 1 >= 0 { surrounding_paper_rolls += input[y][x - 1] as usize } // middle left
                    if x + 1 < width { surrounding_paper_rolls += input[y][x + 1] as usize } // middle right

                    // check lower_layer
                    if y + 1 < height && x as isize - 1 >= 0 { surrounding_paper_rolls += input[y + 1][x - 1] as usize } // down left
                    if y + 1 < height { surrounding_paper_rolls += input[y + 1][x] as usize } // down middle
                    if y + 1 < height && x + 1 < width { surrounding_paper_rolls += input[y + 1][x + 1] as usize } // down right

                    if surrounding_paper_rolls < 4 {
                        input[y][x] = false; // remove from the grid
                        total_round += 1;
                    }


                    x += 1;
                }

                y += 1;
            }


            total += total_round;
            if total_round == 0 { break; }
        }

        total
    }
}
