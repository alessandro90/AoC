#![allow(dead_code)]

use std::collections::HashSet;

use crate::utilities::read_file;

fn get_input() -> String {
    // read_file("problem_9_input")
    read_file("problem_9_input")
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

#[derive(Debug, Copy, Clone)]
struct Move {
    direction: Direction,
    amount: i64,
}

type Moves = Vec<Move>;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Position(i64, i64);

type Positions = HashSet<Position>;

struct Head {
    position: Position,
}

impl Head {
    fn update(&mut self, Move { direction, amount }: Move) {
        match direction {
            Direction::Down => self.position.0 -= amount,
            Direction::Up => self.position.0 += amount,
            Direction::Left => self.position.1 -= amount,
            Direction::Right => self.position.1 += amount,
        };
    }
}

struct Tail {
    position: Position,
}

impl Tail {
    fn update(&mut self, &Position(x, y): &Position) {
        let delta_x = x - self.position.0;
        let delta_y = y - self.position.1;
        if delta_x == 0 {
            if delta_y.abs() > 1 {
                self.position.1 += 1 * delta_y.signum();
            }
            return;
        }
        if delta_y == 0 {
            if delta_x.abs() > 1 {
                self.position.0 += 1 * delta_x.signum();
            }
            return;
        }
        if delta_x.abs() > 1 || delta_y.abs() > 1 {
            self.position.0 += 1 * delta_x.signum();
            self.position.1 += 1 * delta_y.signum();
        }
    }
}

fn parse_input() -> Moves {
    get_input()
        .lines()
        .map(|ln| {
            let (letter, number) = ln.split_once(' ').unwrap();
            let number: i64 = number.parse().unwrap();
            match letter {
                "R" => Move {
                    direction: Direction::Right,
                    amount: number,
                },
                "U" => Move {
                    direction: Direction::Up,
                    amount: number,
                },
                "L" => Move {
                    direction: Direction::Left,
                    amount: number,
                },
                "D" => Move {
                    direction: Direction::Down,
                    amount: number,
                },
                x => panic!("Invalid command {}", x),
            }
        })
        .collect()
}

fn solution_part_1() -> u64 {
    let mut head = Head {
        position: Position(0, 0),
    };
    let mut tail = Tail {
        position: Position(0, 0),
    };
    let mut tail_positions: Positions = HashSet::from([Position(0, 0)]);
    let moves = parse_input();
    for m @ Move {
        direction: _,
        amount,
    } in moves
    {
        for _ in 0..amount {
            head.update(Move { amount: 1, ..m });
            tail.update(&head.position);
            tail_positions.insert(tail.position);
        }
    }
    tail_positions.len() as u64
}

fn solution_part_2() -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::problem_9::*;

    #[test]
    fn problem_9_solution_part_1_test() {
        println!("problem 9 solution 1: {}", solution_part_1());
    }

    #[test]
    fn problem_9_solution_part_2_test() {
        println!("problem 9 solution 2: {}", solution_part_2());
    }
}
