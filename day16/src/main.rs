#![feature(unsigned_signed_diff)]

use crate::Move::{Down, Left, Right, Up};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::path::PathBuf;

fn main() {
    let _reader = read_input("puzzle-input.txt");
    let maze = Maze::from(_reader);
    println!("Cheapest path: {}\nbest seats: {}", maze.cheapest_path(), maze.best_seats());
}
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Position {
    x: usize,
    y: usize,
}
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    pub fn rotate_clockwise(&self) -> Move {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
    pub fn rotate_counter_clockwise(&self) -> Move {
        self.rotate_clockwise()
            .rotate_clockwise()
            .rotate_clockwise()
    }
}
impl Add<Move> for Position {
    type Output = Position;
    fn add(self, rhs: Move) -> Self::Output {
        match rhs {
            Up => Self {
                x: self.x,
                y: self.y - 1,
            },
            Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Left => Self {
                x: self.x - 1,
                y: self.y,
            },
            Right => Self {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Tile {
    Start,
    End,
    Empty,
    Wall,
}
impl From<char> for Tile {
    fn from(c: char) -> Tile {
        match c {
            'S' => Tile::Start,
            'E' => Tile::End,
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            _ => unreachable!(),
        }
    }
}
impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Tile::Start => 'S',
            Tile::End => 'E',
            Tile::Empty => '.',
            Tile::Wall => '#',
        }
    }
}
impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", Into::<char>::into(*self))
    }
}
struct Maze {
    map: Vec<Tile>,
    start: Position,
    end: Position,
    size: Position,
}
impl Maze {
    pub fn where_end(mut self) -> Self {
        self.end = self
            .map
            .iter()
            .position(|&t| t == Tile::End)
            .map(|pos| self.pos_from_index(pos))
            .unwrap();
        assert_eq!(self.at(self.end), Tile::End);
        self
    }
    pub fn where_start(mut self) -> Self {
        self.start = self
            .map
            .iter()
            .rposition(|&t| t == Tile::Start)
            .map(|pos| self.pos_from_index(pos))
            .unwrap();
        assert_eq!(self.at(self.start), Tile::Start);
        self
    }
    fn pos_from_index(&self, index: usize) -> Position {
        Position {
            x: index % self.size.x,
            y: index / self.size.x,
        }
    }
    fn at(&self, index: Position) -> Tile {
        self.map[index.y * (self.size.x) + index.x]
    }
    fn next_nodes(&self, current: Raindeer) -> Vec<Raindeer> {
        let mut next_nodes = Vec::new();
        match self.at(current.forward().pos) {
            Tile::Empty | Tile::End => next_nodes.push(current.forward()),
            _ => {}
        };
        match self.at(current.left().pos) {
            Tile::Empty | Tile::End => next_nodes.push(current.left()),
            _ => {}
        };
        match self.at(current.right().pos) {
            Tile::Empty | Tile::End => next_nodes.push(current.right()),
            _ => {}
        }
        next_nodes
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
    fn cheapest_path_from(&self, start: Raindeer) -> usize {
        let mut open_set = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let mut g_score = HashMap::new();

        g_score.insert(start.pos, start.cost);
        open_set.push(start);

        while let Some(current) = open_set.pop() {
            if current.pos == self.end {
                return current.cost;
            }
            for next in self.next_nodes(current) {
                if next.cost <= *g_score.entry(next.pos).or_insert(usize::MAX) {
                    came_from.insert(next.pos, current.pos);
                    g_score.insert(next.pos, next.cost);
                    if open_set.iter().find(|&&e| e == next).is_none() {
                        open_set.push(next);
                    }
                }
            }
        }
        usize::MAX
    }
    fn cheapest_path(&self) -> usize {
        self.cheapest_path_from(Raindeer {
            pos: self.start,
            orientation: Right,
            cost: 0,
        })
    }

    fn paths_from(&self, path: Vec<Raindeer>, max_cost: usize) -> Vec<Vec<Raindeer>> {
        let mut results = Vec::new();
        match path.last() {
            Some(last) => {
                for next in self.next_nodes(*last) {
                    if self.cheapest_path_from(next) > max_cost {
                        continue;
                    }
                    let mut new_path = path.clone();
                    new_path.push(next);
                    if next.pos == self.end {
                        results.push(new_path);
                    } else {
                        results.append(&mut self.paths_from(new_path, max_cost));
                    }
                }
            }
            None => unreachable!("Empty path"),
        }
        results
    }
    pub fn best_seats(&self) -> usize {
        // 1. Lets get cheapest path
        // 2. DFS for every path to end, which is just as cheap
        let best_cost = self.cheapest_path();
        let paths = self.paths_from(
            vec![Raindeer {
                pos: self.start,
                orientation: Right,
                cost: 0,
            }],
            best_cost,
        );
        paths.into_iter().flatten().map(|entry|entry.pos).collect::<HashSet<_>>().iter().count()
    }
    #[allow(dead_code)]
    fn print_with_marked_path(&self, came_from: &HashMap<Position, Position>, path_end: Raindeer) {
        {
            let mut string = self
                .map
                .iter()
                .map(|tile| format!("{tile}").chars().nth(0).unwrap())
                .collect::<Vec<_>>();
            for pos in Self::reconstruct_path(&came_from, path_end.pos) {
                string[pos.y * (self.size.x) + pos.x] = 'X'
            }
            let string = string.into_iter().collect::<String>();
            let mut slicer = &string[..];
            while !slicer.is_empty() {
                println!("{}", &slicer[..self.size.x]);
                slicer = &slicer[self.size.x..];
            }
        }
    }
}
#[derive(Eq, PartialEq, Copy, Clone, Debug, Ord)]
struct Raindeer {
    pos: Position,
    orientation: Move,
    cost: usize,
}
impl Raindeer {
    pub fn forward(&self) -> Self {
        Self {
            pos: self.pos + self.orientation,
            orientation: self.orientation,
            cost: self.cost + 1,
        }
    }
    pub fn left(&self) -> Self {
        Self {
            pos: self.pos + self.orientation.rotate_counter_clockwise(),
            orientation: self.orientation.rotate_counter_clockwise(),
            cost: self.cost + 1001,
        }
    }
    pub fn right(&self) -> Self {
        Self {
            pos: self.pos + self.orientation.rotate_clockwise(),
            orientation: self.orientation.rotate_clockwise(),
            cost: self.cost + 1001,
        }
    }
}
impl PartialOrd for Raindeer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cost.cmp(&other.cost).reverse())
    }
}

impl<Reader> From<Reader> for Maze
where
    Reader: BufRead,
{
    fn from(reader: Reader) -> Self {
        let map = reader
            .lines()
            .map_while(|line| line.ok())
            .map(|line| line.chars().into_iter().map(Tile::from).collect())
            .collect::<Vec<Vec<Tile>>>();
        let size = Position {
            x: map[0].len(),
            y: map.len(),
        };
        Self {
            map: map.into_iter().flatten().collect(),
            start: Position { x: 0, y: 0 },
            end: Position { x: 0, y: 0 },
            size,
        }
        .where_start()
        .where_end()
    }
}

impl Display for Maze {
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
    use crate::Move::{Down, Left, Right, Up};
    use crate::{Maze, Position, read_input};

    #[test]
    fn test_helpers() {
        let mover = Up;
        assert_eq!(mover.rotate_clockwise(), Right);
        assert_eq!(mover.rotate_counter_clockwise(), Left);

        let pos = Position { x: 1, y: 1 };
        assert_eq!(Position { x: 0, y: 1 }, pos + Left);
        assert_eq!(Position { x: 2, y: 1 }, pos + Right);
        assert_eq!(Position { x: 1, y: 0 }, pos + Up);
        assert_eq!(Position { x: 1, y: 2 }, pos + Down);
    }
    #[test]
    fn test_case1() {
        let _reader = read_input("example-input.txt");
        let maze = Maze::from(_reader);
        assert_eq!(maze.cheapest_path(), 7036);
        assert_eq!(maze.best_seats(), 45);
    }
    #[test]
    fn test_case2() {
        let _reader = read_input("example-input2.txt");
        let maze = Maze::from(_reader);
        assert_eq!(maze.cheapest_path(), 11048);
        assert_eq!(maze.best_seats(), 64);
    }
}
