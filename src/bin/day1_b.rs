use std::error::Error;
use aoc_2022::utils;

fn main() -> Result<(), Box<dyn Error>> {
    let input = utils::read()?;
    let result = input
        .split("\n\n")
        .map(|s| {
            s.split("\n")
                .filter_map(|s| s.parse::<u32>().ok())
                .sum::<u32>()
        })
        .fold((0, 0, 0), |acc, x| {
            if x < acc.2 { acc }
            else if x < acc.1 { (acc.0, acc.1, x) }
            else if x < acc.0 { (acc.0, x, acc.1) }
            else { (x, acc.0, acc.1) }
        });
    let result = result.0 + result.1 + result.2;

    println!("The 3 elves with the most calories have {} calories", result);
    Ok(())
}