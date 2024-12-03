#![allow(dead_code)]

use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use crate::utilities::read_file;

const TOTAL_SPACE: u64 = 70_000_000;
const REQUIRED_SPACE: u64 = 30_000_000;
const ROOT_DIR_NAME: &str = "root";

fn get_input() -> String {
    read_file("problem_7_input")
    // read_file("problem_7_sample")
}

#[derive(Clone)]
struct File {
    name: String,
    size: u64,
}

type Node = Rc<RefCell<Directory>>;
type WeakNode = Weak<RefCell<Directory>>;

#[derive(Clone)]
struct Directory {
    name: String,
    children: Vec<Node>,
    parent: WeakNode,
    files: Vec<File>,
}

impl Directory {
    fn new(name: String, parent: WeakNode) -> Self {
        let parent_path = parent.upgrade().map(|p_dir| p_dir.borrow().name.clone());
        Directory {
            name: parent_path.map_or(name.clone(), |p| make_path(&p, &name)),
            children: vec![],
            parent,
            files: vec![],
        }
    }

    fn get_child(&self, name: &str, path: &str) -> WeakNode {
        let path_name = make_path(path, name);
        Rc::downgrade(
            self.children
                .iter()
                .find(|child| child.borrow().name == path_name)
                .unwrap(),
        )
    }
}

fn make_path(parent: &str, children: &str) -> String {
    format!("{}/{}", parent, children)
}

fn parse_command(curdir: WeakNode, ln: &str) -> WeakNode {
    let cmd = ln.split_whitespace().skip(1).collect::<Vec<_>>();
    match cmd[..] {
        ["ls"] => curdir,
        ["cd", ".."] => curdir.upgrade().unwrap().borrow().parent.clone(),
        ["cd", dirname] => {
            let cur = curdir.upgrade().unwrap();
            let cur = cur.borrow();
            cur.get_child(dirname, &cur.name)
        }
        _ => panic!("Invalid command {:?}", cmd),
    }
}

fn parse_directory(curdir: WeakNode, ln: &str) -> WeakNode {
    let (_, name) = ln.split_once(' ').unwrap();
    let new_dir = Directory::new(name.to_owned(), curdir.clone());
    let dir = curdir.upgrade().unwrap();
    let mut dir = dir.borrow_mut();
    dir.children.push(Rc::new(RefCell::new(new_dir)));
    curdir.clone()
}

fn parse_file(curdir: WeakNode, ln: &str) -> WeakNode {
    let (size, name) = ln.split_once(' ').unwrap();
    curdir.upgrade().unwrap().borrow_mut().files.push(File {
        name: name.to_owned(),
        size: size.parse().unwrap(),
    });
    curdir
}

fn parse_line(curdir: WeakNode, ln: &str) -> WeakNode {
    if ln.starts_with('$') {
        parse_command(curdir, ln)
    } else if ln.starts_with("dir ") {
        parse_directory(curdir, ln)
    } else {
        parse_file(curdir, ln)
    }
}

fn build_file_system(log: String) -> Node {
    let root = Rc::new(RefCell::new(Directory::new(
        ROOT_DIR_NAME.to_owned(),
        Weak::new(),
    )));
    let mut current_directory = Rc::downgrade(&root);
    // Skip first 'cd /'
    for ln in log.lines().skip(1) {
        current_directory = parse_line(current_directory, ln);
    }
    root
}

fn node_files_size(node: &Node) -> u64 {
    node.borrow()
        .files
        .iter()
        .fold(0, |acc, File { name: _, size }| *size + acc)
}

fn count_node_size(node: &Node) -> u64 {
    let mut size = node_files_size(node);
    for child in &node.borrow().children {
        size += count_node_size(child);
    }
    size
}

fn count_nodes_sizes(node: &Node, sizes: &mut HashMap<String, u64>) {
    let node_size = count_node_size(node);
    sizes.insert(node.borrow().name.clone(), node_size);
    for child in &node.borrow().children {
        count_nodes_sizes(child, sizes);
    }
}

fn get_sizes() -> HashMap<String, u64> {
    let fs = build_file_system(get_input());
    let mut sizes = HashMap::new();
    count_nodes_sizes(&fs, &mut sizes);
    sizes
}

fn solution_part_1() -> u64 {
    get_sizes()
        .iter()
        .filter_map(|(_, &size)| if size <= 100_000 { Some(size) } else { None })
        .sum()
}

fn solution_part_2() -> u64 {
    let sizes = get_sizes();
    let space_to_free = REQUIRED_SPACE - (TOTAL_SPACE - sizes[ROOT_DIR_NAME]);
    *sizes
        .iter()
        .filter(|(_, &size)| size >= space_to_free)
        .min_by(|(_, sa), (_, sb)| sa.cmp(sb))
        .unwrap()
        .1
}

#[cfg(test)]
mod tests {
    use crate::problem_7::*;

    #[test]
    fn problem_7_solution_part_1_test() {
        println!("problem 7 solution 1: {}", solution_part_1());
    }

    #[test]
    fn problem_7_solution_part_2_test() {
        println!("problem 7 solution 2: {}", solution_part_2());
    }
}
