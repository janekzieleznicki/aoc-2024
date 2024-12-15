#![feature(unsigned_signed_diff)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn main() {
    let _reader = read_input("puzzle-input.txt");
    let mut grid = Grid::from(_reader);
    for _ in 0..25 {
        grid.blink();
    }
    println!("Stone count after 25 blinks: {}", grid.stone_count());
    for _ in 0..50 {
        grid.blink();
    }
    println!("Stone count after 75 blinks: {}", grid.stone_count());
}
#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Stone {
    number: u128,
}
impl Stone {
    pub fn blink(self) -> (Stone, Option<Stone>) {
        match self.number {
            0 => (Stone { number: 1 }, None),
            x if Self::digits_count(x) % 2 == 0 => {
                let half_digit = Self::digits_count(x) / 2;
                (
                    Stone {
                        number: x / 10u128.pow(half_digit as u32),
                    },
                    Some(Stone {
                        number: x % 10u128.pow(half_digit as u32),
                    }),
                )
            }
            x => (Stone { number: x * 2024 }, None),
        }
    }

    fn digits_count(num: u128) -> usize {
        (0..).take_while(|i| 10u128.pow(*i) <= num).count()
    }
}
struct Grid {
    stones: HashMap<Stone, usize>,
}
impl Grid {
    pub fn blink(&mut self) {
        let mut new_stones = HashMap::new();
        for (stone, count) in self.stones.iter() {
            match stone.blink() {
                (left, Some(right)) => {
                    *new_stones.entry(left).or_insert(0) += count;
                    *new_stones.entry(right).or_insert(0) += count;
                }
                (stone, None) => {
                    *new_stones.entry(stone).or_insert(0) += count;
                }
            }
        }
        self.stones = new_stones;
    }
    pub fn stone_count(&self) -> usize {
        self.stones.values().sum()
    }
}
impl<Reader> From<Reader> for Grid
where
    Reader: BufRead,
{
    fn from(mut reader: Reader) -> Self {
        let mut buf = String::new();
        reader.read_to_string(&mut buf).unwrap();
        let mut stones = HashMap::new();
        buf.split_whitespace()
            .into_iter()
            .map(|str| Stone {
                number: str.parse().unwrap(),
            })
            .for_each(|stone| *stones.entry(stone).or_insert(0) += 1);
        Self { stones }
    }
}
fn read_input(name: &str) -> BufReader<File> {
    let mut example_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    example_data.push(format!("resources/{name}"));

    BufReader::new(File::open(example_data).unwrap())
}
#[cfg(test)]
mod tests {
    use crate::{read_input, Grid, Stone};
    #[test]
    fn test_helpers() {
        assert_eq!(Stone::digits_count(11221), 5);
        assert_eq!(Stone::digits_count(125), 3);
        assert_eq!(Stone::digits_count(253000), 6);
        assert_eq!(Stone::digits_count(1000), 4);

        {
            let stone = Stone { number: 253000 }.blink();
            assert_eq!(stone, (Stone { number: 253 }, Some(Stone { number: 0 })));
        }
        {
            let stone = Stone { number: 512072 }.blink();
            assert_eq!(stone, (Stone { number: 512 }, Some(Stone { number: 72 })));
        }
        {
            let stone = Stone { number: 14168 }.blink();
            assert_eq!(stone, (Stone { number: 28676032 }, None));
        }
    }
    #[test]
    fn test_part1() {
        let _reader = read_input("example-input.txt");
        let mut grid = Grid::from(_reader);
        assert_eq!(grid.stone_count(), 5);
        for _ in 0..1 {
            grid.blink();
        }
        assert_eq!(grid.stone_count(), 7);
        let mut grid = Grid::from("125 17".as_bytes());
        for _ in 0..6 {
            grid.blink();
        }
        assert_eq!(grid.stone_count(), 22);
        let mut grid = Grid::from("125 17".as_bytes());
        for _ in 0..25 {
            grid.blink();
        }
        assert_eq!(grid.stone_count(), 55312);
    }
    #[test]
    fn test_part2() {
        let _reader = read_input("example-input.txt");
    }
}
