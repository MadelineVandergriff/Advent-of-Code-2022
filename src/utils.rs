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