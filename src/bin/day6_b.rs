use std::error::Error;
use aoc_2022::utils;

const PACKET_SIZE: usize = 14;

fn main() -> Result<(), Box<dyn Error>> {
    let packet_comparator = (0..PACKET_SIZE)
        .flat_map(|x| (x+1..PACKET_SIZE).zip((x..x+1).cycle()))
        .collect::<Vec<_>>();

    let result = utils::read()?
        .chars()
        .collect::<Vec<_>>()
        .windows(PACKET_SIZE)
        .enumerate()
        .find_map(|(i, x)| {
            for (l, r) in &packet_comparator {
                if x[*l] == x[*r] { return None }
            }
            Some(i)
        })
        .unwrap();

    println!("{}", result + PACKET_SIZE);

    Ok(())
}