#![allow(dead_code)]

use crate::utilities::read_file;

fn get_input() -> String {
    // read_file("problem_10_sample")
    read_file("problem_10_input")
}

fn parse_input() -> Vec<Instruction> {
    get_input()
        .lines()
        .map(|ln| {
            if ln.starts_with("noop") {
                Instruction::Noop
            } else {
                let n: i32 = ln.split_once(' ').unwrap().1.parse().unwrap();
                Instruction::Addx(n)
            }
        })
        .collect()
}

enum Instruction {
    Noop,
    Addx(i32),
}

const SCREEN_HEIGHT: usize = 6;
const SCREEN_WIDTH: usize = 40;

type Screen = [[char; SCREEN_WIDTH]; SCREEN_HEIGHT];

struct Crt {
    pixels: Screen,
    sprite_center: i32,
    row: usize,
    col: usize,
}

impl Crt {
    fn new() -> Self {
        Self {
            pixels: [[' '; SCREEN_WIDTH]; SCREEN_HEIGHT],
            sprite_center: 1,
            row: 0,
            col: 0,
        }
    }

    fn set_sprite_center(&mut self, center: i32) {
        self.sprite_center = center;
    }

    fn draw_pixel(&mut self) {
        if self.is_pixel_visible() {
            self.pixels[self.row][self.col] = '#';
        }
        if self.col == SCREEN_WIDTH - 1 {
            self.col = 0;
            self.row += 1;
        } else {
            self.col += 1;
        }
    }

    fn is_pixel_visible(&self) -> bool {
        let col = self.col as i32;
        (col >= self.sprite_center && col - self.sprite_center <= 1)
            || (col < self.sprite_center && self.sprite_center - col == 1)
    }

    fn display(&self) -> String {
        self.pixels.into_iter().fold("".to_owned(), |acc, row| {
            let r = row.iter().collect::<String>();
            acc + &r + "\n"
        })
    }
}

struct Cpu {
    reg: i32,
    cycles: Vec<i32>,
    crt: Crt,
}

impl Cpu {
    fn new() -> Self {
        Self {
            reg: 1,
            cycles: vec![],
            crt: Crt::new(),
        }
    }

    fn cycle(&mut self, times: u32) {
        for _ in 0..times {
            self.cycles.push(self.reg);
            self.crt.draw_pixel();
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Noop => self.cycle(1),
            Instruction::Addx(x) => {
                self.cycle(2);
                self.reg += x;
                self.crt.set_sprite_center(self.reg);
            }
        };
    }

    fn signal_strength(&mut self, cycle_nr: usize) -> i32 {
        self.cycles[cycle_nr - 1] * (cycle_nr as i32)
    }
}

fn solution_part_1() -> i32 {
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

fn solution_part_2() -> String {
    let instructions = parse_input();
    let mut cpu = Cpu::new();
    for instruction in instructions {
        cpu.execute(instruction);
    }
    cpu.crt.display()
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
        println!("problem 10 solution 2: \n{}", solution_part_2());
    }
}
