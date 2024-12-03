#![allow(dead_code)]

use crate::utilities::{as_maybe_num, read_file};

fn get_input() -> String {
    read_file("problem_5_sample")
}

struct Instruction {
    crates: usize,
    from: usize,
    to: usize,
}

struct Stacks {
    configuration: Vec<Vec<char>>,
}

impl Stacks {
    fn execute(&mut self, &Instruction { crates, from, to }: &Instruction) {
        for _ in 0..crates {
            let to_move = self.configuration[from].pop().unwrap();
            self.configuration[to].push(to_move);
        }
    }

    fn execute_keep_order(&mut self, &Instruction { crates, from, to }: &Instruction) {
        let from_len = self.configuration[from].len();
        let mut to_push: Vec<_> = self.configuration[from][from_len - crates..from_len]
            .iter()
            .cloned()
            .collect();
        self.configuration[to].append(&mut to_push);
        self.configuration[from].truncate(from_len - crates);
    }

    fn get_crates_order(self) -> String {
        self.configuration
            .into_iter()
            .map(|stack| stack.last().unwrap().clone())
            .collect()
    }
}

fn parse_stack(stack_str: &str) -> char {
    stack_str.chars().skip(1).next().unwrap()
}

fn parse_stacks(stacks_str: String) -> Stacks {
    let mut lines = stacks_str.lines().rev();
    let col_nums = lines.next().unwrap();
    let col_indexes: Vec<_> = col_nums
        .char_indices()
        .filter_map(|(s, c)| as_maybe_num::<usize>(c).map(|digit| (digit - 1, s)))
        .collect();

    let mut stacks = Stacks {
        configuration: vec![vec![]; col_indexes.len()],
    };
    for ln in lines {
        for (vec_idx, row_idx) in &col_indexes {
            let mut chars = ln.chars();
            let ch = chars.nth(*row_idx).unwrap();
            if ch != ' ' {
                stacks.configuration[*vec_idx].push(ch);
            }
        }
    }
    stacks
}

fn parse_instruction(s: &str) -> Instruction {
    let args: Vec<_> = s
        .split_whitespace()
        .filter_map(|chunk| chunk.parse::<usize>().ok())
        .collect();
    Instruction {
        crates: args[0],
        from: args[1] - 1,
        to: args[2] - 1,
    }
}

fn parse_instructions(s: &str) -> Vec<Instruction> {
    s.lines().map(parse_instruction).collect()
}

fn parse_input() -> (Stacks, Vec<Instruction>) {
    let input = get_input();
    let mut parts = input.split_terminator("\r\n\r\n");
    let stacks = parse_stacks(parts.next().unwrap().to_owned());
    let instructions = parse_instructions(parts.next().unwrap());
    (stacks, instructions)
}

fn generic_solution<F>(executor: F) -> String
where
    F: Fn(&mut Stacks, &Instruction) -> (),
{
    let (mut stacks, instructions) = parse_input();
    for instruction in instructions {
        executor(&mut stacks, &instruction);
    }
    stacks.get_crates_order()
}

fn solution_part_1() -> String {
    generic_solution(|stacks, instruction| stacks.execute(instruction))
}

fn solution_part_2() -> String {
    generic_solution(|stacks, instruction| stacks.execute_keep_order(instruction))
}

#[cfg(test)]
mod tests {
    use crate::problem_5::*;

    #[test]
    fn problem_5_solution_part_1_test() {
        println!("problem 5 solution 1: {}", solution_part_1());
    }

    #[test]
    fn problem_5_solution_part_2_test() {
        println!("problem 5 solution 2: {}", solution_part_2());
    }
}
