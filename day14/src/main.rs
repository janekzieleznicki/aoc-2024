#![feature(unsigned_signed_diff)]

use regex::Regex;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, AddAssign, Range};
use std::path::PathBuf;

fn main() {
    let reader = read_input("puzzle-input.txt");
    let mut ebhq = BathroomSecurity::from(reader).with_size(Position { x: 101, y: 103 });
    for i in 0..100 {
        // println!("After {i} seconds:\n{ebhq}");
        ebhq = ebhq.next();
        if ebhq.is_christmas_tree() {
            println!("After {i} seconds:\n{ebhq}");
        }
    }
    // println!("{}", ebhq);
    println!("Safety factor after 100 seconds: {}", ebhq.safety_factor());
    for i in 101.. {
        ebhq = ebhq.next();
        if ebhq.is_christmas_tree() {
            println!("After {i} seconds:\n{ebhq}");
            break;
        }
        println!("Iteration {i}");
    }
}
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Copy)]
struct Position {
    x: isize,
    y: isize,
}
type Velocity = Position;
struct Robot {
    position: Position,
    velocity: Velocity,
}
impl Robot {
    pub fn move_with_teleport(&mut self, bound: &Position) {
        self.position += self.velocity;
        if self.position.x < 0 {
            self.position.x = bound.x + self.position.x
        }
        if self.position.x >= bound.x {
            self.position.x = self.position.x - bound.x
        }
        if self.position.y < 0 {
            self.position.y = bound.y + self.position.y
        }
        if self.position.y >= bound.y {
            self.position.y = self.position.y - bound.y
        }
    }
}
struct BathroomSecurity {
    robots: Vec<Robot>,
    size: Option<Position>,
}

impl AddAssign<Velocity> for Position {
    fn add_assign(&mut self, rhs: Velocity) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
#[derive(Debug)]
struct Quadrant {
    x: Range<usize>,
    y: Range<usize>,
}
impl Quadrant {
    pub fn new(x: Range<usize>, y: Range<usize>) -> Quadrant {
        Quadrant { x, y }
    }
    pub fn contains(&self, pos: &Position) -> bool {
        self.x.contains(&(pos.x as usize)) && self.y.contains(&(pos.y as usize))
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl BathroomSecurity {
    pub fn with_size(mut self, size: Position) -> BathroomSecurity {
        self.size = Some(size);
        self
    }
    pub fn safety_factor(&self) -> usize {
        let max_x = self.size.unwrap().x as usize;
        let max_y = self.size.unwrap().y as usize;
        let quadrants = vec![
            Quadrant::new(0..(max_x / 2), 0..(max_y / 2)),
            Quadrant::new((max_x / 2 + 1)..max_x, 0..(max_y / 2)),
            Quadrant::new(0..(max_x / 2), (max_y / 2 + 1)..max_y),
            Quadrant::new((max_x / 2 + 1)..max_x, (max_y / 2 + 1)..max_y),
        ];
        // println!("{:?}", quadrants);
        quadrants
            .iter()
            .map(|quadrant| {
                self.robots
                    .iter()
                    .filter(|robot| quadrant.contains(&robot.position))
                    .count()
            })
            .fold(1, |acc, count| acc * count)
    }
    fn quadrant_count(&self, quadrant: &Quadrant) -> usize {
        self.robots
            .iter()
            .filter(|robot| quadrant.contains(&robot.position))
            .count()
    }

    pub fn is_christmas_tree(&self) -> bool {
        // Find friendly robot
        let map_quadrant = Quadrant::new(
            0..self.size.unwrap().x as usize,
            0..self.size.unwrap().y as usize,
        );
        for robot in &self.robots {
            let neighbors = vec![
                Velocity { x: -1, y: -1 },
                Velocity { x: -1, y: 0 },
                Velocity { x: -1, y: 1 },
                Velocity { x: 0, y: -1 },
                Velocity { x: 0, y: 0 },
                Velocity { x: 0, y: 1 },
                Velocity { x: 1, y: -1 },
                Velocity { x: 1, y: 0 },
                Velocity { x: 1, y: 1 },
            ]
            .into_iter()
            .map(|vel| robot.position + vel)
            .filter(|pos| map_quadrant.contains(pos))
            .collect::<Vec<_>>();
            if neighbors.len() < 9 {
                break;
            }
            if neighbors.iter().all(|neighbor| {
                self.robots
                    .iter()
                    .find(|robot| robot.position == *neighbor)
                    .is_some()
            }) {
                return true;
            }
        }
        false
    }
    pub fn next(mut self) -> Self {
        let bound = self.size.unwrap();
        for robot in &mut self.robots {
            robot.move_with_teleport(&bound)
        }
        self
    }

    fn robot_counts(&self) -> HashMap<Position, i32> {
        let mut robot_counts = HashMap::new();
        for robot in self.robots.iter() {
            *robot_counts.entry(robot.position).or_insert(0) += 1;
        }
        robot_counts
    }
}
impl Display for BathroomSecurity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let robot_counts = self.robot_counts();
        for y in 0..self.size.unwrap().y {
            for x in 0..self.size.unwrap().x {
                match robot_counts.get(&Position { x, y }) {
                    None => write!(f, ".")?,
                    Some(count) => write!(f, "{}", count)?,
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
impl<Reader> From<Reader> for BathroomSecurity
where
    Reader: BufRead,
{
    fn from(mut reader: Reader) -> Self {
        let reg = Regex::new(r"^p=(?<px>\d+),(?<py>\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)$").unwrap();
        BathroomSecurity {
            robots: reader
                .lines()
                .map_while(|line| line.ok())
                .map(|line| match reg.captures(&line) {
                    Some(caps) => Robot {
                        position: Position {
                            x: caps["px"].parse().unwrap(),
                            y: caps["py"].parse().unwrap(),
                        },
                        velocity: Velocity {
                            x: caps["vx"].parse().unwrap(),
                            y: caps["vy"].parse().unwrap(),
                        },
                    },
                    None => panic!("did not find matching captures"),
                })
                .collect(),
            size: None,
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

    use crate::{BathroomSecurity, Position, read_input};

    #[test]
    fn test_part1() {
        let reader = read_input("example-input.txt");
        let mut ebhq = BathroomSecurity::from(reader).with_size(Position { x: 11, y: 7 });
        for i in 0..100 {
            // println!("After {i} seconds:\n{ebhq}");
            ebhq = ebhq.next();
        }
        // println!("{}", ebhq);
        assert_eq!(ebhq.safety_factor(), 12);
    }

    #[test]
    fn test_part2() {
        let reader = read_input("example-input.txt");
        let ebhq = BathroomSecurity::from(reader).with_size(Position { x: 11, y: 7 });
    }
}
