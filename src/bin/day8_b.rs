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

    let max_scenic_score = input.iter()
        .map(|(row, col, height)| {
            let seen = |(count, blocked), x| (count + 1 - blocked as i32, blocked || x >= *height);
            (0..*row).rev()
                .map(|x| height_at(x, *col))
                .fold((0, false), seen).0
            * (row+1..rows)
                .map(|x| height_at(x, *col))
                .fold((0, false), seen).0
            * (0..*col).rev()
                .map(|x| height_at(*row, x))
                .fold((0, false), seen).0
            * (col+1..columns)
                .map(|x| height_at(*row, x))
                .fold((0, false), seen).0
        })
        .max().unwrap();

    println!("{}", max_scenic_score);

    Ok(())
}