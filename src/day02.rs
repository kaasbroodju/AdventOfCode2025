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
            for n in a..=b {
                // n = "2121"
                let encoded = encode_digits(n);
                // encoded = 0010 0001 0010 0001

                // Tel hoeveel cijfers n heeft
                let digit_count = (n.ilog10() + 1) as usize;

                for &pattern_size in divisors_of(digit_count) {
                    // pattern_size = 2

                    // Stap 1: Extract pattern (eerste 2 cijfers)
                    let pattern_bits = pattern_size * 4;
                    let pattern_mask = (1u64 << pattern_bits) - 1;
                    // pattern_mask = 0000 0000 1111 1111

                    let pattern = encoded & pattern_mask;
                    // 0000 0000 1111 1111
                    // 0010 0001 0010 0001
                    // 0000 0000 0010 0001 &


                    // Stap 2: Herhaal pattern over hele lengte
                    let mut replicated = 0u64;
                    let total_bits = digit_count * 4;

                    for shift in (0..total_bits).step_by(pattern_bits) {
                        replicated |= pattern << shift;
                    }
                    // replicated = 0010 0001 herhaalt

                    // Stap 3: Vergelijk met origineel
                    let full_mask = (1u64 << total_bits) - 1;
                    // full_mask = 1111 1111 1111 1111
                    let result = (encoded & full_mask) == (replicated & full_mask);

                    if result {
                        total += n;
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

fn _divisors(n: usize) -> Vec<usize> {
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
        13 => &[1],
        14 => &[1, 2, 7],
        15 => &[1, 3, 5],
        16 => &[1, 2, 4, 8],
        _ => &[],
    }
}