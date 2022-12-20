use std::cell::RefCell;
use std::error::Error;
use aoc_2022::{tree::Tree, utils};
use crate::Node::{File, Directory};

#[derive(PartialEq)]
enum Node {
    File {
        name: String,
        size: usize
    },
    Directory {
        name: String,
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

fn main() -> Result<(), Box<dyn Error>> {
    let mut tree = Tree {
        node: Directory {name: String::from("/")},
        children: vec![]
    };

    let mut current_leaf = &mut tree;

    let input = utils::read_to_vec()?.leak();
    for line in input {
        match line.as_str() {
            "$ cd .." => {
                //current_leaf = current_leaf.parent(&tree).unwrap();
            }
            s if {s.starts_with("$ cd")} => {
                let s = &s[5..];
                current_leaf.children.push(Tree {
                    node: Directory {name: String::from(s)},
                    children: vec![],
                });
            },
            "$ ls" => { continue },
            s if {s.starts_with("dir")} => {

            },
            s=> {
                let (size, name) = s.split_once(' ').unwrap();

            }
        }
    }

    Ok(())
}