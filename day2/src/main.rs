#![feature(unsigned_signed_diff)]

use std::cmp::{max, min};
use std::collections::HashSet;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

fn main() {
    let mut example_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    example_data.push("resources/puzzle-input.txt");
    let data: Vec<Vec<i16>> = parse(&mut BufReader::new(File::open(example_data).unwrap()));
    let with_dempener = log_input(&data);
    println!("Safe reports: {}, expected 306",safe_reports(&data));
    println!("With Dampener: {}, expected 366",with_dempener);
}

pub fn parse<Reader, Type>(reader: &mut Reader) -> Vec<Vec<Type>>
where
    Reader: ?Sized + BufRead,
    Type: FromStr,
    <Type as FromStr>::Err: Debug,
{
    reader
        .lines()
        .into_iter()
        .filter_map(|opt| opt.ok())
        .map(|line| {
            let split = line.split_whitespace();
            split.into_iter().map(|val| val.parse().unwrap()).collect()
        })
        .collect()
}
fn is_safe_report(input: &[i16]) -> bool
{
    let mut signum = 0;
    for diff in input.windows(2)
        .map(|w| w[0]-w[1]) {
        if signum == 0{signum = diff.signum()}
        if signum == diff.signum() && diff.abs() >= 1 && diff.abs() <= 3 {
            continue
        }
        return false
    }
    true
}
fn unsafe_level(input: &[i16]) -> Option<usize> {
    let mut signums = HashSet::new();
    for (idx,diff) in input.windows(2)
        .map(|w| w[1]-w[0]).enumerate() {
        signums.insert(diff.signum());
        if (signums.contains(&1) && signums.contains(&-1)) ||
            1 > diff.abs() ||
            diff.abs() > 3 {
            return Some(idx)
        }
    }
    None
}

fn safe_with_dampener(values: &[i16]) -> bool {
    match unsafe_level(values) {
        None => true,
        Some(idx) => {
            (idx.checked_sub(1).unwrap_or(0)..min(idx+1, values.len()-1)+1)
            .any(|idx|{
                let vals = values.iter().enumerate()
                    .filter_map(|(i, val)| {
                        if i==idx { return None }
                        Some(*val)
                    }).collect::<Vec<_>>();
                unsafe_level(&vals).is_none()
            })
        }
    }
}

fn log_input(vals: &Vec<Vec<i16>>) -> usize {
    for row in vals {
        let safe = safe_with_dampener(&row);
        println!("{} -> {safe}", row.iter().fold("".to_string(), |out, val| format!("{out} {val}")))
    }
    vals.iter().filter(|row| safe_with_dampener(row)).count()
}
pub fn safe_reports(input: &Vec<Vec<i16>>) -> usize {
    input.iter().filter(|row| is_safe_report(row)).count()
}
#[cfg(test)]
mod tests {
    use crate::{safe_with_dampener, parse, safe_reports};
    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;

    #[test]
    fn test_safe_reports() {
        let mut example_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        example_data.push("resources/example-input.txt");
        let  data: Vec<Vec<i16>> = parse(&mut BufReader::new(File::open(example_data).unwrap()));
        assert_eq!(2,safe_reports(&data));
    }
    #[test]
    fn test_dampened() {
        let mut example_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        example_data.push("resources/example-input.txt");
        let data: Vec<Vec<i16>> = parse(&mut BufReader::new(File::open(example_data).unwrap()));
        let with_dempener =
            data.iter().filter(|row| safe_with_dampener(row.as_slice())).count();
        assert_eq!(4,with_dempener);
    }
}
