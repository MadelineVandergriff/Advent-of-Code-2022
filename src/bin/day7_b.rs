use std::cell::RefCell;
use std::error::Error;
use std::rc::{Rc, Weak};
use aoc_2022::{tree::Tree, utils};
use aoc_2022::tree::TreePointer;
use crate::Node::{File, Directory};

#[derive(PartialEq, Debug)]
enum Node {
    File {
        name: String,
        size: usize
    },
    Directory {
        name: String,
        size: Option<usize>
    }
}

impl Node {
    pub fn is_named(&self, x: &str) -> bool {
        match self {
            File{ name, .. } => name == x,
            Directory { name, .. } => name == x
        }
    }
}

fn tree_size(tree: Rc<RefCell<Tree<Node>>>) -> usize {
    let mut tree = tree.borrow_mut();
    if let Directory {size: None, name } = &tree.node {
        let name = name.clone();
        let size = tree.children
            .iter()
            .map(|x| tree_size(x.clone()))
            .sum();
        tree.node = Directory {size: Some(size), name};
        size
    } else {
        match &tree.node {
            File { size, .. } => {*size}
            Directory { size, .. } => {size.unwrap()}
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let tree = Tree::new(Directory {name: String::from("/"), size: None});
    let mut directory = tree.clone();

    let mut input = utils::read_to_vec()?;
    input.remove(0);
    for line in input {
        match line.as_str() {
            "$ cd .." => {
                directory = directory.parent().unwrap();
            }
            s if {s.starts_with("$ cd")} => {
                let s = &s[5..];
                directory = directory.get_child(|x| {
                    match x {
                        File { .. } => false,
                        Directory { name, ..} => {name == s}
                    }
                }).unwrap();
            },
            "$ ls" => { continue },
            s if {s.starts_with("dir")} => {
                let s = &s[4..];
                directory.add_child(Directory {name: String::from(s), size: None}).unwrap();
            },
            s=> {
                let (size, name) = s.split_once(' ').unwrap();
                directory.add_child(File {name: String::from(name), size: size.parse().unwrap()}).unwrap();
            }
        }
    }

    let used = tree_size(tree.clone());
    let viable = |size: usize| used - size <= 40_000_000;

    let min = tree.iter()
        .filter_map(|x| match &x.upgrade().unwrap().borrow().node {
            Directory { size: Some(size), .. } if viable(*size) => Some(*size),
            _ => None
        })
        .min()
        .unwrap();
    println!("{}", min);

    Ok(())
}