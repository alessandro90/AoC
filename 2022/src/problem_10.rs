#![allow(dead_code)]

use crate::utilities::read_file;

fn get_input() -> String {
    read_file("problem_10_sample")
    // read_file("problem_10_input")
}

fn parse_input() -> Vec<Instruction> {
    get_input()
        .lines()
        .map(|ln| {
            if ln.starts_with("noop") {
                Instruction::Noop
            } else {
                let n: i64 = ln.split_once(' ').unwrap().1.parse().unwrap();
                Instruction::Addx(n)
            }
        })
        .collect()
}

enum Instruction {
    Noop,
    Addx(i64),
}

struct Cpu {
    reg: i64,
    cycles: Vec<i64>,
}

impl Cpu {
    fn new() -> Self {
        Self {
            reg: 1,
            cycles: vec![],
        }
    }

    fn cycle(&mut self, times: u32) {
        for _ in 0..times {
            self.cycles.push(self.reg);
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Noop => self.cycle(1),
            Instruction::Addx(x) => {
                self.cycle(2);
                self.reg += x;
            }
        };
    }

    fn signal_strength(&mut self, cycle_nr: usize) -> i64 {
        self.cycles[cycle_nr - 1] * (cycle_nr as i64)
    }
}

fn solution_part_1() -> i64 {
    let instructions = parse_input();
    let mut cpu = Cpu::new();
    for instruction in instructions {
        cpu.execute(instruction);
    }
    [20usize, 60, 100, 140, 180, 220]
        .iter()
        .map(|&cycle| cpu.signal_strength(cycle))
        .sum()
}

fn solution_part_2() -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::problem_10::*;

    #[test]
    fn problem_10_solution_part_1_test() {
        println!("problem 10 solution 1: {}", solution_part_1());
    }

    #[test]
    fn problem_10_solution_part_2_test() {
        println!("problem 10 solution 2: {}", solution_part_2());
    }
}
