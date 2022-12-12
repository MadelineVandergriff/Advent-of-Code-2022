#![feature(string_leak)]

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

            for _ in 0..c {
                let ch = stacks.get_mut(s1).unwrap().pop().unwrap();
                stacks.get_mut(s2).unwrap().push(ch);
            }
        });

    for mut stack in stacks {
        print!("{}", stack.pop().unwrap())
    }
    println!();

    Ok(())
}