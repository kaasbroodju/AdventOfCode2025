use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use crate::Day;

pub struct Day08;

const AMOUNT_OF_BOXES: usize = 1000;
const ITERATIONS: usize = 1000;
const BIT_ARRAY_SIZE: usize = (AMOUNT_OF_BOXES / u64::BITS as usize) + 1;

#[derive(Debug)]
pub struct JunctionBox {
    x: usize,
    y: usize,
    z: usize,
}

impl JunctionBox {

    #[inline]
    fn get_euclidian_distance(&self, other: &JunctionBox) -> usize {
        (self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)).isqrt()
    }
}

#[derive(Debug)]
pub struct Circuit {
    values: Box<[u64; BIT_ARRAY_SIZE]>
}

impl Circuit {
    fn new(a: usize, b: usize) -> Self {
        let mut bit_array = Box::new([0u64; BIT_ARRAY_SIZE]);
        bit_array[a / 64] |= 1 << (a % 64);
        bit_array[b / 64] |= 1 << (b % 64);

        Self { values: bit_array }
    }

    fn new_single(a: usize) -> Self {
        let mut bit_array = Box::new([0u64; BIT_ARRAY_SIZE]);
        bit_array[a / 64] |= 1 << (a % 64);

        Self { values: bit_array }
    }

    // fn is_inside_circuit(&self, a: &usize, b: &usize) -> bool {
    //     self.values.contains(a) && self.values.contains(b)
    // }

    // fn could_be_connected(&self, a: &usize, b: &usize) -> bool {
    //     assert!(!(self.values.contains(a) && self.values.contains(b)));
    //     self.values.contains(a) || self.values.contains(b)
    // }

    fn contains(&self, a: &usize) -> bool {
        self.values[a / 64] & (1 << (a % 64)) != 0
    }

    fn set(&mut self, a: &usize) {
        self.values[a / 64] |= 1 << (a % 64);
    }

    fn merge(&mut self, other: Self) {
        for i in 0..BIT_ARRAY_SIZE {
            self.values[i] |= other.values[i];
        }
    }

    fn len(&self) -> u32 {
        self.values.iter().fold(0, |acc, a| acc + a.count_ones())
    }
}

#[derive(Eq)]
#[derive(PartialOrd)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Path {
    distance: usize,
    from: usize,
    to: usize,
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl Day<Vec<JunctionBox>, i32> for Day08 {
    fn parse_input(&self, input: &str) -> Vec<JunctionBox> {
        input
            .lines()
            .map(|line| line.split(",").collect::<Vec<_>>())
            .map(|cords| {
                JunctionBox {
                    x: cords[0].parse::<usize>().unwrap(),
                    y: cords[1].parse::<usize>().unwrap(),
                    z: cords[2].parse::<usize>().unwrap(),
                }
            })
            .collect::<Vec<_>>()
    }
    
    fn part1(&self, input: &Vec<JunctionBox>) -> i32 {
        let mut circuits = vec![];
        let length = input.len();


        // Stap 1: Verzamel top 1000 met een min-heap van max 1000 items
        let mut heap: BinaryHeap<Path> = BinaryHeap::with_capacity(ITERATIONS + 1);


        for i in 0..length {
            for j in i+1..length {
                let distance = input[i].get_euclidian_distance(&input[j]);
                let path = Path { distance, from: i, to: j };

                // Slimme insertie: alleen toevoegen als beter dan huidige max
                if heap.len() == ITERATIONS {
                    // Heap is vol - check of nieuwe distance beter is
                    if let Some(worst) = heap.peek() {
                        if distance < worst.distance {
                            heap.pop();  // Verwijder slechtste
                            heap.push(path);  // Voeg betere toe
                        }
                        // Anders: skip deze distance, niet goed genoeg
                    }
                } else {
                    // Heap nog niet vol, gewoon toevoegen
                    heap.push(path);
                }
            }
        }

        // Extract en sorteer - dit blijft exact hetzelfde!
        let mut top_1000: Vec<_> = heap.into_sorted_vec();
        // top_1000.sort_unstable();  // Nu alleen 1000 items sorteren!

        for path in top_1000 {
            let ai = path.from;
            let bi = path.to;
            let ai_found_in_circuit_i = circuits.iter().position(|circuit: &Circuit| circuit.contains(&ai));
            let bi_found_in_circuit_i = circuits.iter().position(|circuit: &Circuit| circuit.contains(&bi));

            match (ai_found_in_circuit_i, bi_found_in_circuit_i) {
                (Some(a_index), Some(b_index)) if a_index == b_index => {}
                (Some(a_index), Some(b_index)) if a_index != b_index => {
                    // two circuits can be joined
                    let remove_index = a_index.max(b_index);
                    let join_index = a_index.min(b_index);

                    let values = circuits.remove(remove_index);
                    circuits.get_mut(join_index).unwrap().merge(values);
                }
                (Some(a_index), None) => {
                    circuits.get_mut(a_index).unwrap().set(&bi);
                }
                (None, Some(b_index)) => {
                    circuits.get_mut(b_index).unwrap().set(&ai);
                }
                (None, None) => {
                    circuits.push(Circuit::new(ai, bi));
                    // panic!()
                },
                (Some(_), Some(_)) => panic!()
            }
        }

        circuits.sort_by(|a, b| b.len().cmp(&a.len()));

        (circuits[0].len() * circuits[1].len() * circuits[2].len()) as i32
    }
    
    fn part2(&self, input: &Vec<JunctionBox>) -> i32 {
        // let mut circuits = vec![];
        let mut distances = vec![];
        let length = input.len();

        for i in 0..length {
            for j in i+1..length {
                if i == j { continue }
                let a = &input[i];
                let b = &input[j];
                let distance = a.get_euclidian_distance(b);
                distances.push((distance, i, j));
            }
        }

        distances.sort_by(|(a, _, _), (b, _, _)| a.cmp(b));

        // let mut i = 0;
        // let mut connections = 0;

        let mut circuits = (0..input.len()).map(|i| Circuit::new_single(i)).collect::<Vec<_>>();
        // let max_connections = distances.len();

        for (_, ai, bi) in distances {
            // let (_, ai, bi) = distances[i];
            // i += 1;



            let ai_found_in_circuit_i = circuits.iter().position(|circuit: &Circuit| circuit.contains(&ai));
            let bi_found_in_circuit_i = circuits.iter().position(|circuit: &Circuit| circuit.contains(&bi));

            match (ai_found_in_circuit_i, bi_found_in_circuit_i) {
                (Some(a_index), Some(b_index)) if a_index == b_index => {}
                (Some(a_index), Some(b_index)) if a_index != b_index => {
                    // two circuits can be joined
                    let remove_index = a_index.max(b_index);
                    let join_index = a_index.min(b_index);

                    let values = circuits.remove(remove_index);
                    circuits.get_mut(join_index).unwrap().merge(values);
                    // connections += 1;
                }
                (Some(a_index), None) => {
                    circuits.get_mut(a_index).unwrap().set(&bi);
                    // connections += 1;
                }
                (None, Some(b_index)) => {
                    circuits.get_mut(b_index).unwrap().set(&ai);
                    // connections += 1;
                }
                (None, None) => {
                    circuits.push(Circuit::new(ai, bi));
                    // connections += 1;
                    // panic!()
                },
                (Some(_), Some(_)) => panic!()
            }
            
            if circuits.len() == 1 {
                return (input[ai].x * input[bi].x) as i32;
            }
        }

        return -1;
    }
}
