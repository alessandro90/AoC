#![allow(dead_code)]

use std::{collections::HashSet, fmt::Debug, str::FromStr};

use crate::utilities::read_file;

fn get_input() -> String {
    // read_file("problem_11_sample")
    read_file("problem_11_input")
}

type Integral = u64;
type MonkeyIndexWithItem = (usize, Integral);
type ItemsToThrow = Vec<MonkeyIndexWithItem>;

#[derive(Debug, Clone)]
enum Operation {
    AddIntegral(Integral),
    AddOld,
    MultiplyIntegral(Integral),
    MultiplyOld,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<Integral>,
    operation: Operation,
    divisibility_check: Integral,
    monkey_true: usize,
    monkey_false: usize,
    inspected: u64,
}

impl Monkey {
    fn new() -> Self {
        Self {
            items: vec![],
            operation: Operation::AddOld,
            divisibility_check: 1,
            monkey_true: 0,
            monkey_false: 0,
            inspected: 0,
        }
    }

    fn play(&mut self) -> ItemsToThrow {
        self.inspected += self.items.len() as u64;
        let to_throw = self.inspect_items(|stress| stress / 3);
        self.items.clear();
        to_throw
    }

    fn play_part_2(&mut self, gcd: u64) -> ItemsToThrow {
        self.inspected += self.items.len() as u64;
        let to_throw = self.inspect_items(|stress| stress % gcd);
        self.items.clear();
        to_throw
    }

    fn inspect_items(&mut self, stess_reducer: impl Fn(Integral) -> Integral) -> ItemsToThrow {
        self.items
            .iter()
            .map(|&item| {
                let item = match self.operation {
                    Operation::AddOld => item * 2,
                    Operation::AddIntegral(n) => item + n,
                    Operation::MultiplyOld => item * item,
                    Operation::MultiplyIntegral(n) => item * n,
                };
                let item = stess_reducer(item);
                let id = if item % self.divisibility_check == 0 {
                    self.monkey_true
                } else {
                    self.monkey_false
                };
                (id, item)
            })
            .collect()
    }

    fn catch_item(&mut self, item: Integral) {
        self.items.push(item);
    }
}

fn split_at_colon(ln: &str) -> (&str, &str) {
    ln.split_once(": ").unwrap()
}

fn parse_operation(operation: &str) -> Operation {
    let rhs = operation.split_once(" = ").unwrap().1;
    let parts: Vec<_> = rhs.splitn(3, ' ').collect();
    let operand = parts[1];
    let value = parts[2];
    match (operand, value) {
        ("+", "old") => Operation::AddOld,
        ("+", n) => Operation::AddIntegral(n.parse().unwrap()),
        ("*", "old") => Operation::MultiplyOld,
        ("*", n) => Operation::MultiplyIntegral(n.parse().unwrap()),
        (x, y) => panic!("Invalid operation: {} {}", x, y),
    }
}

fn get_last_integer<I: FromStr>(ln: &str) -> I
where
    I::Err: Debug,
{
    ln.split_whitespace().last().unwrap().parse().unwrap()
}

fn parse_monkey(monkey_txt: &str) -> Monkey {
    let mut monkey = Monkey::new();
    monkey_txt
        .lines()
        .skip(1)
        .map(|ln| ln.trim_start())
        .for_each(|ln| {
            let (prefix, value) = split_at_colon(ln);
            match prefix {
                "Starting items" => {
                    monkey.items = value.split(", ").map(|n| n.parse().unwrap()).collect()
                }
                "Operation" => monkey.operation = parse_operation(value),
                "Test" => monkey.divisibility_check = get_last_integer::<Integral>(value),
                "If true" => monkey.monkey_true = get_last_integer::<usize>(value),
                "If false" => monkey.monkey_false = get_last_integer::<usize>(value),
                _ => panic!("Invalid input line {}", ln),
            };
        });
    monkey
}

fn parse_input() -> Vec<Monkey> {
    get_input()
        .split_terminator("\r\n\r\n")
        .map(parse_monkey)
        .collect()
}

fn play_round(monkeys: &mut [Monkey], play_fn: impl Fn(&mut Monkey) -> Vec<(usize, u64)>) {
    for i in 0..monkeys.len() {
        let indexed_items = play_fn(&mut monkeys[i]);
        for (id, item) in indexed_items {
            monkeys[id].catch_item(item);
        }
    }
}

fn solution_part_1() -> Integral {
    let mut monkeys = parse_input();
    for _ in 0..20 {
        play_round(&mut monkeys, |monkey| monkey.play());
    }
    monkeys.sort_by(|ma, mb| mb.inspected.cmp(&ma.inspected));
    monkeys[0].inspected * monkeys[1].inspected
}

fn solution_part_2() -> Integral {
    let mut monkeys = parse_input();
    let gcd: HashSet<_> = monkeys.iter().map(|m| m.divisibility_check).collect();
    let gcd = gcd.iter().fold(1, |acc, n| acc * n);
    for _ in 0..10000 {
        play_round(&mut monkeys, |monkey| monkey.play_part_2(gcd));
    }
    monkeys.sort_by(|ma, mb| mb.inspected.cmp(&ma.inspected));
    monkeys[0].inspected * monkeys[1].inspected
}

#[cfg(test)]
mod tests {
    use crate::problem_11::*;

    #[test]
    fn problem_11_solution_part_1_test() {
        println!("problem 11 solution 1: {}", solution_part_1());
    }

    #[test]
    fn problem_11_solution_part_2_test() {
        println!("problem 11 solution 2: {}", solution_part_2());
    }
}
