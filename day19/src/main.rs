#![feature(unsigned_signed_diff)]

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn main() {
    let _reader = read_input("puzzle-input.txt");
    let onsen = Onsen::from(_reader);
    println!(
        "Possible patterns count: {}",
        onsen.possible_patterns().len()
    );
    println!("All combinations: {}", onsen.all_combinations());
}

#[derive(Debug)]
struct Onsen {
    towels: Vec<String>,
    displays: Vec<String>,
}
impl Onsen {
    fn all_combinations(&self) -> usize {
        fn recur<'input>(
            cache: &mut HashMap<&'input str, usize>,
            towels: &[String],
            pattern: &'input str,
        ) -> usize {
            match pattern {
                "" => 1,
                _ => match cache.get(pattern) {
                    Some(&count) => count,
                    None => {
                        let count = towels
                            .iter()
                            .filter_map(|t| pattern.strip_prefix(t))
                            .fold(0, |acc, rest| acc + recur(cache, towels, rest));
                        cache.insert(pattern, count);
                        count
                    }
                },
            }
        }
        let mut cache = HashMap::new();
        self.displays
            .iter()
            .map(|pattern| recur(&mut cache, &self.towels, pattern))
            .sum()
    }
    fn possible_patterns(&self) -> Vec<String> {
        let regex = format!("^({})+$", self.towels.join("|"));
        let regex = Regex::new(&regex).unwrap();
        self.displays
            .iter()
            .filter(|s| regex.is_match(&s))
            .cloned()
            .collect()
    }
}
impl<Reader> From<Reader> for Onsen
where
    Reader: BufRead,
{
    fn from(mut reader: Reader) -> Self {
        let mut string = String::new();
        reader.read_to_string(&mut string).unwrap();
        let towels_reg = Regex::new(r"^(?<towels>(\w+(,\s)*)+)$").unwrap();
        match string.split_once("\n\n") {
            Some((towels, patterns)) => Self {
                towels: match towels_reg.captures(towels) {
                    None => panic!("invalid towels"),
                    Some(towels) => towels["towels"]
                        .split(',')
                        .map(|s| s.trim_start().to_string())
                        .collect(),
                },
                displays: patterns.lines().map(|l| l.to_string()).collect(),
            },
            None => panic!("Unable to parse input"),
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
    use crate::{read_input, Onsen};

    #[test]
    fn test_part1() {
        let _reader = read_input("example-input.txt");
        let onsen = Onsen::from(_reader);
        assert_eq!(onsen.possible_patterns().len(), 6)
    }
    #[test]
    fn test_part2() {
        let _reader = read_input("example-input.txt");
        let onsen = Onsen::from(_reader);
        assert_eq!(onsen.all_combinations(), 16)
    }
}
