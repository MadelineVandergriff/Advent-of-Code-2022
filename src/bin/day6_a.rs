use std::error::Error;
use aoc_2022::utils;

fn main() -> Result<(), Box<dyn Error>> {
    let result = utils::read()?
        .chars()
        .collect::<Vec<_>>()
        .windows(4)
        .enumerate()
        .find_map(|(i, x)| {
            for (l, r) in [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)] {
                if x[l] == x[r] { return None }
            }
            Some(i)
        })
        .unwrap();

    println!("{}", result + 4);

    Ok(())
}