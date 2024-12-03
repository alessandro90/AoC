#![allow(dead_code)]

use crate::utilities::read_file;

fn get_input() -> String {
    read_file("problem_1_input")
}

fn get_calories_per_elf(input: String) -> Vec<usize> {
    input
        .split_terminator("\n\n")
        .map(|elf_calories_list| {
            elf_calories_list
                .split_terminator('\n')
                .map(|calories| calories.parse::<usize>().unwrap())
                .sum()
        })
        .collect()
}

fn solution_part_1() -> usize {
    let content = get_input();
    get_calories_per_elf(content).iter().max().unwrap().clone()
}

fn solution_part_2() -> usize {
    let content = get_input();
    let mut total_calories_per_elf = get_calories_per_elf(content);
    total_calories_per_elf.sort_by(|a, b| b.cmp(a));
    total_calories_per_elf.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use crate::problem_1::*;

    #[test]
    fn problem_1_solution_part_1_test() {
        let result = solution_part_1();
        println!("Solution 1 to problem_1: {}", result);
    }

    #[test]
    fn problem_1_solution_part_2_test() {
        let result = solution_part_2();
        println!("Solution 2 to problem_1: {}", result);
    }
}
