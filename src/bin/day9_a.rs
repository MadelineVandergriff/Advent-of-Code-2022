use std::collections::HashSet;
use std::error::Error;
use aoc_2022::utils;

#[derive(Default)]
struct Rope {
    head: [i32; 2],
    tail: [i32; 2]
}

impl Rope {
    fn update_head(&mut self, diff: &[i32; 2]) {
        self.head = [self.head[0] + diff[0], self.head[1] + diff[1]];

        self.update();
    }

    fn update(&mut self) {
        let diff = [self.head[0] - self.tail[0], self.head[1] - self.tail[1]];
        if diff[0].abs() < 2 && diff[1].abs() < 2 {return;}
        if diff[0] == 0 {
            println!("a");
            self.tail[1] += diff[1] / 2;
        } else if diff[1] == 0 {
            println!("b");
            self.tail[0] += diff[0] / 2;
        } else {
            println!("c");
            self.tail[0] += diff[0].clamp(-1, 1);
            self.tail[1] += diff[1].clamp(-1, 1);
        }
    }
}

#[derive(Default)]
struct RopeChain {
    ropes: [Rope; 9]
}

impl RopeChain {
    fn update_head(&mut self, diff: &[i32; 2]) {
        {
            let mut head = &mut self.ropes[0].head;
            head[0] = head[0] + diff[0];
            head[1] = head[1] + diff[1];
        }
        let mut prev = self.ropes[0].head.clone();
        for rope in &mut self.ropes {
            rope.head = prev;
            rope.update();
            prev = rope.tail;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rope: RopeChain = Default::default();
    let mut tail_hash = HashSet::new();

    let input = utils::read()?;
    for line in input.trim().split('\n') {
        for _ in 0..(line[2..].parse()?) {
            rope.update_head(&match &line[0..1] {
                "L" => Ok([-1, 0]),
                "R" => Ok([1, 0]),
                "U" => Ok([0, 1]),
                "D" => Ok([0, -1]),
                _ => Err("NOOOOO")
            }?);
            tail_hash.insert(rope.ropes[8].tail.clone());
        }
    }
    println!("{}", tail_hash.len());
    Ok(())
}