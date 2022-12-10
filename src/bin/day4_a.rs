use std::error::Error;
use aoc_2022::utils;

fn main() -> Result<(), Box<dyn Error>> {
    let result = utils::read_to_vec()?
        .iter()
        .map(|x| {
            x.split(',')
                .map(|x| x.split('-').collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        ;

    Ok(())
}