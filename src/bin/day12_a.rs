use std::cell::Cell;
use std::error::Error;
use aoc_2022::utils;
use std::cmp::min;
use crate::GridSpace::{End, Other, Start};
use nalgebra_glm as glm;
use num_traits::Zero;
use glm::{IVec2, clamp_vec};

const CLEAR: &str = "\x1B[2J\x1B[H";
const DIRECTIONS: [IVec2; 4] = [IVec2::new(1, 0), IVec2::new(-1, 0), IVec2::new(0, 1), IVec2::new(0, -1)];

#[derive(Copy, Clone)]
enum GridSpace {
    Start,
    End,
    Other(i8)
}

impl GridSpace {
    fn height(&self) -> i8 {
        match self {
            Start => 0,
            End => 25,
            Other(x) => *x,
        }
    }
}

fn parse_height(c: char) -> GridSpace {
    match c {
        'S' => Start,
        'E' => End,
        c => Other((c as u32 - 'a' as u32) as i8)
    }
}

struct Space {
    pos: IVec2,
    height: GridSpace,
    distance: Cell<u32>
}

#[allow(dead_code)]
fn graph(input: &Vec<Space>) {
    print!("{}", CLEAR);
    for window in input.windows(2) {
        if let [cur, next] = window { //always true
            print!("{:2X}", (cur.distance.get() / 5).clamp(0, 0xFF));
            if cur.pos.x != next.pos.x {
                println!();
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = utils::read()?
        .split('\n')
        .enumerate()
        .flat_map(|(row, str)| { str
            .chars()
            .enumerate()
            .map(move |(col, c)| Space {
                pos: IVec2::new(row as i32, col as i32),
                height: parse_height(c),
                distance: if c == 'S' {Cell::new(0)} else {Cell::new(u32::MAX)}
            })
        })
        .collect::<Vec<_>>();
    let bounds = input.last().unwrap().pos + IVec2::new(1, 1);

    let at = |pos: IVec2| {
        if clamp_vec(&pos, &IVec2::zero(), &bounds) != pos { None }
        else { input.get((pos.x * bounds.y + pos.y) as usize) }
    };

    let path_len = [&input].iter()
        .cycle()
        .find(|vec| {
            for space in vec.iter() {
                space.distance.set(
                    min(
                        space.distance.get(),
                        DIRECTIONS.iter()
                            .flat_map(|x| at(space.pos + *x))
                            .filter(|x| x.height.height() - space.height.height() >= -1)
                            .map(|x| x.distance.get().saturating_add(1))
                            .min()
                            .unwrap_or(u32::MAX)
                    )
                )
            }
            graph(&input);
            u32::MAX != vec.iter()
                .find(|Space{ height, .. }| match height {
                    End => true,
                    _ => false
                })
                .unwrap()
                .distance.get()
        })
        .unwrap()
        .iter()
        .find(|Space{ height, .. }| match height {
            End => true,
            _ => false
        })
        .unwrap()
        .distance.get();

    println!("Path length: {}", path_len);

    Ok(())
}