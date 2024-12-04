#![feature(unsigned_signed_diff)]

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn main() {
    let _reader = read_input("puzzle-input.txt");
    todo!();
}
pub fn parse<Reader>(_reader: &mut Reader)
where
    Reader: ?Sized + BufRead,
{
    todo!()
}

fn read_input(name: &str) -> BufReader<File> {
    let mut example_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    example_data.push(format!("resources/{name}"));

    BufReader::new(File::open(example_data).unwrap())
}
#[cfg(test)]
mod tests {

    use crate::read_input;

    #[test]
    fn test_part1() {
        let _reader = read_input("example-input.txt");
        todo!();
    }
    #[test]
    fn test_part2() {
        let _reader = read_input("example-input.txt");
        todo!();
    }
}
