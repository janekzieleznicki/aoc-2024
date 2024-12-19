#![feature(unsigned_signed_diff)]

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::fmt::{Display};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Instant;

fn main() {
    let _reader = read_input("puzzle-input.txt");
    let ram_run = RAMRun::from(_reader);
    {
        let mut ram_run = ram_run.clone();
        let start = Instant::now();
        ram_run.fall_bytes(1024);
        let path = ram_run.cheapest_path_from(Position { x: 0, y: 0 });
        println!(
            "Minimum steps: {} in {:?}",
            path.unwrap().len(),
            start.elapsed()
        );
    }
    {
        let start = Instant::now();
        println!(
            "Best seats: {:?} in {:?}",
            first_blocking_byte(&ram_run),
            start.elapsed()
        );
    }
}
fn first_blocking_byte(ram_run: &RAMRun) -> Position {
    let (mut left, mut right) = (0usize, ram_run.falling_bytes.bytes.len());
    loop {
        let test = left.midpoint(right);
        let mut ram_run = ram_run.clone();
        ram_run.fall_bytes(test);
        match ram_run.cheapest_path_from(Position { x: 0, y: 0 }) {
            Some(_) => left = test,
            None => right = test,
        }
        // println!("Bisection: {left}..{right}");
        if left.abs_diff(right) <= 1 {
            break;
        }
    }
    ram_run.falling_bytes.bytes[right - 1]
}
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Position {
    x: usize,
    y: usize,
}
impl Position {
    fn with_diff(&self, xd: isize, yd: isize) -> Option<Position> {
        let nx = match self.x as isize + xd {
            x if x < 0 => return None,
            x => x as usize,
        };
        let ny = match self.y as isize + yd {
            y if y < 0 => return None,
            y => y as usize,
        };
        Some(Position { x: nx, y: ny })
    }
}
impl FromStr for Position {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        Ok(Self {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

enum Tile {
    Safe,
    Corrupted,
}
impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(match self {
            Tile::Safe => write!(f, ".")?,
            Tile::Corrupted => write!(f, "#")?,
        })
    }
}
#[derive(Debug, Clone)]
struct MemoryCorruptor {
    bytes: VecDeque<Position>,
}
impl MemoryCorruptor {}
impl<Reader> From<Reader> for MemoryCorruptor
where
    Reader: BufRead,
{
    fn from(reader: Reader) -> Self {
        Self {
            bytes: reader
                .lines()
                .map_while(|line| line.ok())
                .map(|line| Position::from_str(&line).unwrap())
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
struct RAMRun {
    falling_bytes: MemoryCorruptor,
    corrupted: Vec<Position>,
    start: Position,
    end: Position,
    size: Position,
}
impl RAMRun {
    fn at(&self, pos: Position) -> Tile {
        match self.corrupted.iter().find(|&&corrupted| corrupted == pos) {
            Some(_) => Tile::Corrupted,
            None => Tile::Safe,
        }
    }
    pub fn fall_byte(&mut self) -> Option<Position> {
        if let Some(pos) = self.falling_bytes.bytes.pop_front() {
            self.corrupted.push(pos);
            return Some(pos);
        }
        None
    }
    pub fn fall_bytes(&mut self, count: usize) {
        for _ in 0..count {
            if let None = self.fall_byte() {
                break;
            }
        }
    }

    fn valid_path(&self, pos: Position) -> Option<Position> {
        if pos.x < self.size.x && pos.y < self.size.y {
            match self.corrupted.contains(&pos) {
                true => None,
                false => Some(pos),
            }
        } else {
            None
        }
    }
    fn next_nodes(&self, pos: Position) -> Vec<Position> {
        vec![
            pos.with_diff(1, 0),
            pos.with_diff(0, 1),
            pos.with_diff(-1, 0),
            pos.with_diff(0, -1),
        ]
        .into_iter()
        .filter_map(|p| match p {
            Some(p) => self.valid_path(p),
            None => None,
        })
        .collect()
    }
    fn reconstruct_path(
        came_from: &HashMap<Position, Position>,
        mut position: Position,
    ) -> Vec<Position> {
        let mut path = VecDeque::new();
        while let Some(pos) = came_from.get(&position) {
            position = *pos;
            path.push_front(*pos)
        }
        path.into_iter().collect()
    }
    fn cheapest_path_from(&self, start: Position) -> Option<Vec<Position>> {
        let mut open_set = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let mut g_score = HashMap::new();

        g_score.insert(start, 0);
        open_set.push(HeapEntry {
            pos: start,
            cost: 0,
        });

        while let Some(HeapEntry {
            pos: current,
            mut cost,
        }) = open_set.pop()
        {
            cost += 1;
            if current == self.end {
                return Some(Self::reconstruct_path(&came_from, current));
            }
            for next in self.next_nodes(current) {
                if cost <= *g_score.entry(next).or_insert(usize::MAX) {
                    came_from.insert(next, current);
                    // println!("Current path: \n{}\n",self.print_with_marked_path(&came_from, next));
                    g_score.insert(next, cost);
                    let new = HeapEntry {
                        pos: next,
                        cost: cost,
                    };
                    if open_set.iter().find(|&&e| e == new).is_none() {
                        open_set.push(new);
                    }
                }
            }
        }
        None
    }
    #[allow(dead_code)]
    fn print_with_marked_path(
        &self,
        came_from: &HashMap<Position, Position>,
        path_end: Position,
    ) -> String {
        let path = Self::reconstruct_path(&came_from, path_end);
        let mut str = String::new();
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let in_oath = path
                    .iter()
                    .find(|&&corrupted| corrupted == Position { x, y })
                    .is_some();
                match self.at(Position { x, y }) {
                    Tile::Safe => {
                        if in_oath {
                            str.push('O');
                        } else {
                            str.push('.');
                        }
                    }
                    Tile::Corrupted => {
                        if in_oath {
                            str.push('!');
                        } else {
                            str.push('#');
                        }
                    }
                }
            }
            str.push('\n');
        }
        str
    }
}
#[derive(Eq, PartialEq, Copy, Clone, Debug, Ord, Hash)]
struct HeapEntry {
    pos: Position,
    cost: usize,
}
impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cost.cmp(&other.cost).reverse())
    }
}
impl<Reader> From<Reader> for RAMRun
where
    Reader: BufRead,
{
    fn from(reader: Reader) -> Self {
        let corruptor = MemoryCorruptor::from(reader);
        let max = corruptor.bytes.iter().max_by_key(|&&pos| pos.x).unwrap();
        let end = if max.x > 6 || max.y > 6 {
            Position { x: 70, y: 70 }
        } else {
            Position { x: 6, y: 6 }
        };
        Self {
            falling_bytes: corruptor,
            corrupted: vec![],
            start: Position { x: 0, y: 0 },
            end: end,
            size: Position {
                x: end.x + 1,
                y: end.y + 1,
            },
        }
    }
}

impl Display for RAMRun {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                write!(f, "{}", self.at(Position { x, y }))?;
            }
            writeln!(f, "")?
        }
        Ok(())
    }
}
fn read_input(name: &str) -> BufReader<File> {
    let mut example_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    example_data.push(format!("resources/{name}"));

    BufReader::new(File::open(example_data).unwrap())
}
#[cfg(test)]
mod tests {
    use crate::{first_blocking_byte, read_input, Position, RAMRun};

    #[test]
    fn test_part1() {
        let _reader = read_input("example-input.txt");
        let mut ram_run = RAMRun::from(_reader);
        // println!("Init:\n{ram_run}");
        ram_run.fall_bytes(12);
        // println!("After 12ns:\n{ram_run}");
        assert_eq!(ram_run.cheapest_path_from(ram_run.start).unwrap().len(), 22);
    }
    #[test]
    fn test_part2() {
        let _reader = read_input("example-input.txt");
        let ram_run = RAMRun::from(_reader);
        assert_eq!(first_blocking_byte(&ram_run), Position { x: 6, y: 1 });
    }
}
