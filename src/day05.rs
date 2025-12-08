use std::ops::Deref;
use crate::Day;

#[derive(Debug)]
pub struct Node {
    l_value: usize,
    r_value: usize,


    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}


impl Node {
    fn new(l_value: usize, r_value: usize) -> Self {
        Self {
            l_value,
            r_value,
            left: None,
            right: None,
        }
    }

    fn upsert(&mut self, other: Node) {
        // Case 1: other fits completely inside this node
        if other.l_value >= self.l_value && other.r_value <= self.r_value {
            return;
        }

        // Case 2: other is completely to the left (no overlap)
        if other.r_value < self.l_value {
            if let Some(left_node) = self.left.as_mut() {
                left_node.upsert(other);
            } else {
                self.left = Some(Box::new(other));
            }
            return;
        }

        // Case 3: other is completely to the right (no overlap)
        if other.l_value > self.r_value {
            if let Some(right_node) = self.right.as_mut() {
                right_node.upsert(other);
            } else {
                self.right = Some(Box::new(other));
            }
            return;
        }

        // Case 4: There is overlap - merge the ranges
        let new_l = self.l_value.min(other.l_value);
        let new_r = self.r_value.max(other.r_value);

        self.l_value = new_l;
        self.r_value = new_r;

        // After merging, we need to check if our children now overlap with us
        // and re-insert them to maintain tree invariants

        // Take and re-process left subtree
        if let Some(left) = self.left.take() {
            self.reinsert_subtree(*left);
        }

        // Take and re-process right subtree
        if let Some(right) = self.right.take() {
            self.reinsert_subtree(*right);
        }
    }

    fn reinsert_subtree(&mut self, node: Node) {
        // Recursively collect all nodes from the subtree
        let mut nodes = vec![node];
        let mut i = 0;

        while i < nodes.len() {
            if let Some(left) = nodes[i].left.take() {
                nodes.push(*left);
            }
            if let Some(right) = nodes[i].right.take() {
                nodes.push(*right);
            }
            i += 1;
        }

        // Re-insert all nodes
        for mut node in nodes {
            // Clear the node's children to insert just the range
            node.left = None;
            node.right = None;
            self.upsert(node);
        }
    }

    fn is_valid(&self, id: usize) -> bool {
        if id < self.l_value {
            if let Some(node) = self.left.as_ref() {
                node.is_valid(id)
            } else {
                false
            }
        } else if id > self.r_value {
            if let Some(node) = self.right.as_ref() {
                node.is_valid(id)
            } else {
                false
            }
        } else {
            true
        }
    }

    fn amount_of_valid_ids(&self) -> usize {
        (self.r_value
            - self.l_value + 1)
            + self.left.as_ref().map_or(0, |l| l.amount_of_valid_ids())
            + self.right.as_ref().map_or(0, |r| r.amount_of_valid_ids())
    }
}

pub struct Day05;

impl Day<(Node, Vec<usize>), usize> for Day05 {
    fn parse_input(&self, input: &str) -> (Node, Vec<usize>) {
        let (ranges, values) = input.split_once("\r\n\r\n").unwrap();
        let mut ranges: Vec<_> = ranges
            .lines()
            .map(|line| {
                let (start, end) = line.split_once("-").unwrap();
                Node::new(start.parse::<usize>().unwrap(), end.parse::<usize>().unwrap())

            })
            .collect();

        let mut root_node = ranges.pop().unwrap();
        for node in ranges {
            root_node.upsert(node);
        }

        let values = values
            .lines()
            .map(|line| line.parse::<usize>().unwrap())
            .collect::<Vec<_>>();


        (root_node, values)
        
    }
    
    fn part1(&self, (root_node, input): &(Node, Vec<usize>)) -> usize {
        input.iter().filter(|&&id| root_node.is_valid(id)).count()
    }
    
    fn part2(&self, (root_node, _): &(Node, Vec<usize>)) -> usize {
        root_node.amount_of_valid_ids()
    }
}
