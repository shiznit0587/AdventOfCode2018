use crate::utils;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub fn day7() -> std::io::Result<()> {
    println!("Running Day 7 - a");

    let rex = Regex::new(r"Step (\w) .* step (\w)").unwrap();

    let edges: Vec<(usize, usize)> = utils::readDay(7)?
        .iter()
        .map(|l| rex.captures(l).unwrap())
        .map(|c| (utils::tryParse_char(&c[1]), utils::tryParse_char(&c[2])))
        .map(|(a, b)| ((a - 65) as usize, (b - 65) as usize))
        .collect();

    let mut visited: u32 = 0;
    let mut visitedCount = 0;
    let mut visitOrder: [usize; 26] = [0; 26];

    let mut nodes = Vec::with_capacity(26);
    for i in 0..26 {
        nodes.push(Node::new(i));
    }

    for edge in edges {
        addNodeDependency(edge.0, edge.1, &mut nodes);
    }

    let mut queue: BinaryHeap<&Node> = BinaryHeap::new();
    for node in nodes.iter().filter(|n| n.dependencies == 0) {
        // println!("Queueing {}", node.get_char());
        queue.push(node);
    }

    while visitedCount < 26 {
        let node = queue.pop().unwrap();
        // println!("Visiting {}", node.get_char());

        utils::set_bit(&mut visited, node.id);
        visitOrder[visitedCount] = node.id;
        visitedCount += 1;

        for i in 0..26 {
            if utils::is_bit_set(&node.dependents, i) {
                let nextNode = &nodes[i];
                // println!("Checking dependent {}...", nextNode.get_char());

                if utils::intersection(nextNode.dependencies, visited) == nextNode.dependencies {
                    queue.push(nextNode);
                    // println!("Queueing {}", nextNode.get_char());
                }
            }
        }

        // println!("Finished visiting {}", node.get_char());
    }

    let order = visitOrder
        .iter()
        .map(|&i| nodes[i].get_char())
        .collect::<String>();

    println!("Instruction Order = {}", order);

    println!("Running Day 7 - b");

    Ok(())
}

fn addNodeDependency(from: usize, to: usize, nodes: &mut Vec<Node>) {
    // println!("Tracking dependency from {} to {}", nodes[from].get_char(), nodes[to].get_char());
    utils::set_bit(&mut nodes[from].dependents, to);
    utils::set_bit(&mut nodes[to].dependencies, from);
}

#[derive(Eq, PartialEq)]
struct Node {
    id: usize,
    dependencies: u32,
    dependents: u32,
}

impl Node {
    fn new(id: usize) -> Node {
        Node {
            id: id,
            dependencies: 0,
            dependents: 0,
        }
    }

    fn get_char(&self) -> char {
        (self.id as u8 + 65) as char
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        other.id.cmp(&self.id)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
