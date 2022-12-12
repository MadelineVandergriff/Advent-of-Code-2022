#![feature(iterator_try_collect)]

use std::error::Error;
use aoc_2022::utils;

fn main() -> Result<(), Box<dyn Error>> {
    let result = utils::read_to_vec()?
        .iter()
        .map(|x| {
            x.split(',')
                .map(|x| x.split('-').map(str::parse::<i32>).try_collect::<Vec<_>>())
                .try_collect::<Vec<_>>().expect("failed to parse ints")
        })
        .map(|mut x| {
            for inner in x.as_mut_slice() {
                inner.sort();
            }

            let a = x.get(0).unwrap().get(0).unwrap();
            let b = x.get(0).unwrap().get(1).unwrap();
            let c = x.get(1).unwrap().get(0).unwrap();
            let d = x.get(1).unwrap().get(1).unwrap();

            (b >= c && a <= d) || (d >= a && c <= a)
        })
        .filter(|x| *x)
        .count();

    println!("{}", result);

    Ok(())
}