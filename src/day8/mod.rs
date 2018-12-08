use std::str::FromStr;
use std::ops::Add;

pub fn step1(input : String) {
    let numbers = get_string_numbers(&input);
    let tree = parse_tree(&numbers).0;
    println!("{}", tree.sum_metadata());
}

pub fn step2(input : String) {
    let numbers = get_string_numbers(&input);
    let tree = parse_tree(&numbers).0;
    println!("{}", tree.value());
}

fn get_string_numbers(str : &str) -> Vec<usize> {
    str.split_whitespace().map(|s| usize::from_str(s).unwrap()).collect()
}

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn sum_metadata(&self) -> usize {
        self.children.iter().fold(0, |acc, child| acc + child.sum_metadata()) + self.metadata.iter().fold(0, usize::add)
    }

    fn value(&self) -> usize {
        if self.children.len() == 0 {
            self.metadata.iter().fold(0, usize::add)
        } else {
            self.metadata.iter().fold(0, |acc, &m| {
                if m == 0 {
                    acc
                } else {
                    match self.children.get(m - 1) {
                        None => acc,
                        Some(c) => acc + c.value(),
                    }
                }
            })
        }
    }
}

fn parse_tree(numbers: &[usize]) -> (Node, &[usize]) {
    let children_number = numbers[0];
    let metadata_number = numbers[1];
    let mut remaining = &numbers[2..];
    let mut children = Vec::new();
    for _i in 0..children_number {
        let (child, r) = parse_tree(remaining);
        children.push(child);
        remaining = r;
    }
    let metadata = Vec::from(&remaining[0..metadata_number]);
    let node = Node {
        children,
        metadata,
    };
    (node, &remaining[metadata_number..])
}