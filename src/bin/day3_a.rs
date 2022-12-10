use std::collections::HashSet;
use std::error::Error;
use aoc_2022::utils;

fn main() -> Result<(), Box<dyn Error>> {
    let input = utils::read()?;
    let lower_a = 'a' as u32;
    let upper_a = 'A' as u32;

    let result = utils::map_over_lines(input.as_str(), |x|{
        let (left, right) = x.split_at(x.len() / 2);
        let left = left.chars().collect::<HashSet<_>>();
        let right = right.chars().collect::<HashSet<_>>();

        let item = *left.intersection(&right).nth(0).unwrap() as u32;
        if item >= lower_a {item - lower_a + 1} else {item - upper_a + 27}
    }).sum::<u32>();

    println!("The total priority is {}", result);
    Ok(())
}