#![feature(unsigned_signed_diff)]

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn main() {
    let mut puzzle = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    puzzle.push("resources/puzzle-input.txt");
    let calculated = multiplier(&mut BufReader::new(File::open(puzzle.clone()).unwrap()));
    let part2 = multiplier_part2(&mut BufReader::new(File::open(puzzle).unwrap()));
    println!("Calculated: {}", calculated);
    println!("Conditional: {}", part2);
}
pub fn multiplier<Reader>(reader: &mut Reader) -> i64
where
    Reader: ?Sized + BufRead,
{
    let mut buffer = String::new();
    match reader.read_to_string(&mut buffer) {
        Ok(_) => compute(&buffer),
        Err(e) => panic!("{}", e),
    }
}

fn compute(buffer: &str) -> i64 {
    let reg = Regex::new(r"mul\((?<arg1>[[:digit:]]+),(?<arg2>[[:digit:]]+)\)").unwrap();
    reg.captures_iter(&buffer)
        .map(|caps| {
            (
                caps.name("arg1").unwrap().as_str().parse::<i64>().unwrap(),
                caps.name("arg2").unwrap().as_str().parse::<i64>().unwrap(),
            )
        })
        .map(|(arg1, arg2)| arg1 * arg2)
        .sum()
}

pub fn multiplier_part2<Reader>(reader: &mut Reader) -> i64
where
    Reader: ?Sized + BufRead,
{
    let mut buffer = String::new();
    match reader.read_to_string(&mut buffer) {
        Err(e) => panic!("{}", e),
        Ok(_) => {}
    }
    {
        let cleared = Regex::new(r"(?:don't\(\))([\s\S]+?)(?:do\(\))")
            .unwrap()
            .replace_all(&buffer, "");
        // dbg!(&cleared);
        compute(&cleared)
    }
}
#[cfg(test)]
mod tests {
    use crate::{multiplier, multiplier_part2};
    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;

    #[test]
    fn test_part1() {
        let mut example_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        example_data.push("resources/example-input.txt");
        assert_eq!(
            161,
            multiplier(&mut BufReader::new(File::open(example_data).unwrap()))
        );
    }
    #[test]
    fn test_part2() {
        let mut example_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        example_data.push("resources/example-input2.txt");
        assert_eq!(
            48,
            multiplier_part2(&mut BufReader::new(File::open(example_data).unwrap()))
        );
    }
}
