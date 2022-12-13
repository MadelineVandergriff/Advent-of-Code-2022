use std::error::Error;
use std::io::{BufRead, stdin};

pub fn read() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    for line in stdin().lock().lines() {
        let line = line?;
        if line.contains("~") {
            break;
        }

        input.push_str(line.as_str());
        input.push('\n');
    }
    Ok(input)
}

pub fn map_over_lines<'a, B, F: 'a + FnMut(&str) -> B> (lines: &'a str, f: F) -> impl DoubleEndedIterator<Item = B> + 'a {
    lines.trim().split('\n').map(f)
}

pub fn read_to_vec() -> Result<Vec<String>, Box<dyn Error>>{
    let mut input = Vec::new();
    for line in stdin().lock().lines() {
        let line = line?;
        if line.contains("~") {
            break;
        }

        input.push(line);
    }

    Ok(input)
}

pub struct Tree<T> {
    arena: Vec<(T, Vec<usize>)>
}

impl<T> Tree<T> {
    pub fn new(root: T) -> Self {
        Self {
            arena: vec![(root, Vec::new())]
        }
    }

    pub fn depth(&self, idx: usize) -> Option<usize> {
        self.arena.get(idx).and_then(|(_, children)| {
            children.iter().filter_map(|x| self.depth(*x)).max()
        })
    }

    pub fn find_by<P: Fn(&T) -> bool>(&self, predicate: P) -> Option<usize> {
        self.arena
            .iter()
            .enumerate()
            .find_map(|(idx, (x, _))| if predicate(x) { Some(idx) } else { None })
    }

    pub fn append(&mut self, parent: usize, node: T) -> Option<usize> {
        if self.arena.get(parent).is_none() {
            return None;
        }

        let idx = self.arena.len();
        self.arena.push((node, Vec::new()));
        self.arena.get_mut(parent).unwrap().1.push(idx);
        Some(idx)
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        self.arena.get(idx).and_then(|(x, _)| Some(x))
    }
}

impl<T: PartialEq> Tree<T> {
    pub fn find(&self, other: &T) -> Option<usize> {
        self.arena
            .iter()
            .enumerate()
            .find_map(|(idx, (x, _))| if x == other { Some(idx) } else { None })
    }
}