#![allow(dead_code)]

use std::collections::HashSet;

use crate::utilities::read_file;

fn get_input() -> String {
    read_file("problem_3_input")
}

fn parse_input() -> Vec<String> {
    get_input().lines().map(|line| line.to_owned()).collect()
}

fn parse_input_2() -> Vec<Vec<String>> {
    parse_input().chunks(3).map(|ch| ch.to_owned()).collect()
}

fn get_priority(letter: char) -> u64 {
    if letter.is_lowercase() {
        letter as u64 - 'a' as u64 + 1u64
    } else {
        letter as u64 - 'A' as u64 + 27u64
    }
}

fn find_duplicate_letter(first_half: &[char], second_half: &[char]) -> char {
    first_half
        .iter()
        .fold(None, |acc, &letter| {
            if second_half.contains(&letter) {
                Some(letter)
            } else {
                acc
            }
        })
        .unwrap()
}

fn split_line_into_vector_of_chars(line: String) -> (Vec<char>, Vec<char>) {
    let (first_half, second_half) = line.split_at(line.len() / 2);
    let first_half = first_half.chars().collect::<Vec<char>>();
    let second_half = second_half.chars().collect::<Vec<char>>();
    (first_half, second_half)
}

fn solution_part_1() -> u64 {
    parse_input()
        .into_iter()
        .map(|line| {
            let (first_half, second_half) = split_line_into_vector_of_chars(line);
            find_duplicate_letter(&first_half, &second_half)
        })
        .map(get_priority)
        .sum()
}

fn solution_part_2() -> u64 {
    parse_input_2()
        .into_iter()
        .map(|group| {
            let hash_1: HashSet<_> = HashSet::from_iter(group[0].chars());
            let hash_2: HashSet<_> = HashSet::from_iter(group[1].chars());
            let hash_3: HashSet<_> = HashSet::from_iter(group[2].chars());
            let intersection: HashSet<_> = hash_1.intersection(&hash_2).cloned().collect();
            let common_item: Vec<_> = intersection.intersection(&hash_3).cloned().collect();
            common_item[0]
        })
        .map(get_priority)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::problem_3::*;

    #[test]
    fn problem_3_solution_part_1_test() {
        println!("problem 3 solution 1: {}", solution_part_1());
    }

    #[test]
    fn problem_3_solution_part_2_test() {
        println!("problem 3 solution 2: {}", solution_part_2());
    }
}
