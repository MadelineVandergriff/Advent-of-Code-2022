use std::cell::RefCell;
use std::error::Error;
use aoc_2022::utils;
use regex::Regex;

// Handcoded, but now we can always do calculations moldulo this
const LCM: u64 = 2 * 7 * 13 * 19 * 11 * 5 * 3 * 17;

struct Monkey {
    inspections: RefCell<u64>,
    items: RefCell<Vec<u64>>,
    op: Box<dyn Fn(u64) -> (u64, usize)> // BigUint: new, usize: monkey index
}

enum MonkeyOperations {
    Add(u64),
    Multiply(u64),
    Square
}

#[derive(Default)]
struct MonkeyBuilder {
    id: Option<usize>,
    starting_items: Option<Vec<u64>>,
    operation: Option<MonkeyOperations>,
    test: Option<u64>,
    left: Option<usize>,
    right: Option<usize>
}

impl MonkeyBuilder {
    fn new() -> MonkeyBuilder {
        Default::default()
    }

    fn build(self) -> Monkey {
        let op = self.operation.unwrap();
        let test = self.test.unwrap();
        let left = self.left.unwrap();
        let right = self.right.unwrap();
        Monkey {
            inspections: RefCell::new(0),
            items: RefCell::new(self.starting_items.unwrap()),
            op: match op {
                MonkeyOperations::Add(x) => Box::new(move |old| {let new = (old + x) % LCM; if new % test == 0 {(new, left)} else {(new, right)}}),
                MonkeyOperations::Multiply(x) => Box::new(move |old| {let new = (old * x) % LCM; if new % test == 0 {(new, left)} else {(new, right)}}),
                MonkeyOperations::Square => Box::new(move |old| {let new = (old * old) % LCM; if new % test == 0 {(new, left)} else {(new, right)}})
            },
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let regex = Regex::new(r#"(?i)(monkey)\s+(\d+):|(starting items): ((?:\d+[, ]*)+)|(operation): new = old (\+ \d+|\* \d+|\* old)|(test): divisible by (\d+)|if (true): throw to monkey (\d+)|if (false): throw to monkey (\d+)"#)?;

    let monkeys = utils::read()?
    .split("\n\n")
    .map(|x| {
        let mut mb = MonkeyBuilder::new();
        x.split('\n')
        .for_each(|x| {
            let captures = regex.captures(x).map(|x| {
                x.iter()
                .skip(1)
                .filter_map(|c| c)
                .map(|m| m.as_str())
                .collect::<Vec<_>>()
            });
            match captures.as_ref().map(|x| x.as_slice()) {
                Some(["Monkey", x]) => {
                    mb.id = Some(x.parse().unwrap());
                }
                Some(["Starting items", x]) => {
                    mb.starting_items = Some(x.split(", ").map(|x| x.parse().unwrap()).collect());
                }
                Some(["Operation", x]) => {
                    mb.operation = Some(match *x {
                        "* old" => MonkeyOperations::Square,
                        s if s.starts_with('*') => MonkeyOperations::Multiply(s[2..].parse().unwrap()),
                        s if s.starts_with('+') => MonkeyOperations::Add(s[2..].parse().unwrap()),
                        _ => panic!("unacceptable operation")
                    });
                }
                Some(["Test", x]) => {
                    mb.test = Some(x.parse().unwrap());
                }
                Some(["true", x]) => {
                    mb.left = Some(x.parse().unwrap());
                }
                Some(["false", x]) => {
                    mb.right = Some(x.parse().unwrap());
                }
                _ => ()
            }
        });
        mb.build()
    })
    .collect::<Vec<_>>();

    for iter in 0..10_000 {
        for monkey in &monkeys {
            *monkey.inspections.borrow_mut() += monkey.items.borrow().len() as u64;
            monkey.items.take().iter()
                .map(|x| (monkey.op)(x.clone()))
                .for_each(|(new, id)| monkeys.get(id).unwrap().items.borrow_mut().push(new))
        }
        if iter % 100 == 0 {
            println!("iteration: {}", iter);
        }
    }

    let scores = monkeys.iter()
        .map(|x| x.inspections.take())
        .fold((1, 1), |(l, r), i| {
            if i < r {(l, r)}
            else if i < l {(l, i)}
            else {(i, l)}
        });

    println!("{}", scores.0 * scores.1);
    Ok(())
}