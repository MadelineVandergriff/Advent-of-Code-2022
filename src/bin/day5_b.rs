#![feature(string_leak)]
#![feature(get_many_mut)]

use std::error::Error;
use aoc_2022::utils;

fn main() -> Result<(), Box<dyn Error>> {
    let (crates, moves)
        = utils::read()?
        .leak()
        .split_once("\n\n")
        .ok_or("could not process input")?;

    let num_stacks = (crates.lines().rev().next().unwrap().len() + 1) / 4;
    let mut stacks = vec![Vec::<char>::new(); num_stacks];

    crates
        .lines()
        .rev()
        .skip(1)
        .map(str::chars)
        .for_each(|mut x| {
            x.next();
            for i in 0..num_stacks {
                let c = x.next().unwrap();
                if c.is_alphabetic() {
                    stacks.get_mut(i).unwrap().push(c);
                }
                x.nth(2);
            }
        });

    moves
        .lines()
        .for_each(|x| {
            let (_, c) = x.split_once("move ").unwrap();
            let (c, s1) = c.split_once(" from ").unwrap();
            let (s1, s2) = s1.split_once(" to ").unwrap();

            let c = c.parse::<usize>().unwrap();
            let s1 = s1.parse::<usize>().unwrap() - 1;
            let s2 = s2.parse::<usize>().unwrap() - 1;

            let [s1, s2] = stacks.get_many_mut([s1, s2]).unwrap();
            let crates = s1.drain((s1.len() - c)..);
            s2.extend(crates);
        });

    for mut stack in stacks {
        print!("{}", stack.pop().unwrap())
    }
    println!();

    Ok(())
}