#![allow(dead_code)]

use crate::utilities::read_file;

fn get_input() -> String {
    read_file("problem_8_input")
    // read_file("problem_8_sample")
}

type Height = u8;
type TreeField = Vec<Vec<Height>>;

#[derive(Clone, Copy)]
struct Tree {
    x: usize,
    y: usize,
    h: Height,
}

impl Tree {
    fn is_edge(&self) -> bool {
        self.x == 0 || self.y == 0
    }
}

struct Field {
    tree_field: TreeField,
}

impl Field {
    fn iter(&self) -> FieldIter {
        FieldIter::new(&self.tree_field)
    }

    fn is_visible(&self, tree: Tree) -> bool {
        if tree.is_edge() {
            return true;
        }
        let rows = self.tree_field.len();
        let cols = self.tree_field[0].len();
        let Tree { x, y, h } = tree;
        let from_above: Vec<_> = self.tree_field[0..x].iter().map(|c| c[y]).collect();
        let from_below: Vec<_> = self.tree_field[x + 1..rows].iter().map(|c| c[y]).collect();
        Self::is_line_visible(h, &self.tree_field[x][0..y])
            || Self::is_line_visible(h, &self.tree_field[x][y + 1..cols])
            || Self::is_line_visible(h, &from_below)
            || Self::is_line_visible(h, &from_above)
    }

    fn scenic_score(&self, tree: Tree) -> u64 {
        if tree.is_edge() {
            return 0;
        }
        let rows = self.tree_field.len();
        let cols = self.tree_field[0].len();
        let Tree { x, y, h } = tree;
        let from_above: Vec<_> = self.tree_field[0..x].iter().rev().map(|c| c[y]).collect();
        let from_below: Vec<_> = self.tree_field[x + 1..rows].iter().map(|c| c[y]).collect();
        let from_left: Vec<_> = self.tree_field[x][0..y].iter().rev().cloned().collect();
        let from_right: Vec<_> = self.tree_field[x][y + 1..cols].iter().cloned().collect();

        Self::line_score(h, &from_above)
            * Self::line_score(h, &from_below)
            * Self::line_score(h, &from_left)
            * Self::line_score(h, &from_right)
    }

    fn is_line_visible(h: Height, line: &[Height]) -> bool {
        line.iter().all(|&x| x < h)
    }

    fn line_score(h: Height, line: &[Height]) -> u64 {
        let mut score = 0;
        for &tree_height in line {
            score += 1;
            if tree_height >= h {
                break;
            }
        }
        score
    }
}

struct FieldIter<'a> {
    x: usize,
    y: usize,
    tree_field: &'a [Vec<Height>],
}

impl<'a> FieldIter<'a> {
    fn new(tree_field: &'a [Vec<Height>]) -> Self {
        Self {
            x: 0,
            y: 0,
            tree_field,
        }
    }
}

impl<'a> Iterator for FieldIter<'a> {
    type Item = Tree;

    fn next(&mut self) -> Option<Self::Item> {
        let rows = self.tree_field.len();
        if self.x == rows {
            return None;
        }
        let cols = if rows > 0 {
            self.tree_field[0].len()
        } else {
            0
        };
        let item = Some(Self::Item {
            x: self.x,
            y: self.y,
            h: self.tree_field[self.x][self.y],
        });
        self.y += 1;
        if self.y == cols {
            self.y = 0;
            self.x += 1;
        }
        item
    }
}

fn parse_input() -> Field {
    let tree_field = get_input().lines().fold(vec![], |mut acc, ln| {
        acc.push(
            ln.chars()
                .map(|c| c.to_digit(10).unwrap() as Height)
                .collect(),
        );
        acc
    });
    Field { tree_field }
}

fn solution_part_1() -> usize {
    let field = parse_input();
    field.iter().filter(|&tree| field.is_visible(tree)).count()
}

fn solution_part_2() -> u64 {
    let field = parse_input();
    field
        .iter()
        .map(|tree| field.scenic_score(tree))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::problem_8::*;

    #[test]
    fn problem_8_solution_part_1_test() {
        println!("problem 8 solution 1: {}", solution_part_1());
    }

    #[test]
    fn problem_8_solution_part_2_test() {
        println!("problem 8 solution 2: {}", solution_part_2());
    }
}
