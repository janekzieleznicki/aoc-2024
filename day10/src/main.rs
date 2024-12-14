#![feature(unsigned_signed_diff)]

use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn main() {
    let _reader = read_input("puzzle-input.txt");
    let topo = TopoMap::from(_reader);
    println!("Trailhead scores: {}", topo.trailhead_scores());
    println!("Trailhead rating: {}", topo.trailhead_rating());
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position{
    x: isize,
    y: isize,
}
#[derive(Debug,Copy, Clone)]
enum Direction{
    Top,
    Bottom,
    Left,
    Right,
}
struct TopoMap {
    elevations: Vec<u8>,
    summits: Vec<Position>,
    trailheads: Vec<Position>,
    dimensions: Position,
}
impl TopoMap {
    fn elevation_at(&self, pos: &Position) -> u8 {
        self.elevations[pos.y as usize * self.dimensions.x as usize + pos.x as usize]
    }


    fn can_move(&self, pos: &Position, dir: Direction) -> Option<Position> {
        let in_dimensions = |new_pos: Position|{
          if new_pos.x < 0 || new_pos.x >= self.dimensions.x {
              return None
          }
            if new_pos.y < 0 || new_pos.y >= self.dimensions.y {
                return None
            }
            Some(new_pos)
        };
        in_dimensions(match dir {
            Direction::Top => {Position{x: pos.x, y: pos.y - 1}},
            Direction::Bottom => {Position{x: pos.x, y: pos.y + 1}},
            Direction::Left => {Position{x: pos.x - 1, y: pos.y}},
            Direction::Right => {Position{x: pos.x + 1, y: pos.y}},
        })
    }
    fn reachable_summits(&self, pos: &Position, start_elevation: u8) -> Vec<Position> {
        let mut reachable: Vec<Position> = Vec::new();
        for new_pos in vec![Direction::Bottom, Direction::Right, Direction::Left, Direction::Top].iter()
            .filter_map(|dir| self.can_move(pos, *dir)){
            match self.elevation_at(&new_pos){
                x if x == 9 && start_elevation == 8=> reachable.push(new_pos),
                x if x == start_elevation+1 => reachable.append(&mut self.reachable_summits(&new_pos,x)),
                _ => {}
            }
        }
        reachable
    }
    pub fn trailhead_scores(&self) -> usize {
        // for th in self.trailheads.iter(){
        //     println!("From {th:?} we can reach {}",self.reachable_summits(th,0).into_iter().collect::<HashSet<_>>().len())
        // }
        self.trailheads.iter()
            .map(|th|self.reachable_summits(th,0).into_iter().collect::<HashSet<_>>().len())
            .sum()
    }
    pub fn trailhead_rating(&self) -> usize {
        // for th in self.trailheads.iter(){
        //     println!("From {th:?} we can reach {}",self.reachable_summits(th,0).into_iter().collect::<HashSet<_>>().len())
        // }
        self.trailheads.iter()
            .map(|th|self.reachable_summits(th,0).len())
            .sum()
    }
}
impl Display for TopoMap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in 0..self.dimensions.y{
            for x in 0..self.dimensions.x {
                write!(f, "{}", self.elevation_at(&Position{x,y}))?;
            }
            write!(f, "\n")?;
        }
        write!(f, "Summits: {:?}\n", self.summits)?;
        write!(f, "Trailheads: {:?}\n", self.trailheads)?;
        Ok(())
    }
}
impl<Reader> From<Reader> for TopoMap
where
    Reader: BufRead,
{
    fn from(mut reader: Reader) -> Self {
        let mut elevations = Vec::new();
        let mut summits = Vec::new();
        let mut trailheads = Vec::new();
        let mut dimensions = Position{x: 0, y: 0};
        for (y,line) in reader.lines()
            .map_while(|line| {line.ok()}).enumerate() {
            dimensions = Position{x: line.len() as isize, y: y as isize+1};
            for (x,char) in line.chars().enumerate() {
                elevations.push(char.to_digit(10).unwrap() as u8);
                match char {
                    '9' => summits.push(Position{x: x as isize, y: y as isize}),
                    '0' => trailheads.push(Position{x: x as isize, y:y as isize}),
                    _ => {}
                }
            }
        }
        Self{
             elevations,
             summits,
             trailheads,
            dimensions,
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

    use crate::{read_input, TopoMap};

    #[test]
    fn test_part1() {
        let _reader = read_input("example-input.txt");
        let topo = TopoMap::from(_reader);
        assert_eq!(topo.trailhead_scores(), 36);
    }
    #[test]
    fn test_part2() {
        let _reader = read_input("example-input.txt");
        let topo = TopoMap::from(_reader);
        assert_eq!(topo.trailhead_rating(), 81);
    }
}
