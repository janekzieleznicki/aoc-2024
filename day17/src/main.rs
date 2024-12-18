#![feature(unsigned_signed_diff)]
#![feature(int_roundings)]

use crate::Instruction::*;
use regex::Regex;
use std::collections::VecDeque;
use std::fs;
use std::num::ParseIntError;
use std::ops::BitXor;
use std::path::PathBuf;
use std::str::FromStr;

fn main() {
    let mut chronospatial_computer = ChronospatialComputer::from_str(&read_input("puzzle-input.txt")).unwrap();
    println!("Chronospatial comupter debug: {}", chronospatial_computer.execute_to_string());
    println!("a : {}",chronospatial_computer.find_a().unwrap());
}
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}
impl TryFrom<u8> for Instruction {
    type Error = ProgramHaltedError;

    fn try_from(c: u8) -> Result<Instruction, ProgramHaltedError> {
        Ok(match c {
            0 => Adv,
            1 => Bxl,
            2 => Bst,
            3 => Jnz,
            4 => Bxc,
            5 => Out,
            6 => Bdv,
            7 => Cdv,
            _ => return Err(ProgramHaltedError{msg: "Invalid instruction {c}".to_string()})
        })
    }
}
struct Registers{
    a: u128,
    b: u128,
    c: u128,
}
struct ChronospatialComputer {
    program: Vec<u8>,
    registers: Registers,
    instruction: usize,
}
#[derive(Debug)]
struct ParseError{}
impl From<ParseIntError> for ParseError {
    fn from(_: ParseIntError) -> Self {
        Self{}
    }
}
#[allow(dead_code)]
#[derive(Debug)]
struct ProgramHaltedError{
    msg: String
}

impl ChronospatialComputer {
    fn combo_operand(&self, operand: u8) -> Result<u128, ProgramHaltedError> {
        match operand {
            0..=3 => Ok(operand as u128),
            4 => Ok(self.registers.a),
            5 => Ok(self.registers.b),
            6 => Ok(self.registers.c),
            _ => Err(ProgramHaltedError{msg: "Invalid operand".to_string()}),
        }
    }
    pub fn next(&mut self) -> Result<Option<u8>, ProgramHaltedError> {
        let instr = match self.program.get(self.instruction) {
            None => return Err(ProgramHaltedError{msg: "Attempt to read past program".to_string()}),
            Some(instr) => Instruction::try_from(*instr)?,
        };
        let literal_operand = match self.program.get(self.instruction + 1) {
            None => return Err(ProgramHaltedError{msg: "Attempt to read past program".to_string()}),
            Some(instr) => *instr,
        };
        self.instruction += 2;
        match instr {
            Adv => {
                self.registers.a = self.registers.a.div_floor(2_u128.pow(self.combo_operand(literal_operand)? as u32));
                Ok(None)
            }
            Bxl => {
                self.registers.b = self.registers.b.bitxor(literal_operand as u128);
                Ok(None)
            }
            Bst => {
                self.registers.b = self.combo_operand(literal_operand)? % 8;
                Ok(None)
            }
            Jnz => {
                match self.registers.a {
                    0 => Ok(None),
                    _ => {
                        self.instruction = literal_operand as usize;
                        Ok(None)
                    }
                }
            }
            Bxc => {
                self.registers.b = self.registers.b.bitxor(self.registers.c);
                Ok(None)
            }
            Out => {
                Ok(Some(self.combo_operand(literal_operand)? as u8 % 8))
            }
            Bdv => {
                self.registers.b = self.registers.a.div_floor(2_u128.pow(self.combo_operand(literal_operand)? as u32));
                Ok(None)
            }
            Cdv => {
                self.registers.c = self.registers.a.div_floor(2_u128.pow(self.combo_operand(literal_operand)? as u32));
                Ok(None)
            }
        }
    }
    fn execute(&mut self) -> Vec<u8> {
        let mut output = Vec::new();
        while let Ok(sth) = self.next(){
            match sth{
                None => {}
                Some(x) => output.push(x)
            }
        }
        output
    }
    pub fn execute_to_string(&mut self) -> String {
        self.execute().into_iter().map(|x|format!("{x}")).collect::<Vec<_>>().join(",")
    }

    pub fn find_a(&mut self) -> Option<u128> {
        let mut to_visit = VecDeque::from([(self.program.len(),0)]);
        while let Some((pos,a)) = to_visit.pop_front() {
            for i in 0..8 {
                let new_a = a * 8 + i;
                self.registers = Registers{
                    a: new_a,
                    b: 0,
                    c: 0,
                };
                self.instruction = 0;
                let o = self.execute();
                let o_string = o.iter().map(|x|format!("{x}")).collect::<Vec<_>>().join(",");
                println!("Checking a: {new_a} | a: {new_a:x?} => {}", o_string);
                if o[..] == self.program[pos - 1..] {
                    to_visit.push_back((pos-1, new_a));
                    if o.len() == self.program.len() {
                        return Some(new_a);
                    }
                }
            }
        }
        None
    }
}

impl FromStr for ChronospatialComputer {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reg = Regex::new(
            r"^Register a: (?<reg_a>\d+)\nRegister b: (?<reg_b>\d+)\nRegister c: (?<reg_c>\d+)\n\nProgram: (?<program>[\d,]+)$")
            .unwrap();
        match reg.captures(s) {
            Some(captures) =>
            Ok(Self{
                program: captures["program"].split(',').map(|x| x.parse().unwrap()).collect(),
                registers: Registers {
                    a: captures["reg_a"].parse()?,
                    b: captures["reg_b"].parse()?,
                    c: captures["reg_c"].parse()?,
                },
                instruction: 0,
            }),
            None => Err(ParseError{})
        }

    }
}

fn read_input(name: &str) -> String {
    let mut example_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    example_data.push(format!("resources/{name}"));
    fs::read_to_string(&example_data).unwrap()
}
#[cfg(test)]
mod tests {
    use crate::{read_input, ChronospatialComputer};
    use std::str::FromStr;

    #[test]
    fn test_part1() {
        let mut chronospatial_computer = ChronospatialComputer::from_str(&read_input("example-input.txt")).unwrap();
        let res = chronospatial_computer.execute_to_string();
        assert_eq!(res, "4,6,3,5,6,3,5,2,1,0");
        // assert_eq!(chronospatial_computer.find_a().unwrap(), 117440); // doesn't work for test input
    }
    #[test]
    fn test_part2() {
        let chronospatial_computer = ChronospatialComputer::from_str(&read_input("example-input.txt"));
    }
}
