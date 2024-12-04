use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::time::Instant;
fn main() {
    let puzzle = parse(&mut read_input("puzzle-input.txt"));
    let mut start = Instant::now();
    let xmas_count = puzzle.xmas_count();
    println!("XMAS count => {} in {:?}", xmas_count, start.elapsed());
    start = Instant::now();
    let cross_count = puzzle.cross_count();
    println!("X-MAS count => {} in {:?}", cross_count, start.elapsed());
}
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct Coordinates {
    row: usize,
    col: usize,
}
pub struct Vector {
    row: isize,
    col: isize,
}
#[derive(Debug, Default)]
pub struct Puzzle {
    indices: HashMap<char, HashSet<Coordinates>>,
    chars_map: HashMap<Coordinates, char>,
}
impl Puzzle {
    pub fn insert(&mut self, c: char, coord: Coordinates) {
        //self.indices[&c].insert(coord);
        self.indices.entry(c).or_default().insert(coord.clone());
        self.chars_map.insert(coord, c);
    }
    fn next_index(&self, coord: Coordinates, vector: &Vector) -> Option<Coordinates> {
        match coord.col.checked_add_signed(vector.col) {
            None => None,
            Some(col) => match coord.row.checked_add_signed(vector.row) {
                None => None,
                Some(row) => Some(Coordinates { row, col }),
            },
        }
    }

    fn xmas_with_start(&self, coord: Coordinates) -> usize {
        let word = "XMAS";
        let directions = vec![
            Vector { row:  0, col:  1 },   // east
            Vector { row:  1, col:  0 },   // north
            Vector { row:  1, col:  1 },   // north east
            Vector { row:  0, col: -1 },   // west
            Vector { row: -1, col:  0 },   // south
            Vector { row: -1, col: -1 },   // south west
            Vector { row: -1, col:  1 },   // south east
            Vector { row:  1, col: -1 },   // north west
        ];
        let mut count = 0;
        for dir in directions {
            let mut tmp_coords = coord.clone();
            for char in word.chars() {
                if !self.indices.get(&char).unwrap().contains(&tmp_coords) {
                    break;
                } else if char == word.chars().last().unwrap() {
                    count += 1;
                } else {
                    match self.next_index(tmp_coords, &dir) {
                        None => break,
                        Some(c) => tmp_coords = c,
                    }
                }
            }
        }
        count
    }
    pub fn xmas_count(&self) -> usize {
        let xindices = self.indices.get(&'X').unwrap();
        xindices
            .iter()
            .map(|coord| self.xmas_with_start(*coord))
            .sum()
    }
    fn cross_with_middle(&self, coord: Coordinates) -> bool {
        let directions = vec![
            Vector { row:  1, col: -1 },  // north west
            Vector { row:  1, col:  1 },  // north east
            Vector { row: -1, col:  1 },  // south east
            Vector { row: -1, col: -1 },  // south west
        ];
        let neighbours = directions
            .iter()
            .map_while(|dir| self.next_index(coord.clone(), dir))
            .map_while(|coords| self.chars_map.get(&coords))
            .copied()
            .collect::<String>();
        match neighbours.as_str() {
            "SSMM" => true,
            "SMMS" => true,
            "MMSS" => true,
            "MSSM" => true,
            _ => false,
        }
    }
    pub fn cross_count(&self) -> usize {
        let aindices = self.indices.get(&'A').unwrap().clone();
        aindices
            .into_iter()
            .filter(|coord| self.cross_with_middle(*coord))
            .count()
    }
}

pub fn parse<Reader>(reader: &mut Reader) -> Puzzle
where
    Reader: ?Sized + BufRead,
{
    let mut puzzle = Puzzle::default();
    reader
        .lines()
        .into_iter()
        .map_while(Result::ok)
        .enumerate()
        .for_each(|(row, line)| {
            line.char_indices()
                .for_each(|(col, c)| puzzle.insert(c, Coordinates { row, col }))
        });
    return puzzle;
}

fn read_input(name: &str) -> BufReader<File> {
    let mut example_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    example_data.push(format!("resources/{name}"));
    BufReader::new(File::open(example_data).unwrap())
}
#[cfg(test)]
mod tests {

    use crate::{parse, read_input};

    #[test]
    fn test_part1() {
        let puzzle = parse(&mut read_input("example-input.txt"));
        assert_eq!(18, puzzle.xmas_count());
    }
    #[test]
    fn test_part2() {
        let puzzle = parse(&mut read_input("example-input.txt"));
        assert_eq!(9, puzzle.cross_count());
    }
}
