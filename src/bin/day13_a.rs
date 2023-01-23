use std::cmp::Ordering;
use std::error::Error;
use itertools::Itertools;
use aoc_2022::utils;
use crate::Packet::{Array, Int};

#[derive(Eq, PartialEq, Debug)]
enum Packet {
    Int(u32),
    Array(Vec<Packet>)
}

impl PartialOrd<Self> for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match (self, other) {
            (Int(l), Int(r)) => {
                l.cmp(r)
            }
            (Array(l), Array(r)) => {
                l.iter().cmp(r.iter())
            }
            (Int(l), Array(r)) => {
                [Int(*l)].iter().cmp(r.iter())
            }
            (Array(l), Int(r)) => {
                l.iter().cmp([Int(*r)].iter())
            }
        })
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

enum PacketBuildData {
    Int(u32),
    Array(usize)
}

struct PacketBuilder {
    data: Vec<Vec<PacketBuildData>>,
    ptr: Vec<usize>
}

impl PacketBuilder {
    fn new() -> Self {
        Self {
            data: vec![vec![]],
            ptr: vec![0]
        }
    }

    fn add_int(&mut self, int: u32) {
        self.data.get_mut(*self.ptr.last().unwrap()).unwrap().push(PacketBuildData::Int(int));
    }

    fn step_in(&mut self) {
        let new_ptr = self.data.len();
        self.data.get_mut(*self.ptr.last().unwrap()).unwrap().push(PacketBuildData::Array(new_ptr));
        self.ptr.push(new_ptr);
        self.data.push(vec![]);
    }

    fn step_out(&mut self) {
        self.ptr.pop();
    }

    fn build(self) -> Packet {
        fn build_helper(_self: &PacketBuilder, ptr: usize) -> Packet {
            Array(_self.data.get(ptr).unwrap().iter().map(|x| match x {
                PacketBuildData::Int(x) => Int(*x),
                PacketBuildData::Array(new_ptr) => build_helper(_self, *new_ptr)
            }).collect())
        }

        build_helper(&self, 0)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let sum = utils::read()?
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.chars().skip(1)
            .fold((PacketBuilder::new(), None), |(mut pb, mut int), c| {
                match c {
                    '[' => pb.step_in(),
                    ']' => {
                        if let Some(int) = int {
                            pb.add_int(int);
                        }
                        int = None;
                        pb.step_out()
                    }
                    ',' => {
                        if let Some(int) = int {
                            pb.add_int(int);
                        }
                        int = None;
                    }
                    x => int = Some(int.unwrap_or(0) * 10 + x.to_digit(10).unwrap())
                }
                (pb, int)
            }).0.build()
        })
        .chunks(2).into_iter()
        .map(|mut chunk| [chunk.next().unwrap(), chunk.next().unwrap()])
        .enumerate()
        .filter_map(|(idx, [l, r])| if l < r { Some(idx + 1) } else { None })
        .sum::<usize>();

    println!("Sum: {}", sum);

    Ok(())
}