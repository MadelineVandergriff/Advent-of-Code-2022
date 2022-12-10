#![feature(iter_array_chunks)]

use std::collections::hash_set::Intersection;
use std::collections::HashSet;
use std::error::Error;
use aoc_2022::utils;

fn main() -> Result<(), Box<dyn Error>> {
    let input = utils::read()?;
    let lower_a = 'a' as u32;
    let upper_a = 'A' as u32;

    let result = input.trim()
        .split('\n')
        .array_chunks::<3>()
        .map(|chunk| {
            *chunk
                .iter()
                .map(|x| x.chars().collect::<HashSet<_>>())
                .reduce(|mut acc, x| {acc.retain(|val| x.contains(val)); acc})
                .unwrap()
                .iter()
                .nth(0).unwrap() as u32
        })
        .map(|x| if x >= lower_a {x - lower_a + 1} else {x - upper_a + 27})
        .sum::<u32>();

    println!("The total priority is {}", result);
    Ok(())
}