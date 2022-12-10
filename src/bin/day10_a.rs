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
    let mut signal = 0;

    for inst in instructions {
        if cycle % 40 == 20 {
            signal += register * cycle;
        }

        match inst {
            Noop => {}
            AddX(v) => {register += v;}
        }

        cycle += 1;
    }

    print!("{}", signal);

    Ok(())
}