use std::error::Error;
use aoc_2022::utils;

fn main() -> Result<(), Box<dyn Error>> {
    let input = utils::read()?
        .split('\n')
        .enumerate()
        .flat_map(|(row, str)| str
            .chars()
            .enumerate()
            .map(move |(col, c)| (row, col, c.to_digit(10).unwrap() as i8))
        )
        .collect::<Vec<_>>();
    let (rows, columns, _) = input.last().unwrap().clone();
    let (rows, columns) = (rows + 1, columns + 1);

    let height_at = |row: usize, col: usize| input.get(row * columns + col).unwrap().2;

    let count = input.iter()
        .filter(|(row, col, height)| {
            (0..*row).map(|x| height_at(x, *col)).max().unwrap_or(-1) < *height
            || (row+1..rows).map(|x| height_at(x, *col)).max().unwrap_or(-1) < *height
            || (0..*col).map(|x| height_at(*row, x)).max().unwrap_or(-1) < *height
            || (col+1..columns).map(|x| height_at(*row, x)).max().unwrap_or(-1) < *height
        })
        .count();

    println!("{}", count);

    Ok(())
}