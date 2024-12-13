#![feature(unsigned_signed_diff)]

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::path::PathBuf;
use std::str::FromStr;
fn main() {
    let _reader = read_input("puzzle-input.txt");
    let calibrations = parse(_reader);
    println!(
        "Total calibration result: {}",
        total_calibration_result(calibrations.iter().cloned().collect())
    );
    println!(
        "Total calibration result part2: {}",
        total_calibration_result_part2(calibrations)
    );
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

#[derive(Debug, Clone)]
pub struct ParserError {
    #[allow(dead_code)]
    info: String,
}
impl From<ParseIntError> for ParserError {
    fn from(err: ParseIntError) -> Self {
        ParserError {
            info: err.to_string(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct CalibrationEquation {
    test_value: u64,
    values: Vec<u64>,
}
fn evaluate_operator(operator: &Operator, left: u64, right: u64) -> u64 {
    match operator {
        Operator::Add => left + right,
        Operator::Multiply => left * right,
        Operator::Concatenate => format!("{}{}", left, right).parse::<u64>().unwrap(),
    }
}
fn test_operators(values: Option<(&[u64], &[u64])>, test_value: u64, result: u64) -> Option<()> {
    if result > test_value {
        return None;
    }
    if let Some((_, values)) = values {
        if values.len() == 0 {
            if result == test_value {
                return Some(());
            }
            return None;
        }
        for operator in vec![Operator::Add, Operator::Multiply].into_iter() {
            if result == 0 {
                let result = evaluate_operator(&operator, values[0], values[1]);
                match test_operators(values.split_at_checked(2), test_value, result) {
                    Some(()) => return Some(()),
                    None => continue,
                }
            } else {
                let result = evaluate_operator(&operator, result, values[0]);
                match test_operators(values.split_at_checked(1), test_value, result) {
                    Some(()) => return Some(()),
                    None => continue,
                }
            }
        }
    }
    None
}

fn test_operators_part2(
    values: Option<(&[u64], &[u64])>,
    test_value: u64,
    result: u64,
) -> Option<()> {
    if result > test_value {
        return None;
    }
    if let Some((_, values)) = values {
        if values.len() == 0 {
            if test_value == result {
                return Some(());
            }
            return None;
        }
        for operator in vec![Operator::Add, Operator::Multiply, Operator::Concatenate].into_iter() {
            if result == 0 {
                let result = evaluate_operator(&operator, values[0], values[1]);
                match test_operators_part2(values.split_at_checked(2), test_value, result) {
                    Some(()) => return Some(()),
                    None => continue,
                }
            } else {
                let result = evaluate_operator(&operator, result, values[0]);
                match test_operators_part2(values.split_at_checked(1), test_value, result) {
                    Some(()) => return Some(()),
                    None => continue,
                }
            }
        }
    }
    None
}

impl CalibrationEquation {
    pub fn can_construct_equation(&self) -> bool {
        test_operators(Some((&self.values, &self.values)), self.test_value, 0).is_some()
    }
    pub fn can_construct_equation_part2(&self) -> bool {
        test_operators_part2(Some((&self.values, &self.values)), self.test_value, 0).is_some()
    }
}

pub fn total_calibration_result(calibration: Vec<CalibrationEquation>) -> u64 {
    calibration
        .into_iter()
        .filter(|c| c.can_construct_equation())
        .map(|c| c.test_value)
        .sum()
}
pub fn total_calibration_result_part2(calibration: Vec<CalibrationEquation>) -> u64 {
    calibration
        .into_iter()
        .filter(|c| c.can_construct_equation_part2())
        .map(|c| c.test_value)
        .sum()
}
impl FromStr for CalibrationEquation {
    type Err = ParserError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.split_once(':') {
            None => Err(ParserError {
                info: format!("Missing : in '{:?}'", input),
            }),
            Some((test_val, vals)) => Ok(Self {
                test_value: test_val.parse::<u64>()?,
                values: vals
                    .split_whitespace()
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>(),
            }),
        }
    }
}
pub fn parse<Reader>(_reader: Reader) -> Vec<CalibrationEquation>
where
    Reader: BufRead,
{
    _reader
        .lines()
        .map_while(|line| line.ok())
        .map(|line| CalibrationEquation::from_str(line.trim()).unwrap())
        .collect()
}

fn read_input(name: &str) -> BufReader<File> {
    let mut example_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    example_data.push(format!("resources/{name}"));

    BufReader::new(File::open(example_data).unwrap())
}
#[cfg(test)]
mod tests {

    use crate::{
        Operator, evaluate_operator, parse, read_input, test_operators, test_operators_part2,
        total_calibration_result, total_calibration_result_part2,
    };

    #[test]
    fn test_operators_1() {
        let values = vec![81, 40, 27];
        assert_eq!(test_operators(Some((&values, &values)), 3267, 0), Some(()));
        let values = vec![6, 8, 6, 15];
        assert_eq!(
            test_operators_part2(Some((&values, &values)), 7290, 0),
            Some(())
        );
        let values = vec![17, 8, 14];
        assert_eq!(
            test_operators_part2(Some((&values, &values)), 192, 0),
            Some(())
        );
    }
    #[test]
    fn test_part1() {
        let _reader = read_input("example-input.txt");
        let calibrations = parse(_reader);
        assert_eq!(total_calibration_result(calibrations), 3749)
    }
    #[test]
    fn test_part2() {
        let _reader = read_input("example-input.txt");
        let calibrations = parse(_reader);
        assert_eq!(evaluate_operator(&Operator::Concatenate, 10, 1), 101);
        assert_eq!(total_calibration_result_part2(calibrations), 11387)
    }
}
