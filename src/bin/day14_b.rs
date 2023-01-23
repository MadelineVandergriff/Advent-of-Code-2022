use std::cell::Cell;
use std::cmp::{max, Ordering};
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use nalgebra_glm as glm;
use glm::IVec2;
use aoc_2022::utils;
use itertools::{Itertools, MinMaxResult, FoldWhile::{Done, Continue}};
use num_traits::Zero;
use std::os::unix::io::FromRawFd;
use std::io::Write;
use std::mem;
use std::thread::sleep;
use std::time::Duration;
use crate::State::Falling;

const GRAPH_FREQ: i32 = 0;
const CLEAR: &str = "\x1B[J";
const HOME: &str = "\x1B[H";

enum State {
    Falling, Rest
}

fn graph(old: &HashSet<IVec2>, input: &HashSet<IVec2>, xbounds: (i32, i32), ybounds: (i32, i32)) {
    let mut str = vec![];

    let left = format!("{:X}", xbounds.0);
    let right = format!("{:X}", xbounds.1);
    let lr_len = max(left.len(), right.len());

    write!(str, "{}", HOME).unwrap();

    for y in 0..lr_len {
        write!(str, "      ").unwrap();
        for x in xbounds.0 ..= xbounds.1 {
            if x == xbounds.0 {
                match left.chars().nth((y + left.len()).wrapping_sub(lr_len)) {
                    None => write!(str, " ").unwrap(),
                    Some(c) => write!(str, "{}", c).unwrap()
                }
            } else if x == xbounds.1 {
                match right.chars().nth((y + right.len()).wrapping_sub(lr_len)) {
                    None => write!(str, " ").unwrap(),
                    Some(c) => write!(str, "{}", c).unwrap()
                }
            } else {
                write!(str, " ").unwrap();
            }
        }
        writeln!(str).unwrap();
    }

    for y in ybounds.0 ..= ybounds.1 {
        write!(str, "{:#5X} ", y).unwrap();
        for x in xbounds.0 ..= xbounds.1 {
            if (x, y) == (500, 0) {
                write!(str, "+").unwrap();
            } else if old.contains(&IVec2::new(x, y)) {
                write!(str, "#").unwrap();
            } else if input.contains(&IVec2::new(x, y)) {
                write!(str, "o").unwrap();
            } else {
                write!(str, ".").unwrap();
            }
        }
        writeln!(str).unwrap();
    }

    let mut lock = unsafe {File::from_raw_fd(1)};
    write!(lock, "{}", String::from_utf8(str).unwrap()).unwrap();
    lock.flush().unwrap();
    mem::forget(lock);
    //sleep(Duration::from_millis(10));
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

    let ybounds = match coords.iter().map(|coord| coord.y).chain([0].into_iter()).minmax() {
        MinMaxResult::MinMax(min, max) => (min, max + 2),
        _ => panic!("No minmax found")
    };

    let xbounds = match coords.iter().map(|coord| coord.x).minmax() {
        MinMaxResult::MinMax(min, max) => (min - ybounds.1, max + ybounds.1),
        _ => panic!("No minmax found")
    };

    coords.extend(
        (xbounds.0 ..= xbounds.1)
            .map(|x| IVec2::new(x, ybounds.1))
    );

    let old = coords.clone();

    let lower_bound = ybounds.1;
    let fall_directions = [IVec2::new(0, 1), IVec2::new(-1, 1), IVec2::new(1, 1)];

    println!("{}{}{}", CLEAR, HOME, CLEAR);
    let idx = 1 + (0..) //gotta account for 0-indexing
        .position(|idx| {
            let pos = (0..).fold_while(Ok(IVec2::new(500, 0)), |pos, _| {
                match pos {
                    Ok(pos) => Continue(fall_directions.iter()
                        .find(|dir| !coords.contains(&(pos + *dir)))
                        .map(|dir| pos + dir)
                        .ok_or(pos)),
                    err => Done(err)
                }
            }).into_inner().unwrap_err();
            coords.insert(pos);
            if GRAPH_FREQ >= 0 && (idx % (GRAPH_FREQ + 1)) == 0 {
                graph(&old, &coords, xbounds, ybounds);
            }
            pos == IVec2::new(500 ,0)
        }).unwrap();

    if GRAPH_FREQ >= 0 {
        graph(&old, &coords, xbounds, ybounds);
    }
    println!("{}", idx);

    Ok(())
}