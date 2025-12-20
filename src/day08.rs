use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use rayon::prelude::ParallelSliceMut;
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
    fn calculate_distance(&self, other: &JunctionBox) -> usize {
        self.x.abs_diff(other.x).pow(2) +
        self.y.abs_diff(other.y).pow(2) +
        self.z.abs_diff(other.z).pow(2)
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

    fn len(&self) -> usize {
        self.values.iter().fold(0, |acc, a| acc + a.count_ones()) as usize
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

impl Day<Vec<JunctionBox>, usize> for Day08 {
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
    
    fn part1(&self, input: &Vec<JunctionBox>) -> usize {
        
        let mut circuits = vec![];
        let length = input.len();


        // Stap 1: Verzamel top 1000 met een min-heap van max 1000 items
        let mut heap: BinaryHeap<Path> = BinaryHeap::with_capacity(ITERATIONS + 1);


        for i in 0..length {
            for j in i+1..length {
                let distance = input[i].calculate_distance(&input[j]);
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

        for path in top_1000 {
            let ai = path.from;
            let bi = path.to;
            let ai_found_in_circuit_i = circuits.iter().position(|circuit: &Circuit| circuit.contains(&ai));
            let bi_found_in_circuit_i = circuits.iter().position(|circuit: &Circuit| circuit.contains(&bi));

            match (ai_found_in_circuit_i, bi_found_in_circuit_i) {
                (Some(a_index), Some(b_index)) => {
                    if a_index == b_index { continue; }
                    
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
                }
            }
        }

        circuits.sort_by(|a, b| b.len().cmp(&a.len()));

        circuits[0].len() * circuits[1].len() * circuits[2].len()
    }
    
    fn part2(&self, input: &Vec<JunctionBox>) -> usize {
        
        let length = input.len();
        let mut distances = Vec::with_capacity((length * (length - 1)) / 2);

        for i in 0..length {
            for j in i+1..length {
                let sq_distance = input[i].calculate_distance(&input[j]);
                distances.push((sq_distance, i, j));
            }
        }

        radix_sort_distances(&mut distances);

        // Union-Find
        let mut uf = UnionFind::new(length);
        let mut components = length;

        for (_, ai, bi) in distances {
            if uf.union(ai, bi) {
                components -= 1;
                if components == 1 {
                    return input[ai].x * input[bi].x;
                }
            }
        }
        0
    }
    
    
}

struct UnionFind {
    parent: Vec<usize>,  // parent[i] = parent van node i
    rank: Vec<usize>,    // rank[i] = diepte van tree onder i
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),  // iedereen is eigen parent
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // Path compression
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false; // Al in dezelfde set
        }

        // Union by rank
        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }
        true
    }
}

fn radix_sort_distances(distances: &mut Vec<(usize, usize, usize)>) {
    if distances.is_empty() { return; }

    // Vind max voor aantal bits
    let max_dist = distances.iter().map(|&(d, _, _)| d).max().unwrap();

    // Aantal bits nodig
    let max_bits = 64 - max_dist.leading_zeros();

    // Sorteer per bit (of per 8 bits voor bytes)
    let mut temp = vec![(0, 0, 0); distances.len()];

    // Process per byte (8 bits tegelijk)
    for byte_pos in 0..((max_bits + 7) / 8) {
        let shift = byte_pos * 8;
        let mut counts = [0usize; 256];

        // Count occurrences
        for &(dist, _, _) in distances.iter() {
            let byte = ((dist >> shift) & 0xFF) as usize;
            counts[byte] += 1;
        }

        // Prefix sum voor positions
        let mut pos = 0;
        for count in counts.iter_mut() {
            let tmp = *count;
            *count = pos;
            pos += tmp;
        }

        // Place elements in sorted position
        for &(dist, from, to) in distances.iter() {
            let byte = ((dist >> shift) & 0xFF) as usize;
            temp[counts[byte]] = (dist, from, to);
            counts[byte] += 1;
        }

        distances.copy_from_slice(&temp);
    }
}