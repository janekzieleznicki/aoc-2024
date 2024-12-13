#![feature(unsigned_signed_diff)]

use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, AddAssign};
use std::path::PathBuf;
use std::time::SystemTime;

fn main() {
    let _reader = read_input("puzzle-input.txt");
    let mut map = GuarddMap::from(_reader);
    let start = SystemTime::now();
    println!("Visited count {} in {:?}", map.visited_postions(), SystemTime::now().duration_since(start).unwrap());
    println!("Put obstacles count {} in {:?}", map.put_obstacles(), SystemTime::now().duration_since(start).unwrap());

}
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
struct Position{
    x: isize,
    y: isize,
}
type Dimension = Position;
#[derive(Debug,Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct Direction {
    x: isize,
    y: isize,
}
impl Direction {
    pub fn turn_right(&self) -> Direction {
        Direction{x: -self.y, y: self.x}
    }
}

impl Add<&Direction> for Position {
    type Output = Position;
    fn add(self, rhs: &Direction) -> Position {
        Position{x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl AddAssign<&Direction> for Position {
    fn add_assign(&mut self, rhs: &Direction) {
        *self = self.clone() + rhs
    }
}
#[derive(Clone)]
struct Arena {
    dimensions: Dimension,
    obstacles: HashSet<Position>,
}
impl Arena {
    pub fn contains(&self, position: Position) -> bool {
        if position.x < 0 || position.x >= self.dimensions.x+1 {
            false
        }else if position.y < 0 || position.y >= self.dimensions.y+1 {
            false
        } else {
            true
        }
    }
}

#[derive(Debug, PartialEq)]
struct InfiniteLoopError {}
#[derive(Clone)]
struct GuarddMap {
    guard_start: Position,
    guard_direction: Direction,
    arena: Arena,
    visited: HashSet<Position>,
}
impl Display for GuarddMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..=self.arena.dimensions.y {
            for x in 0..=self.arena.dimensions.x {
                if self.arena.obstacles.contains(&Position{x,y}){
                    write!(f, "#")?;
                } else if self.guard_start == (Position{x,y}) {
                    write!(f, "^")?;
                } else if self.visited.contains(&Position{x,y}){
                    write!(f, "X")?;
                }
                else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
impl GuarddMap {
    pub fn visited_postions(&mut self) -> usize {
        let mut guard_position = self.guard_start.clone();
        let mut guard_direction = self.guard_direction.clone();
        loop {
            if !self.arena.contains(guard_position){
                return self.visited.len()
            }
            if self.arena.obstacles.contains(&(guard_position.clone()+&guard_direction)){
                guard_direction = guard_direction.turn_right();
            } else {
                self.visited.insert(guard_position.clone());
                guard_position+=&guard_direction;
            }
            // println!("Visited: {}:\n{self}", self.visited.len());
        }
    }
    pub fn move_guard(&self) -> Result<(), InfiniteLoopError> {
        let mut guard_position = self.guard_start.clone();
        let mut guard_direction = self.guard_direction.clone();
        let mut path = Vec::new();
        loop {
            if !self.arena.contains(guard_position){
                return Ok(())
            }
            if self.arena.obstacles.contains(&(guard_position.clone()+&guard_direction)){
                guard_direction = guard_direction.turn_right();
            } else {
                guard_position+=&guard_direction;
            }
            if path.contains(&(guard_position.clone(), guard_direction.clone())){
                return Err(InfiniteLoopError {});
            }
            path.push((guard_position.clone(), guard_direction.clone()));
        }
    }
    pub fn put_obstacles(&self) -> usize {
        let mut guard_position = self.guard_start.clone();
        let mut guard_direction = self.guard_direction.clone();
        let mut possible_obstacles = Vec::new();
        loop {
            if self.guard_start != guard_position+&guard_direction {
                let mut possible_map = self.clone();
                possible_map.arena.obstacles.insert(guard_position+&guard_direction);
                possible_map.guard_start = guard_position;
                possible_map.guard_direction = guard_direction;
                match possible_map.move_guard() {
                    Ok(()) => {},
                    Err(InfiniteLoopError {}) => { possible_obstacles.push(guard_position) },
                }
            }
            if self.arena.obstacles.contains(&(guard_position+&guard_direction)){
                guard_direction = guard_direction.turn_right();
            } else {
                guard_position+=&guard_direction;
            }
            if !self.arena.contains(guard_position){
                return possible_obstacles.into_iter().collect::<HashSet<_>>().len()
            }
        }
    }
}
impl<Reader> From<Reader> for GuarddMap
where
    Reader: BufRead,
{
    fn from(reader: Reader) -> Self {
        let mut arena = Arena{dimensions: Dimension{x: 0, y:0}, obstacles: HashSet::new()};
        let mut guard_start = Position{x: 0, y: 0};
        let mut guard_direction = Direction{x: 0, y: 0};
        for (y,line) in reader.lines()
            .map_while(|line| {line.ok()}).enumerate() {
                arena.dimensions = Dimension{x: (line.len()-1) as isize, y: y as isize};
                for (x,char) in line.chars().enumerate() {
                    match char {
                        '.' => continue,
                        '#' => { arena.obstacles.insert(Position { x: x as isize, y: y as isize }); },
                        '^' => {
                            guard_start = Position{x: x as isize, y: y as isize};
                            guard_direction = Direction{x: 0,y: -1};
                        },
                        _ => panic!("Unknown char in input: {}", char)
                    }
                }
            }
        Self{
            guard_start,
            guard_direction,
            arena,
            visited: HashSet::new()
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
    use crate::{read_input, Direction, GuarddMap, InfiniteLoopError, Position};



    #[test]
    fn test_direction() {
        let direction = Direction{x: 0, y: 1};
        assert_eq!(direction.turn_right(), Direction{x: -1, y: 0});
        assert_eq!(direction.turn_right().turn_right(), Direction{x: 0, y: -1});
        assert_eq!(direction.turn_right().turn_right().turn_right(), Direction{x: 1, y: 0});

        let mut position = Position{x: 0, y: 0};
        position+=&direction;
        assert_eq!(position, Position{ x: 0, y: 1 });
        position+=&direction;
        assert_eq!(position, Position{ x: 0, y: 2 })
    }
    #[test]
    fn test_part1() {
        let _reader = read_input("example-input.txt");
        let mut map = GuarddMap::from(_reader);
        // println!("Map:\n{map}");
        assert_eq!(map.visited_postions(), 41);
    }
    #[test]
    fn test_printing_press() {
        let _reader = read_input("example-input.txt");
        let mut map = GuarddMap::from(_reader);
        map.arena.obstacles.insert(Position{x:3, y:6});
        println!("Map printing press:\n{map}");
        assert_eq!(map.move_guard(), Err(InfiniteLoopError {}));
    }
    #[test]
    fn test_failed_suit_prototypes() {
        let _reader = read_input("example-input.txt");
        let mut map = GuarddMap::from(_reader);
        map.arena.obstacles.insert(Position{x:6, y:7});
        println!("Map failed suit:\n{map}");
        assert_eq!(map.move_guard(), Err(InfiniteLoopError {}));
    }

    #[test]
    fn test_tank_glue() {
        let _reader = read_input("example-input.txt");
        let mut map = GuarddMap::from(_reader);
        map.arena.obstacles.insert(Position{x:7, y:9});
        println!("Map tank glue:\n{map}");
        assert_eq!(map.move_guard(), Err(InfiniteLoopError {}));
    }
    #[test]
    fn test_put_obstacles() {

        let _reader = read_input("example-input.txt");
        let mut map = GuarddMap::from(_reader);
        assert_eq!(map.put_obstacles(), 6);
    }
}
