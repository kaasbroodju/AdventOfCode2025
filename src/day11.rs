use crate::Day;
use std::collections::{HashMap, HashSet};

pub struct Day11;

const ASCII_OFFSET: u8 = 'a' as u8;
const YOU_ID: u16 = str_to_id("you");
const OUT_ID: u16 = str_to_id("out");
const FFT_ID: u16 = str_to_id("fft");
const DAC_ID: u16 = str_to_id("dac");
const SVR_ID: u16 = str_to_id("svr");

const fn str_to_id(a: &str) -> u16 {
    let bytes = a.as_bytes();
    let mut result = 0u16;
    let mut i = 0;

    while i < bytes.len() {
        let b = bytes[i];
        result |= ((b - ASCII_OFFSET) as u16) << (i * 5);
        i += 1;
    }

    result
}

impl Day<Graph, usize> for Day11 {
    fn parse_input(&self, input: &str) -> Graph {
        let mut result = HashMap::new();
        input
            .lines()
            .map(|line| line.split_once(": ").unwrap())
            .for_each(|(key, val)| {
                result
                    .insert(
                        str_to_id(key),
                        val
                            .trim()
                            .split(' ')
                            .map(str_to_id)
                            .collect::<Vec<u16>>()
                    );
            });
        Graph::from_hashmap(&result)

    }
    
    fn part1(&self, input: &Graph) -> usize {
        let you_idx = input.get_idx(YOU_ID).unwrap();
        solve_recurse_graph(&input, you_idx)
    }
    
    fn part2(&self, input: &Graph) -> usize {
        let svr_idx = input.get_idx(SVR_ID).unwrap();
        solve_recurse_graph_part_2(&input, &mut Cache::new(input.len()), false, false, svr_idx)
    }
}

pub struct Graph {
    nodes: Vec<GraphNode>,
    id_to_idx: HashMap<u16, usize>, // Alleen voor initialisatie
}

pub struct GraphNode {
    id: u16,
    edges: Vec<usize>, // indices naar Graph.nodes
}

impl Graph {
    pub fn from_hashmap(map: &HashMap<u16, Vec<u16>>) -> Self {
        // Stap 1: Verzamel ALLE unieke node IDs (sources + destinations)
        let mut all_ids = HashSet::new();

        // Voeg alle keys toe (source nodes)
        for &id in map.keys() {
            all_ids.insert(id);
        }

        // Voeg alle edge destinations toe
        for edges in map.values() {
            for &edge_id in edges {
                all_ids.insert(edge_id);
            }
        }

        // Converteer naar sorted Vec voor deterministische indices
        let ids: Vec<u16> = all_ids.into_iter().collect();

        // Stap 2: Maak id -> index mapping
        let id_to_idx: HashMap<u16, usize> = ids.iter()
            .enumerate()
            .map(|(idx, &id)| (id, idx))
            .collect();

        // Stap 3: Bouw nodes met edge indices
        let nodes = ids.iter().map(|&id| {
            let edges = match map.get(&id) {
                Some(edge_ids) => edge_ids.iter()
                    .filter_map(|&edge_id| id_to_idx.get(&edge_id).copied())
                    .collect(),
                None => Vec::new()  // ← Leaf node zonder uitgaande edges
            };

            GraphNode { id, edges }
        }).collect();

        Graph { nodes, id_to_idx }
    }

    pub fn get_idx(&self, id: u16) -> Option<usize> {
        self.id_to_idx.get(&id).copied()
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

fn solve_recurse_graph(
    graph: &Graph,
    current_idx: usize
) -> usize {
    if graph.nodes[current_idx].id == OUT_ID {
        1
    } else {
        graph.nodes[current_idx]
            .edges
            .iter()
            .map(|&next_idx| solve_recurse_graph(graph, next_idx))
            .sum()
    }
}

struct Cache {
    negatives: Vec<Option<usize>>,
    true_fft: Vec<Option<usize>>,
    true_dac: Vec<Option<usize>>,
    positives: Vec<Option<usize>>,
}

impl Cache {
    fn new(capacity: usize) -> Self {
        Self {
            negatives: vec![None; capacity],
            true_fft: vec![None; capacity],
            true_dac: vec![None; capacity],
            positives: vec![None; capacity],
        }
    }

    fn _get_cache_line(&self, seen_fft: bool, seen_dac: bool) -> &[Option<usize>] {
        match (seen_fft, seen_dac) {
            (false, false) => self.negatives.as_slice(),
            (true, false) => self.true_fft.as_slice(),
            (false, true) => self.true_dac.as_slice(),
            (true, true) => self.positives.as_slice(),
        }
    }

    fn _get_cache_line_mut(&mut self, seen_fft: bool, seen_dac: bool) -> &mut [Option<usize>] {
        match (seen_fft, seen_dac) {
            (false, false) => self.negatives.as_mut_slice(),
            (true, false) => self.true_fft.as_mut_slice(),
            (false, true) => self.true_dac.as_mut_slice(),
            (true, true) => self.positives.as_mut_slice(),
        }
    }

    fn get(&self, seen_fft: bool, seen_dac: bool, idx: usize) -> Option<usize> {
        self._get_cache_line(seen_fft, seen_dac)[idx]
    }

    fn set(&mut self, seen_fft: bool, seen_dac: bool, idx: usize, value: usize) {
        self._get_cache_line_mut(seen_fft, seen_dac)[idx] = Some(value);
    }
}

fn solve_recurse_graph_part_2(
    graph: &Graph,
    cache: &mut Cache,
    seen_fft: bool,
    seen_dac: bool,
    idx: usize,
) -> usize {
    let value = graph.nodes[idx].id;

    // Eerst checken: zijn we bij OUT?
    if value == OUT_ID {
        return (seen_fft && seen_dac) as usize;
    }

    if let Some(cached) = cache.get(seen_fft, seen_dac, idx) {
        return cached;
    }

    let result: usize = graph.nodes[idx]
        .edges
        .iter()
        .map(|&next| {
            solve_recurse_graph_part_2(
                graph,
                cache,
                seen_fft || (value == FFT_ID),
                seen_dac || (value == DAC_ID),
                next,
            )
        })
        .sum();

    cache.set(seen_fft, seen_dac, idx, result);

    result
}