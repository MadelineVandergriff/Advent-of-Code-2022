use std::error::Error;
use aoc_2022::utils;
use crate::Node::{File, Directory};

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
    let mut tree = utils::Tree::<Node>::new(Directory {name: String::from("/")});
    let mut path = vec![0];

    let input = utils::read_to_vec()?.leak();
    for line in input {
        match line.as_str() {
            s if {s.starts_with("$ cd")} => {
                let name = &s[5..];
                if name == "/" {
                    path.clear();
                    path.push(0usize);
                } else if name == ".." {
                    path.pop();
                } else {
                    let idx = tree
                        .find_by(|x| x.is_named(name))
                        .unwrap();
                    path.push(idx);
                }
            },
            s if {s.starts_with('$')} => {},
            s if {s.starts_with("dir")} => {
                let name = &s[4..];
                tree.append(
                    *path.last().unwrap(),
                    Directory {name: String::from(name)}
                ).unwrap();
            },
            s=> {
                let (size, name) = s.split_once(' ').unwrap();

            }
        }
    }

    Ok(())
}