#![allow(dead_code)]

use crate::utilities::read_file;

fn get_input() -> String {
    read_file("problem_2_input")
}

fn parse_input() -> Vec<(char, char)> {
    let fcontent = get_input();
    fcontent
        .lines()
        .map(|line| {
            let (player_a_move, player_b_move) = line.split_once(' ').unwrap();
            (
                player_a_move.chars().next().unwrap(),
                player_b_move.chars().next().unwrap(),
            )
        })
        .collect()
}
// A === X === Rock
// B === Y === Paper
// C === Z === Scissors
// Lost => 0
// Win => 6
// Draw => 3
fn get_score_from_move(m: char) -> u64 {
    match m {
        'A' | 'X' => 1,
        'B' | 'Y' => 2,
        'C' | 'Z' => 3,
        _ => panic!("Invalid move"),
    }
}

fn get_round_score(player_moves: &(char, char)) -> u64 {
    get_score_from_move(player_moves.1)
        + match player_moves {
            ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
            ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
            ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
            _ => panic!("Invalid moves"),
        }
}

fn solution_part_1() -> u64 {
    parse_input().iter().map(get_round_score).sum()
}

// Y => Draw
// X => Lost
// Z => Win
fn get_round_moves((player_move, outcome): (char, char)) -> (char, char) {
    (
        player_move,
        match outcome {
            'X' => match player_move {
                'A' => 'C',
                'B' => 'A',
                'C' => 'B',
                _ => panic!("Invalid move"),
            },
            'Y' => player_move,
            'Z' => match player_move {
                'A' => 'B',
                'B' => 'C',
                'C' => 'A',
                _ => panic!("Invalid move"),
            },
            _ => panic!("Invalid strategy"),
        },
    )
}

fn get_round_score_v2(player_moves: (char, char)) -> u64 {
    get_score_from_move(player_moves.1)
        + match player_moves {
            ('A', 'B') | ('B', 'C') | ('C', 'A') => 6,
            ('A', 'A') | ('B', 'B') | ('C', 'C') => 3,
            ('A', 'C') | ('B', 'A') | ('C', 'B') => 0,
            _ => panic!("Invalid moves"),
        }
}

fn solution_part_2() -> u64 {
    parse_input()
        .into_iter()
        .map(tool::compose(get_round_score_v2, get_round_moves))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::problem_2::*;

    #[test]
    fn problem_2_solution_part_1_test() {
        println!("problem 2 solution 1: {}", solution_part_1());
    }

    #[test]
    fn problem_2_solution_part_2_test() {
        println!("problem 2 solution 2: {}", solution_part_2());
    }
}
