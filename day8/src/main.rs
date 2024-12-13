#![feature(unsigned_signed_diff)]

use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, IndexMut};
use std::path::PathBuf;
use gcd::Gcd;
fn main() {
    let _reader = read_input("puzzle-input.txt");
    let antennas_map = AntennasMap::from(_reader);
    println!("Unique antinodes: {}", antennas_map.unique_antinodes());
    println!("Rezonance harmonics: {}", antennas_map.rezonance_harmonics())

}
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Position{
    x: isize,
    y: isize,
}
type Vector = Position;
impl Vector {
    pub fn reverse(&self) -> Self{
        Self{
            x: -self.x,
            y: -self.y,
        }
    }
}
impl From<(&Position,&Position)> for Vector {
    fn from(value: (&Position, &Position)) -> Self {
        Self{
            x: value.1.x - value.0.x,
            y: value.1.y - value.0.y,
        }
    }
}
impl Add<Vector> for Position {
    type Output = Position;
    fn add(self, rhs: Vector) -> Self::Output {
        Self::Output{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
pub fn antinodes(first: Position, second: Position) -> (Position, Position){
    let vector = Vector::from((&first, &second));
    let reverse = vector.reverse();
    (first+reverse, second+vector)
}
type Dimensions = Position;
#[derive(Debug,Default)]
struct AntennasMap{
    antennas: HashMap<char, Vec<Position>>,
    dimensions: Dimensions,
}
impl AntennasMap {
    fn in_dimensions(&self, position: &Position) -> bool {
        position.x >= 0 && position.x < self.dimensions.x && position.y >= 0 && position.y <= self.dimensions.y
    }
    pub fn unique_antinodes(&self) -> usize{
        self.antennas.iter().map({|(antenna, positions)|
            positions.iter().enumerate().map(|(idx, pos)|
                match positions.split_at_checked(idx+1) {
                    Some((_,rest)) => {
                        rest.iter()
                            .map(|sec|antinodes(*pos,*sec))
                            .map(|(a1,a2)|vec![a1,a2])
                            .into_iter().flatten()
                            .collect::<Vec<_>>()
                    }
                    None => {
                        vec![]
                    }
                }
            ).flatten()
        })
            .flatten()
            .filter(|pos| self.in_dimensions(pos))
            .collect::<HashSet<_>>()
            .iter().count()
    }
    fn antinodes(&self, first: &Position, second: &Position) -> Vec<Position> {
        let tmp = Vector::from((first, second));
        let gcd = (tmp.x.abs() as u64).gcd(tmp.y.abs() as u64) as isize;
        let vector = Vector{
            x: tmp.x / gcd,
            y: tmp.y / gcd
        };
        // println!("{tmp:?}, gcd: {gcd} => {vector:?}");
        let reverse = vector.reverse();
        let mut out = Vec::new();
        let mut first = first.clone();
        loop {
            if self.in_dimensions(&first) {
                out.push(first);
            }else{
                break;
            }
            first = first + reverse;
        }
        let mut second = second.clone();
        loop {
            if self.in_dimensions(&second) {
                out.push(second);
            }else{
                break;
            }
            second = second + vector;
        }
        out
    }
    pub fn rezonance_harmonics(&self) -> usize{
        let antinodes = self.antennas.iter().map({|(antenna, positions)|
            positions.iter().enumerate().map(|(idx, pos)|
                match positions.split_at_checked(idx+1) {
                    Some((_,rest)) => {
                        rest.iter()
                            .map(|sec|self.antinodes(pos,sec))
                            .into_iter().flatten()
                            .collect::<Vec<_>>()
                    }
                    None => {
                        vec![]
                    }
                }
            ).flatten()
        })
            .flatten()
            .filter(|pos| self.in_dimensions(pos))
            .collect::<HashSet<_>>();
        println!("{}\n\n{}", self, pretty_print(&self,&antinodes));
        antinodes.into_iter().count()
    }
}
impl Display for AntennasMap{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string =
        (0..=self.dimensions.y)
            .map(|_| vec!['.'; self.dimensions.x as usize+1])
            .map(|mut v| {
                v.push('\n');
                v
            })
            .flatten().collect::<Vec<_>>();

        for (char, pos) in self.antennas.iter(){
            for pos in pos{
                let idx = (pos.y*(self.dimensions.x+2)+pos.x) as usize;
                string[idx] = *char;
            }
        }
        write!(f, "{}", string.into_iter().collect::<String>());
        Ok(())
    }
}
pub fn pretty_print(map: &AntennasMap, antinodes: &HashSet<Position>) -> String {
    let mut string =
        (0..=map.dimensions.y)
            .map(|_| vec!['.'; map.dimensions.x as usize])
            .map(|mut v| {
                v.push('\n');
                v
            })
            .flatten().collect::<Vec<_>>();
    for antinodes in antinodes{
        let idx = (antinodes.y*(map.dimensions.x+1)+antinodes.x) as usize;
        string[idx] = '#';
    }
    for (char, pos) in map.antennas.iter(){
        for pos in pos{
            let idx = (pos.y*(map.dimensions.x+1)+pos.x) as usize;
            string[idx] = *char;
        }
    }
    string.into_iter().collect::<String>()
}
impl<Reader> From<Reader> for AntennasMap
where
    Reader: BufRead,
{
    fn from(reader: Reader) -> Self {
        let mut antennas_map = Self::default();
        for (y,line) in reader.lines()
            .map_while(|line| {line.ok()}).enumerate() {
            antennas_map.dimensions = Dimensions{x: line.len() as isize, y: y as isize};
            for (x,char) in line.chars().enumerate() {
                match char {
                    '.' => continue,
                    a if a.is_alphanumeric() => {
                        antennas_map.antennas.entry(a).or_default().push(Position{x: x as isize, y: y as isize});
                    }
                    _ => panic!("Unknown char in input: {}", char)
                }
            }
        }
        antennas_map
    }
}
fn read_input(name: &str) -> BufReader<File> {
    let mut example_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    example_data.push(format!("resources/{name}"));

    BufReader::new(File::open(example_data).unwrap())
}
#[cfg(test)]
mod tests {

    use crate::{read_input, AntennasMap};

    #[test]
    fn test_part1() {
        let _reader = read_input("example-input.txt");
        let antennas_map = AntennasMap::from(_reader);
        println!("{}", antennas_map);
        assert_eq!(antennas_map.unique_antinodes(),14);
    }
    #[test]
    fn test_part2() {
        let _reader = read_input("example-input.txt");
        let antennas_map = AntennasMap::from(_reader);
        println!("{}", antennas_map);
        assert_eq!(antennas_map.rezonance_harmonics(),34);
    }
}
