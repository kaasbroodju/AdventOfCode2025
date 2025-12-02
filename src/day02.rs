use crate::Day;

pub struct Day02;

impl Day<Vec<(usize, usize)>, usize> for Day02 {
    fn parse_input(&self, input: &str) -> Vec<(usize, usize)> {
        input
            .split(",")
            .map(|line| line.split_once("-").unwrap())
            .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
            .collect::<Vec<_>>()
    }
    
    fn part1(&self, input: &Vec<(usize, usize)>) -> usize {
        let mut total = 0;

        for &(a, b) in input {
            for n in a..=b {

                // n = "2121"
                let encoded_number = encode_digits(n);
                // encoded_number = 0010 0001 0010 0001

                // Tel hoeveel cijfers n heeft
                let digit_count = (n.ilog10() + 1) as usize;

                // hoeveel bits neem ik mee voor de vergelijking?
                let half = digit_count / 2;
                let half_bits = half * 4;
                let mask = (1u64 << half_bits) - 1;
                // mask = 0000 0000 1111 1111

                // dmv de mask neem ik de rechter helft alleen mee.
                let right_half = encoded_number & mask;
                // right_half = 0010 0001

                // verplaats de linker helft naar de rechterkant en herhaal de vergelijking.
                let left_half = (encoded_number >> half_bits) & mask;

                // branchless programming: als het aantal cijfers even is en de linker en rechter helft overeenkomen, dan mag i opgeteld worden.
                total += (digit_count % 2 == 0 && left_half == right_half) as usize * n;
            }
        }

        total
    }
    
    fn part2(&self, input: &Vec<(usize, usize)>) -> usize {
        let mut total = 0;

        for &(a, b) in input {
            for i in a..=b {
                let s = i.to_string();
                let upperbound = (i.ilog10() + 1) as usize;

                for &cursor_size in divisors_of(upperbound) {
                    let pattern = &s[0..cursor_size];

                    let is_repeating = (cursor_size..upperbound)
                        .step_by(cursor_size)
                        .all(|pos| pattern == &s[pos..pos + cursor_size]);

                    if is_repeating {
                        total += i;
                        break;
                    }
                }
            }
        }

        total
    }
}

fn encode_digits(mut n: usize) -> u64 {
    let mut result = 0u64;
    let mut shift = 0;

    // Extract digits van rechts naar links
    while n > 0 {
        let digit = (n % 10) as u64;
        result |= digit << shift;
        shift += 4;
        n /= 10;
    }

    result
}

fn divisors(n: usize) -> Vec<usize> {
    if n == 0 { return vec![]; }
    if n == 1 { return vec![1]; }

    let mut divs = Vec::new();
    let sqrt_n = (n as f64).sqrt() as usize;

    for i in 1..=sqrt_n {
        if n % i == 0 {
            divs.push(i);

            // Voeg ook de "partner" deler toe
            let partner = n / i;
            if partner != i {  // Vermijd duplicaten bij perfecte kwadraten
                divs.push(partner);
            }
        }
    }

    divs.sort_unstable();
    divs
}

const fn divisors_of(n: usize) -> &'static [usize] {
    match n {
        1 => &[],
        2 => &[1],
        3 => &[1],
        4 => &[1, 2],
        5 => &[1],
        6 => &[1, 2, 3],
        7 => &[1],
        8 => &[1, 2, 4],
        9 => &[1, 3],
        10 => &[1, 2, 5],
        11 => &[1],
        12 => &[1, 2, 3, 4, 6],
        _ => &[],
    }
}