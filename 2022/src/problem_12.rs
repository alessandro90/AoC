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

    fn reacheable_node_positions<BarrierFn>(
        &self,
        pos @ Position { x, y }: Position,
        barrier_function: BarrierFn,
    ) -> Vec<Position>
    where
        BarrierFn: Fn(i16, i16) -> bool + Copy,
    {
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
        .filter(|&p| self.check_position(pos, p, barrier_function))
        .collect()
    }
}

fn parse_input() -> Graph {
    let text = get_input();
    text.lines()
        .enumerate()
        .map(|(x, ln)| {
            ln.char_indices()
                .map(|(y, c)| {
                    let (height, category) = match c {
                        'S' => ('a' as i16, Category::Start),
                        'E' => ('z' as i16, Category::End),
                        _ => (c as i16, Category::Generic),
                    };
                    Node {
                        height,
                        category,
                        distance: Distance::Infinite,
                        position: Position { x, y },
                    }
                })
                .collect()
        })
        .fold(Graph { nodes: vec![] }, |mut graph, row| {
            graph.nodes.push(row);
            graph
        })
}

fn dijkstra<EndPredicate, BarrierFn>(
    mut graph: Graph,
    start: Position,
    end_predicate: EndPredicate,
    barrier_function: BarrierFn,
) -> u64
where
    EndPredicate: Fn(&Node) -> bool,
    BarrierFn: Fn(i16, i16) -> bool + Copy,
{
    let mut unvisited = BinaryHeap::new();
    unvisited.push(graph.get_node(start));

    while let Some(node) = unvisited.pop() {
        if end_predicate(&node) {
            return match node.distance {
                Distance::Infinite => panic!("Invalid distance"),
                Distance::Finite(d) => d,
            };
        }
        for nearest in graph.reacheable_node_positions(node.position, barrier_function) {
            match node.distance {
                Distance::Infinite => panic!("Invalid distance"),
                Distance::Finite(d) => {
                    let mut nearest_node = graph.get_node(nearest);
                    let cost = 1;
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
    let start = graph.find_start();
    graph.nodes[start.x][start.y].distance = Distance::Finite(0);
    let end = graph.find_end();
    dijkstra(
        graph,
        start,
        |node| node.position == end,
        |h1, h2| h2 - h1 <= 1,
    )
}

fn solution_part_2() -> u64 {
    let mut graph = parse_input();
    let start = graph.find_end();
    graph.nodes[start.x][start.y].distance = Distance::Finite(0);
    dijkstra(
        graph,
        start,
        |node| node.height == ('a' as i16),
        |h1, h2| h1 - h2 <= 1,
    )
}

#[cfg(test)]
mod tests {
    use crate::problem_12::*;

    #[test]
    fn problem_12_solution_part_1_test() {
        println!("problem 12 solution 1: {}", solution_part_1());
    }

    #[test]
    fn problem_12_solution_part_2_test() {
        println!("problem 12 solution 2: {}", solution_part_2());
    }
}
