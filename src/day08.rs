use crate::Day;

pub struct Day08;


#[derive(Debug)]
pub struct JunctionBox {
    x: usize,
    y: usize,
    z: usize,
}

impl JunctionBox {
    fn get_euclidian_distance(&self, other: &JunctionBox) -> usize {
        (((self.x.abs_diff(other.x)).pow(2) + (self.y.abs_diff(other.y)).pow(2) + (self.z.abs_diff(other.z)).pow(2))).isqrt()
    }
}

#[derive(Debug)]
pub struct Circuit {
    values: Vec<usize>
}

impl Circuit {
    fn new(a: usize, b: usize) -> Self {
        Self { values: vec![a, b] }
    }

    fn is_inside_circuit(&self, a: &usize, b: &usize) -> bool {
        self.values.contains(a) && self.values.contains(b)
    }

    fn could_be_connected(&self, a: &usize, b: &usize) -> bool {
        assert!(!(self.values.contains(a) && self.values.contains(b)));
        self.values.contains(a) || self.values.contains(b)
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
        let max_connections = 1000usize;
        let mut circuits = vec![];
        let mut distances = vec![];
        let length = input.len();

        println!("Creating");
        for i in 0..length {
            for j in i+1..length {
                if i == j { continue }
                let a = &input[i];
                let b = &input[j];
                let distance = a.get_euclidian_distance(b);
                // match distances.binary_search_by(|(a, _, _): &(usize, _, _)| {
                //     a.cmp(&distance)
                // }) {
                //     Ok(index) => {
                //         distances.insert(index, (distance, i , j));
                //     }
                //     Err(index) => {
                //         distances.insert(index, (distance, i , j));
                //     }
                // }
                distances.push((distance, i, j));
            }
        }

        println!("sorting");
        distances.sort_by(|(a, _, _), (b, _, _)| a.cmp(b));

        let mut i = 0;
        let mut connections = 0;

        // circuits = (0..20).map(|i| Circuit { values: vec![i] }).collect::<Vec<_>>();
        

        println!("merging");
        while i < max_connections {
            let (_, ai, bi) = distances[i];
            i += 1;
            
  

            let ai_found_in_circuit_i = circuits.iter().position(|circuit: &Circuit| circuit.values.contains(&ai));
            let bi_found_in_circuit_i = circuits.iter().position(|circuit: &Circuit| circuit.values.contains(&bi));

            match (ai_found_in_circuit_i, bi_found_in_circuit_i) {
                (Some(a_index), Some(b_index)) if a_index == b_index => {}
                (Some(a_index), Some(b_index)) if a_index != b_index => {
                    // two circuits can be joined
                    let remove_index = a_index.max(b_index);
                    let join_index = a_index.min(b_index);

                    let mut values = circuits.remove(remove_index).values;
                    circuits.get_mut(join_index).unwrap().values.append(&mut values);
                    connections += 1;
                }
                (Some(a_index), None) => {
                    circuits.get_mut(a_index).unwrap().values.push(bi);
                    connections += 1;
                }
                (None, Some(b_index)) => {
                    circuits.get_mut(b_index).unwrap().values.push(ai);
                    connections += 1;
                }
                (None, None) => {
                    circuits.push(Circuit::new(ai, bi));
                    connections += 1;
                },
                (Some(_), Some(_)) => panic!()
            }
        }




        circuits.sort_by(|a, b| b.values.len().cmp(&a.values.len()));
        // println!("{:?}", circuits);
        // println!("{:?}", circuits[0].values.len() * circuits[1].values.len() * circuits[2].values.len());
        // TODO: Implement part 1 solution
        // 5814 too low
        // 102816
        (circuits[0].values.len() * circuits[1].values.len() * circuits[2].values.len()) as i32 
    }
    
    fn part2(&self, input: &Vec<JunctionBox>) -> i32 {
        let max_connections = 1000usize;
        let mut circuits = vec![];
        let mut distances = vec![];
        let length = input.len();

        println!("Creating");
        for i in 0..length {
            for j in i+1..length {
                if i == j { continue }
                let a = &input[i];
                let b = &input[j];
                let distance = a.get_euclidian_distance(b);
                distances.push((distance, i, j));
            }
        }

        println!("sorting");
        distances.sort_by(|(a, _, _), (b, _, _)| a.cmp(b));

        let mut i = 0;
        let mut connections = 0;

        circuits = (0..20).map(|i| Circuit { values: vec![i] }).collect::<Vec<_>>();
        let max_connections = distances.len();

        println!("merging");
        while i < max_connections {
            let (_, ai, bi) = distances[i];
            i += 1;



            let ai_found_in_circuit_i = circuits.iter().position(|circuit: &Circuit| circuit.values.contains(&ai));
            let bi_found_in_circuit_i = circuits.iter().position(|circuit: &Circuit| circuit.values.contains(&bi));

            match (ai_found_in_circuit_i, bi_found_in_circuit_i) {
                (Some(a_index), Some(b_index)) if a_index == b_index => {}
                (Some(a_index), Some(b_index)) if a_index != b_index => {
                    // two circuits can be joined
                    let remove_index = a_index.max(b_index);
                    let join_index = a_index.min(b_index);

                    let mut values = circuits.remove(remove_index).values;
                    circuits.get_mut(join_index).unwrap().values.append(&mut values);
                    connections += 1;
                }
                (Some(a_index), None) => {
                    circuits.get_mut(a_index).unwrap().values.push(bi);
                    connections += 1;
                }
                (None, Some(b_index)) => {
                    circuits.get_mut(b_index).unwrap().values.push(ai);
                    connections += 1;
                }
                (None, None) => {
                    circuits.push(Circuit::new(ai, bi));
                    connections += 1;
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
