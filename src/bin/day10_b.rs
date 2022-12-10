use std::error::Error;
use aoc_2022::utils;
use crate::Instruction::{AddX, Noop};

enum Instruction {
    Noop,
    AddX(i32)
}

fn main() -> Result<(), Box<dyn Error>> {
    let instructions: Vec<_> = utils::read_to_vec()?
        .iter()
        .flat_map(|x|{
            if x.contains("noop") { vec![Noop] }
            else { vec![Noop, AddX(*&x[5..].parse::<i32>().unwrap())]}
        })
        .collect();

    let mut cycle = 1;
    let mut register = 1;
    let mut x = 0i32;

    let mut crt = String::new();
    for inst in instructions {
        if (register - x).abs() <= 1 {
            crt.push('#');
        } else {
            crt.push('.');
        }

        match inst {
            Noop => {}
            AddX(v) => {register += v;}
        }

        x += 1;

        if cycle % 40 == 0 {
            crt.push('\n');
            x = 0;
        }

        cycle += 1;
    }

    print!("{}",crt);

    Ok(())
}