use std::cell::Cell;
use std::cmp::{max, Ordering};
use std::collections::HashSet;
use std::error::Error;
use nalgebra_glm as glm;
use glm::IVec2;
use aoc_2022::utils;
use itertools::{Itertools, MinMaxResult, FoldWhile::{Done, Continue}};
use num_traits::Zero;
use crate::State::Falling;

const CLEAR: &str = "\x1B[2J\x1B[H";

enum State {
    Falling, Rest
}

fn graph(old: &HashSet<IVec2>, input: &HashSet<IVec2>, xbounds: (i32, i32), ybounds: (i32, i32)) {
    let left = format!("{:X}", xbounds.0);
    let right = format!("{:X}", xbounds.1);
    let lr_len = max(left.len(), right.len());

    print!("{}", CLEAR);

    for y in 0..lr_len {
        print!("      ");
        for x in xbounds.0 ..= xbounds.1 {
            if x == xbounds.0 {
                match left.chars().nth((y + left.len()).wrapping_sub(lr_len)) {
                    None => print!(" "),
                    Some(c) => print!("{}", c)
                }
            } else if x == xbounds.1 {
                match right.chars().nth((y + right.len()).wrapping_sub(lr_len)) {
                    None => print!(" "),
                    Some(c) => print!("{}", c)
                }
            } else {
                print!(" ");
            }
        }
        println!();
    }

    for y in ybounds.0 ..= ybounds.1 {
        print!("{:#5X} ", y);
        for x in xbounds.0 ..= xbounds.1 {
            if (x, y) == (500, 0) {
                print!("+");
            } else if old.contains(&IVec2::new(x, y)) {
                print!("#");
            } else if input.contains(&IVec2::new(x, y)) {
                print!("o");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut coords = utils::read()?
        .lines()
        .flat_map(|line| {
            line.split(" -> ")
                .map(|pair| {
                    let (x, y) = pair
                        .split(',')
                        .map(|x| x.parse().unwrap())
                        .collect_tuple()
                        .unwrap();
                    IVec2::new(x, y)
                })
                .collect_vec()
                .windows(2)
                .flat_map(|coords| {
                    (coords[0].x ..= coords[1].x)
                        .chain(coords[1].x ..= coords[0].x)
                        .cartesian_product(
                            (coords[0].y ..= coords[1].y)
                                .chain(coords[1].y ..= coords[0].y)
                        )
                        .map(|(x, y)| IVec2::new(x, y))
                })
                .collect_vec()
                .into_iter()
        }).collect::<HashSet<_>>();

    let xbounds = match coords.iter().map(|coord| coord.x).minmax() {
        MinMaxResult::MinMax(min, max) => (min, max),
        _ => panic!("No minmax found")
    };

    let ybounds = match coords.iter().map(|coord| coord.y).chain([0].into_iter()).minmax() {
        MinMaxResult::MinMax(min, max) => (min, max),
        _ => panic!("No minmax found")
    };

    let old = coords.clone();

    let lower_bound = ybounds.1;
    let fall_directions = [IVec2::new(0, 1), IVec2::new(-1, 1), IVec2::new(1, 1)];

    let idx = (0..)
        .position(|_| {
            let pos = (0..).fold_while(Ok(IVec2::new(500, 0)), |pos, _| {
                match pos {
                    Ok(pos) if pos.y > lower_bound => Done(Err(pos)),
                    Ok(pos) => Continue(fall_directions.iter()
                        .find(|dir| !coords.contains(&(pos + *dir)))
                        .map(|dir| pos + dir)
                        .ok_or(pos)),
                    err => Done(err)
                }
            }).into_inner().unwrap_err();
            coords.insert(pos);
            graph(&old, &coords, xbounds, ybounds);
            //std::thread::sleep(std::time::Duration::from_millis(500));
            pos.y > lower_bound
        }).unwrap();

    println!("{}", idx);

    Ok(())
}