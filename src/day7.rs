use crate::utils;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt;

const NUM_NODES: usize = 26;

type Edge = (usize, usize);

pub fn day7() -> std::io::Result<()> {
    println!("Running Day 7 - a");

    let rex = Regex::new(r"Step (\w) .* step (\w)").unwrap();

    let edges: Vec<Edge> = utils::readDay(7)?
        .iter()
        .map(|l| rex.captures(l).unwrap())
        .map(|c| (Node::get_id_from_name(&c[1]), Node::get_id_from_name(&c[2])))
        .collect();

    let mut day7 = Day7::new(&edges);

    let visitOrder: [usize; NUM_NODES] = day7.solve_a();

    let order = visitOrder
        .iter()
        .map(|&i| day7.nodes[i].get_name())
        .collect::<String>();

    println!("Instruction Order = {}", order);

    println!("Running Day 7 - b");

    Ok(())
}

struct Day7 {
    nodes: Vec<Node>,
}

impl Day7 {
    fn new(edges: &Vec<Edge>) -> Day7 {
        let mut day7 = Day7 {
            nodes: Vec::with_capacity(NUM_NODES),
        };

        for i in 0..NUM_NODES {
            day7.nodes.push(Node::new(i));
        }

        for edge in edges {
            utils::set_bit(&mut day7.nodes[edge.0].dependents, edge.1);
            utils::set_bit(&mut day7.nodes[edge.1].dependencies, edge.0);
        }

        day7
    }

    fn solve_a(&mut self) -> [usize; NUM_NODES] {
        let mut visitOrder: [usize; NUM_NODES] = [0; NUM_NODES];
        let mut visitCount = 0;
        let mut iter = self.iter();

        while let Some(node) = iter.next() {
            iter.visit(node);
            visitOrder[visitCount] = node.id;
            visitCount += 1;
            // println!("Finished visiting {:?}", node);
        }

        visitOrder
    }

    fn iter(&self) -> Day7Iterator<'_> {
        Day7Iterator::new(self)
    }
}

struct Day7Iterator<'a> {
    day7: &'a Day7,
    visited: u32,
    queue: BinaryHeap<&'a Node>,
}

impl Day7Iterator<'_> {
    fn new(day7: &Day7) -> Day7Iterator {
        let mut iterator = Day7Iterator {
            day7: day7,
            visited: 0,
            queue: BinaryHeap::with_capacity(NUM_NODES),
        };

        for node in day7.nodes.iter().filter(|n| n.dependencies == 0) {
            // println!("Queueing {:?}", node);
            iterator.queue.push(node);
        }

        iterator
    }

    fn visit(&mut self, node: &Node) {
        // println!("Visiting {:?}", node);

        utils::set_bit(&mut self.visited, node.id);

        for i in 0..NUM_NODES {
            if utils::is_bit_set(&node.dependents, i) {
                let nextNode = &self.day7.nodes[i];
                // println!("Checking dependent {:?}...", nextNode);

                if utils::intersection(nextNode.dependencies, self.visited) == nextNode.dependencies
                {
                    // println!("Queueing {:?}", nextNode);
                    self.queue.push(nextNode);
                }
            }
        }
    }
}

impl<'a> Iterator for Day7Iterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<&'a Node> {
        let next = self.queue.pop();
        // match next {
        //     Some(node) => {
        //         println!("Iterator :: next={:?}", node);
        //     }
        //     _ => {}
        // }
        next
    }
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

    fn get_name(&self) -> String {
        ((self.id as u8 + 65) as char).to_string()
    }

    fn get_id_from_name(name: &str) -> usize {
        (utils::tryParse_char(name) - 65) as usize
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_name())
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
