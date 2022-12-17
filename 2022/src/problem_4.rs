#![allow(dead_code)]

use crate::utilities::read_file;

fn get_input() -> String {
    read_file("problem_4_input")
}

struct Range {
    min: u64,
    max: u64,
}

fn range_from_ids(ids: &str) -> Range {
    let (min, max) = ids.split_once('-').unwrap();
    Range {
        min: min.parse::<u64>().unwrap(),
        max: max.parse::<u64>().unwrap(),
    }
}

fn parse_input() -> Vec<(Range, Range)> {
    get_input()
        .lines()
        .map(|ln| {
            let (rng_1, rng_2) = ln.split_once(',').unwrap();
            (range_from_ids(rng_1), range_from_ids(rng_2))
        })
        .collect()
}

fn fully_contains((rng_1, rng_2): &(Range, Range)) -> bool {
    (rng_1.min <= rng_2.min && rng_1.max >= rng_2.max)
        || (rng_2.min <= rng_1.min && rng_2.max >= rng_1.max)
}

fn overlaps((rng_1, rng_2): &(Range, Range)) -> bool {
    !(rng_2.min > rng_1.max || rng_2.max < rng_1.min)
        || !(rng_1.min > rng_2.max || rng_1.max < rng_2.min)
}

fn generic_solution<F>(f: F) -> u64
where
    F: Fn(&(Range, Range)) -> bool,
{
    parse_input().into_iter().filter(f).count() as u64
}

fn solution_part_1() -> u64 {
    generic_solution(fully_contains)
}

fn solution_part_2() -> u64 {
    generic_solution(overlaps)
}

#[cfg(test)]
mod tests {
    use crate::problem_4::*;

    #[test]
    fn problem_4_solution_part_1_test() {
        println!("problem 4 solution 1: {}", solution_part_1());
    }

    #[test]
    fn problem_4_solution_part_2_test() {
        println!("problem 4 solution 2: {}", solution_part_2());
    }
}
