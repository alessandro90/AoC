#![allow(dead_code)]

use std::collections::HashSet;

use crate::utilities::read_file;

fn get_input() -> String {
    read_file("problem_6_input")
}

fn advance_window<const N: usize>(arr: &mut [char; N], c: char) {
    for i in 0..(N - 1) {
        arr[i] = arr[i + 1];
    }
    arr[N - 1] = c;
}

fn is_packet<const N: usize>(arr: &[char; N]) -> bool {
    HashSet::from(*arr).len() == N
}

fn generic_solution<const N: usize>() -> u64 {
    let mut packet = ['\0'; N];
    for (i, c) in get_input().char_indices() {
        if i < N {
            packet[i] = c;
            continue;
        }
        if is_packet(&packet) {
            return i as u64;
        }
        advance_window(&mut packet, c);
    }
    panic!("Invalid input");
}

fn solution_part_1() -> u64 {
    generic_solution::<4>()
}

fn solution_part_2() -> u64 {
    generic_solution::<14>()
}

#[cfg(test)]
mod tests {
    use crate::problem_6::*;

    #[test]
    fn problem_6_solution_part_1_test() {
        println!("problem 6 solution 1: {}", solution_part_1());
    }

    #[test]
    fn problem_6_solution_part_2_test() {
        println!("problem 6 solution 2: {}", solution_part_2());
    }
}
