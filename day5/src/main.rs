#![feature(unsigned_signed_diff)]

use regex::Regex;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

fn main() {
    let reader = read_input("puzzle-input.txt");
    let print_queue = PrintQueue::from(reader);
    println!("Sum of middle pages: {}", print_queue.middle_pages());
    println!(
        "Sum of incorrect pages: {}",
        print_queue.incorrect_updates()
    );
}
struct Rule {
    first: usize,
    latter: usize,
}

impl FromStr for Rule {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('|');
        Ok(Self {
            first: split.next().unwrap().parse()?,
            latter: split.next().unwrap().parse()?,
        })
    }
}
struct PrintQueue {
    rules: Vec<Rule>,
    prints: Vec<Vec<usize>>,
    must_print_before: HashMap<usize, HashSet<usize>>,
    must_print_after: HashMap<usize, HashSet<usize>>,
}
impl PrintQueue {
    fn new(rules: Vec<Rule>, prints: Vec<Vec<usize>>) -> PrintQueue {
        let mut pq = PrintQueue {
            rules,
            prints,
            must_print_before: HashMap::new(),
            must_print_after: HashMap::new(),
        };
        for rule in &mut pq.rules {
            pq.must_print_before
                .entry(rule.first)
                .or_default()
                .insert(rule.latter);
            pq.must_print_after
                .entry(rule.latter)
                .or_default()
                .insert(rule.first);
        }
        pq
    }
    pub fn violates_rule(&self, pages: &Vec<usize>) -> bool {
        for (idx, page) in pages.iter().enumerate() {
            let printed = &pages[..idx];
            let to_print = &pages[idx + 1..];
            match self.must_print_before.get(page) {
                None => {}
                Some(ensure_after) => {
                    if printed.iter().any(|&page| ensure_after.contains(&page)) {
                        return false;
                    }
                }
            }
            match self.must_print_after.get(page) {
                None => {}
                Some(ensure_before) => {
                    if to_print.iter().any(|&page| ensure_before.contains(&page)) {
                        return false;
                    }
                }
            }
        }
        true
    }
    pub fn middle_pages(&self) -> usize {
        self.prints
            .iter()
            .filter(|print| self.violates_rule(print))
            .map(|print| print[print.len() / 2])
            .sum()
    }
    fn order_two_pages(&self, left: &usize, right: &usize) -> core::cmp::Ordering {
        match self.must_print_before.get(&left) {
            Some(ensure_after) => {
                if ensure_after.contains(&right) {
                    return Less;
                }
            }
            None => {}
        }
        match self.must_print_after.get(&left) {
            Some(ensure_before) => {
                if ensure_before.contains(&right) {
                    return Greater;
                }
            }
            None => {}
        }
        match self.must_print_before.get(&right) {
            Some(ensure_after) => {
                if ensure_after.contains(&left) {
                    return Less;
                }
            }
            None => {}
        }
        match self.must_print_after.get(&right) {
            Some(ensure_before) => {
                if ensure_before.contains(&left) {
                    return Greater;
                }
            }
            None => {}
        }
        Equal
    }
    fn order_rule(&self, mut pages: Vec<usize>) -> Vec<usize> {
        pages.sort_unstable_by(|left, right| self.order_two_pages(left, right));
        pages
    }
    pub fn incorrect_updates(&self) -> usize {
        self.prints
            .iter()
            .filter(|print| !self.violates_rule(print))
            .cloned()
            .map(|print| self.order_rule(print))
            .map(|print| print[print.len() / 2])
            .sum()
    }
}
impl<Reader> From<Reader> for PrintQueue
where
    Reader: BufRead,
{
    fn from(reader: Reader) -> Self {
        let mut rules = Vec::new();
        let mut pages = Vec::new();
        let re = Regex::new(r"(?<rules>\d+\|\d+$)|(?<pages>(\d+(,\d+))+$)").unwrap();
        for line in reader.lines().filter_map(Result::ok) {
            if let Some(caps) = re.captures(&line) {
                if let Some(rule) = caps.name("rules") {
                    rules.push(Rule::from_str(rule.as_str()).unwrap());
                }
                if let Some(pages_match) = caps.name("pages") {
                    pages.push(
                        pages_match
                            .as_str()
                            .split(",")
                            .into_iter()
                            .filter_map(|page| page.parse().ok())
                            .collect::<Vec<usize>>(),
                    );
                }
            }
        }
        PrintQueue::new(rules, pages)
    }
}

fn read_input(name: &str) -> BufReader<File> {
    let mut example_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    example_data.push(format!("resources/{name}"));

    BufReader::new(File::open(example_data).unwrap())
}
#[cfg(test)]
mod tests {
    use crate::{PrintQueue, read_input};
    use std::cmp::Ordering::{Greater};

    #[test]
    fn test_part1() {
        let reader = read_input("example-input.txt");
        let print_queue = PrintQueue::from(reader);
        assert_eq!(print_queue.middle_pages(), 143);
    }
    #[test]
    fn test_part2() {
        let reader = read_input("example-input.txt");
        let print_queue = PrintQueue::from(reader);
        assert_eq!(print_queue.order_two_pages(&75, &97), Greater);
        assert_eq!(print_queue.order_two_pages(&13, &29), Greater);
        assert_eq!(print_queue.order_two_pages(&13, &75), Greater);
        assert_eq!(print_queue.order_rule(vec![75, 97, 47, 61, 53]), vec![
            97, 75, 47, 61, 53
        ]);
        assert_eq!(print_queue.order_rule(vec![61, 13, 29]), vec![61, 29, 13]);
        assert_eq!(print_queue.order_rule(vec![97, 13, 75, 29, 47]), vec![
            97, 75, 47, 29, 13
        ]);
        assert_eq!(print_queue.incorrect_updates(), 123);
    }
}
