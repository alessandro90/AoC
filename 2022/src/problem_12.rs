#![allow(dead_code)]

use std::{cmp::Ordering, collections::BinaryHeap};

use crate::utilities::read_file;

fn get_input() -> String {
    // read_file("problem_12_sample")
    read_file("problem_12_input")
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Category {
    Start,
    End,
    Generic,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Distance {
    Infinite,
    Finite(u64),
}

impl Distance {
    fn is_infinite(&self) -> bool {
        *self == Self::Infinite
    }
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Infinite, Self::Infinite) => Ordering::Equal,
            (Self::Infinite, _) => Ordering::Greater,
            (Self::Finite(_), Self::Infinite) => Ordering::Less,
            (Self::Finite(lhs), Self::Finite(rhs)) => lhs.cmp(rhs),
        }
    }
}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Node {
    height: i16,
    category: Category,
    distance: Distance,
    position: Position,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Nodes = Vec<Vec<Node>>;

#[derive(Debug)]
struct Graph {
    nodes: Nodes,
}

impl Graph {
    fn find_position(&self, category: Category) -> Position {
        self.nodes
            .iter()
            .find_map(|row| {
                row.iter().find_map(|node| {
                    if node.category == category {
                        Some(node.position)
                    } else {
                        None
                    }
                })
            })
            .unwrap()
    }

    fn find_end(&self) -> Position {
        self.find_position(Category::End)
    }

    fn find_start(&self) -> Position {
        self.find_position(Category::Start)
    }

    fn check_position<BarrierFn>(
        &self,
        center: Position,
        near: Position,
        barrier_function: BarrierFn,
    ) -> bool
    where
        BarrierFn: Fn(i16, i16) -> bool,
    {
        center != near
            && barrier_function(
                self.nodes[center.x][center.y].height,
                self.nodes[near.x][near.y].height,
            )
    }

    fn get_node(&self, Position { x, y }: Position) -> Node {
        self.nodes[x][y]
    }

    fn reacheable_node_positions(&self, pos @ Position { x, y }: Position) -> Vec<Position> {
        let inf_x = if x == 0 { 0 } else { x - 1 };
        let inf_y = if y == 0 { 0 } else { y - 1 };
        let sup_x = if x == self.nodes.len() - 1 { x } else { x + 1 };
        let sup_y = if y == self.nodes[0].len() - 1 {
            y
        } else {
            y + 1
        };

        [
            Position { x, y: inf_y },
            Position { x, y: sup_y },
            Position { x: inf_x, y },
            Position { x: sup_x, y },
        ]
        .into_iter()
        .filter(|&p| self.check_position(pos, p, |h1, h2| h2 - h1 <= 1))
        .collect()
    }
}

fn parse_input() -> Graph {
    let text = get_input();
    text.lines()
        .enumerate()
        .map(|(x, ln)| {
            ln.char_indices()
                .map(|(y, c)| match c {
                    'S' => Node {
                        height: 'a' as i16,
                        category: Category::Start,
                        distance: Distance::Finite(0),
                        position: Position { x, y },
                    },
                    'E' => Node {
                        height: 'z' as i16,
                        category: Category::End,
                        distance: Distance::Infinite,
                        position: Position { x, y },
                    },
                    ch => Node {
                        height: ch as i16,
                        category: Category::Generic,
                        distance: Distance::Infinite,
                        position: Position { x, y },
                    },
                })
                .collect()
        })
        .fold(Graph { nodes: vec![] }, |mut graph, row| {
            graph.nodes.push(row);
            graph
        })
}

fn dijkstra(graph: &mut Graph) {
    let start = graph.find_start();
    let end = graph.find_end();
    let mut unvisited = BinaryHeap::new();
    unvisited.push(graph.get_node(start));

    while let Some(node) = unvisited.pop() {
        if node.position == end {
            return;
        }
        for nearest in graph.reacheable_node_positions(node.position) {
            let mut nearest_node = graph.get_node(nearest);
            let cost = 1;
            match node.distance {
                Distance::Infinite => panic!("Invalid distance"),
                Distance::Finite(d) => {
                    let tentative_dist = Distance::Finite(d + cost);
                    if tentative_dist < nearest_node.distance {
                        nearest_node.distance = tentative_dist;
                        graph.nodes[nearest.x][nearest.y].distance = tentative_dist;
                        unvisited.push(nearest_node);
                    }
                }
            }
        }
    }
    panic!("End never reached");
}

fn solution_part_1() -> u64 {
    let mut graph = parse_input();
    dijkstra(&mut graph);
    let end_node_pos = graph.find_end();
    match graph.get_node(end_node_pos).distance {
        Distance::Infinite => panic!("Invalid infinite distance"),
        Distance::Finite(d) => d,
    }
}

fn solution_part_2() -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::problem_12::*;

    #[test]
    fn problem_11_solution_part_1_test() {
        println!("problem 12 solution 1: {}", solution_part_1());
    }

    #[test]
    fn problem_11_solution_part_2_test() {
        println!("problem 12 solution 2: {}", solution_part_2());
    }
}
