use std::collections::HashMap;
use std::error::Error;
use aoc_2022::utils;

use crate::Round::{Draw, Lose, Win};

enum Round {
    Lose, Draw, Win
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = utils::read()?;

    let score_key = HashMap::from([
        ('A', 0),
        ('B', 1),
        ('C', 2),
    ]);

    let round_key = HashMap::from([
        ('X', Lose),
        ('Y', Draw),
        ('Z', Win),
    ]);

    let score = input.trim()
        .split("\n")
        .map(|x| (
            score_key.get(&x.chars().nth(0).unwrap()).unwrap(),
            round_key.get(&x.chars().nth(2).unwrap()).unwrap()
        ))
        .map(|(opponent, round)| {
            1 + match round {
                Lose => {(opponent + 2) % 3}
                Draw => {3 + opponent}
                Win => {6 + (opponent + 1) % 3}
            }
        })
        .sum::<i32>();

    println!("The final player score is {}", score);

    Ok(())
}