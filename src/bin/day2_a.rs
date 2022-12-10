use std::collections::HashMap;
use std::error::Error;
use aoc_2022::utils;

fn main() -> Result<(), Box<dyn Error>> {
    let input = utils::read()?;

    let scoremap = HashMap::from([
        ('A', 1),
        ('B', 2),
        ('C', 3),
        ('X', 1),
        ('Y', 2),
        ('Z', 3)
    ]);

    let score = input.trim()
        .split("\n")
        .map(|x| {
            let opponent = scoremap.get(&x.chars().nth(0).unwrap()).unwrap();
            let player = scoremap.get(&x.chars().nth(2).unwrap()).unwrap();

            player
            + if opponent == player {3} else {0}
            + if (player - opponent + 3) % 3 == 1 {6} else {0}
        })
        .sum::<i32>();

    println!("The final player score is {}", score);

    Ok(())
}