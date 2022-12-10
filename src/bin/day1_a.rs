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
        .max()
        .unwrap();

    println!("The elf with the most calories has {} calories", result);
    Ok(())
}