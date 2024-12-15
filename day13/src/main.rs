#![feature(unsigned_signed_diff)]

use regex::Regex;
use std::cmp::PartialEq;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::path::PathBuf;

fn main() {
    let _reader = read_input("puzzle-input.txt");
    let mut machines = Machines::from(_reader);
    println!("Used tokens: {}", machines.used_tokens());
    machines.machines.iter_mut().for_each(|m| {
        m.prize.x += 10000000000000;
        m.prize.y += 10000000000000;
    });
    println!("Used tokens part 2: {}", machines.used_tokens());
}
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct Button {
    x: isize,
    y: isize,
}
type Position = Button;
impl Add<Button> for Position {
    type Output = Position;
    fn add(self, other: Button) -> Position {
        Position{
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}
#[derive(Debug, Copy, Clone)]
struct ClawMachine {
    button_a: Button,
    button_b: Button,
    prize: Position,
}

impl ClawMachine {
    fn cheapest_combination(&mut self) -> Option<isize> {
        // solve for B
        let b_pressed = (self.button_a.y*self.prize.x - self.button_a.x*self.prize.y) /
            (self.button_a.y*self.button_b.x - self.button_a.x*self.button_b.y);
        // solve for A
        let a_pressed = (self.prize.x - self.button_b.x* b_pressed) / self.button_a.x;
        let end_pos = Position{x: b_pressed *self.button_b.x + a_pressed *self.button_a.x, y: b_pressed *self.button_b.y + a_pressed *self.button_a.y};
        if self.prize == end_pos{
            return Some(b_pressed *1+ a_pressed *3)
        }
        None
    }
}

struct Machines {
    machines: Vec<ClawMachine>,
}
impl Machines{
    pub fn used_tokens(&self) -> isize {
        self.machines.iter().cloned().filter_map(|mut m| m.cheapest_combination()).sum()
    }
}
impl<Reader> From<Reader> for Machines
where
    Reader: BufRead,
{
    fn from(mut value: Reader) -> Self {
        let mut string = String::new();
        value.read_to_string(&mut string).unwrap();
        let re = Regex::new(r"(Button A: X\+(?<ax>\d+), Y\+(?<ay>\d+)\nButton B: X\+(?<bx>\d+), Y\+(?<by>\d+)\nPrize: X=(?<px>\d+), Y=(?<py>\d+))").unwrap();
        Self {
            machines: re
                .captures_iter(&string)
                .map(|cap| ClawMachine {
                    button_a: Button {
                        x: cap["ax"].parse().unwrap(),
                        y: cap["ay"].parse().unwrap(),
                    },
                    button_b: Button {
                        x: cap["bx"].parse().unwrap(),
                        y: cap["by"].parse().unwrap(),
                    },
                    prize: Button {
                        x: cap["px"].parse::<isize>().unwrap(),
                        y: cap["py"].parse::<isize>().unwrap(),
                    },
                })
                .collect(),
        }
    }
}
fn read_input(name: &str) -> BufReader<File> {
    let mut example_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    example_data.push(format!("resources/{name}"));

    BufReader::new(File::open(example_data).unwrap())
}
#[cfg(test)]
mod tests {
    use crate::{read_input, Machines};

    #[test]
    fn test_part1() {
        let _reader = read_input("example-input.txt");
        let mut machines = Machines::from(_reader);
        assert_eq!(machines.machines[1].cheapest_combination(), None);
        assert_eq!(machines.machines[3].cheapest_combination(), None);
        assert_eq!(machines.machines[0].cheapest_combination(), Some(280));
        assert_eq!(machines.machines[2].cheapest_combination(), Some(200));
    }
    #[test]
    fn test_part2() {
        let _reader = read_input("example-input.txt");
        let _reader = read_input("example-input.txt");
        let mut machines = Machines::from(_reader);
        machines.machines.iter_mut().for_each(|m| {
            m.prize.x += 10000000000000;
            m.prize.y += 10000000000000;
        });
        assert_eq!(machines.machines[1].cheapest_combination(), Some(459236326669));
        assert_eq!(machines.machines[3].cheapest_combination(), Some(416082282239));
        assert_eq!(machines.machines[0].cheapest_combination(), None);
        assert_eq!(machines.machines[2].cheapest_combination(), None);

    }
}
