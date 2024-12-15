#![feature(unsigned_signed_diff)]

use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem::swap;
use std::path::PathBuf;

fn main() {
    let _reader = read_input("puzzle-input.txt");
    let farm = Farm::from(_reader);
    println!("Total fencing cost: {}", farm.fencing_cost());
    println!("Total discounted cost: {}", farm.discounted_cost())
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Position {
    x: isize,
    y: isize,
}
#[derive(Debug)]
struct Field {
    crop: char,
    occupies: Vec<Position>,
    area: usize,
    perimeter: usize,
    corners: usize,
}
impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // write!(f, "Field '{}' A: {} P: {}",self.crop, self.area, self.perimeter)
        write!(f, "Field '{}' A: {} C: {}",self.crop, self.area, self.corners)
    }
}
impl Field {
    pub fn add(&mut self, pos: Position) {
        match self.occupies.len() {
            0 => {
                self.perimeter = 4;
                self.corners = 4;
            },
            _ => {
                match (self.occupies.iter().find(|&&p| p == Position { x: pos.x - 1, y: pos.y })
                       , self.occupies.iter().find(|&&p| p == Position { x: pos.x, y: pos.y - 1 })) {
                    (Some(_), Some(_)) => {
                        self.corners -= 2;
                        if self.occupies.iter().find(|&&p| p == Position { x: pos.x +1, y: pos.y - 1 }).is_some() {
                            self.corners += 2;
                        }
                    }
                    (None, Some(_)) => {
                        self.perimeter += 2;
                        if self.occupies.iter().find(|&&p| p == Position { x: pos.x +1, y: pos.y - 1 }).is_some() {
                            self.corners += 2;
                        }
                        if self.occupies.iter().find(|&&p| p == Position { x: pos.x -1, y: pos.y - 1 }).is_some() {
                            self.corners += 2;
                        }
                    }
                    (Some(_), None) => {
                        self.perimeter += 2;
                        if self.occupies.iter().find(|&&p| p == Position { x: pos.x -1, y: pos.y - 1 }).is_some() {
                            self.corners += 2;
                        }
                    }
                    (None, None) => unreachable!(),
                }

            }
        }
        self.area += 1;
        self.occupies.push(pos);
    }
    pub fn new_with_crop(crop: char, pos: Position) -> Self {
        let mut field = Self{
            crop,
            occupies: vec![],
            area: 0,
            perimeter: 0,
            corners: 0,
        };
        field.add(pos);
        field
    }
    pub fn merge_at(&mut self, mut other: Field, pos: Position) {
        let top_right = Position{x: pos.x +1, y: pos.y - 1};
        if self.occupies.contains(&top_right) || other.occupies.contains(&top_right) {
            self.corners += 2;
        }
        self.occupies.append(&mut other.occupies);
        self.area += other.area + 1;
        self.perimeter += other.perimeter;
        self.corners += other.corners -2 ;
        self.occupies.push(pos)
    }
}
#[derive(Debug)]
struct Farm {
    fields: Vec<Field>,
}
impl Display for Farm {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Fields: [")?;
        for field in &self.fields {
            write!(f, "{}, ", field)?;
        }
        write!(f, "] discounted cost: {}", self.discounted_cost())?;

        Ok(())
    }
}
impl Farm {
    pub fn fencing_cost(&self) -> usize {
        self.fields.iter().map(|f|f.perimeter*f.area).sum()
    }
    pub fn discounted_cost(&self) -> usize {
        self.fields.iter().map(|f|f.corners*f.area).sum()
    }
    fn field_with_pos(&self, crop: char, pos: Position) -> Option<usize>{
        self.fields.iter().position(|f| f.crop == crop && f.occupies.contains(&pos))
    }
    fn add_plot(&mut self, crop: char, pos: Position) {

        match (self.field_with_pos(crop, Position{ x: pos.x-1, y: pos.y }),self.field_with_pos(crop, Position{ x: pos.x, y: pos.y-1 })) {
            (None, None) => self.fields.push(Field::new_with_crop(crop, pos)),
            (Some(idx), None) => self.fields[idx].add(pos),
            (None, Some(idx)) => self.fields[idx].add(pos),
            (Some(left), Some(right)) if left == right => self.fields[left].add(pos),
            (Some(mut left),Some(mut right)) => {
                if right < left{
                    swap(&mut left,&mut right);
                }
                let right = self.fields.remove(right);
                self.fields[left].merge_at(right, pos)
            },
        }
    }
}
impl Default for Farm {
    fn default() -> Self {
        Self {
            fields: Vec::new(),
        }
    }

}

impl<Reader> From<Reader> for Farm
where
    Reader: BufRead,
{
    fn from(reader: Reader) -> Self {
        let mut farm = Farm::default();
        for (y, line) in reader.lines().map_while(|line| line.ok()).enumerate() {
            for (x, char) in line.chars().enumerate() {
                farm.add_plot(char, Position { x: x as isize, y: y as isize });
                // println!("{char} => {farm}")
            }
        }
        farm
    }
}
fn read_input(name: &str) -> BufReader<File> {
    let mut example_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    example_data.push(format!("resources/{name}"));

    BufReader::new(File::open(example_data).unwrap())
}
#[cfg(test)]
mod tests {
    use crate::Farm;
    #[test]
    fn test_example1() {
        let example = r"AAAA
BBCD
BBCC
EEEC";
        let farm = Farm::from(example.as_bytes());
        assert_eq!(farm.fencing_cost(), 140);
        assert_eq!(farm.discounted_cost(), 80);
    }
    #[test]
    fn test_example2() {
        let example = r"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        let farm = Farm::from(example.as_bytes());
        assert_eq!(farm.fencing_cost(), 772);
        assert_eq!(farm.discounted_cost(), 436);
    }
    #[test]
    fn test_example3() {
        let example = r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        let farm = Farm::from(example.as_bytes());
        assert_eq!(farm.fencing_cost(), 1930);
        assert_eq!(farm.discounted_cost(), 1206);
    }
    #[test]
    fn test_example4() {
        let example = r"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        let farm = Farm::from(example.as_bytes());
        assert_eq!(farm.fencing_cost(), 692);
        assert_eq!(farm.discounted_cost(), 236);
    }
    #[test]
    fn test_example5() {
        let example = r"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        let farm = Farm::from(example.as_bytes());
        assert_eq!(farm.fencing_cost(), 1184);
        assert_eq!(farm.discounted_cost(), 368);
    }
}
