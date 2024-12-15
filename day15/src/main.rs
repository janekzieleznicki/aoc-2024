#![feature(unsigned_signed_diff)]

use regex::Regex;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::path::PathBuf;

fn main() {
    let reader = read_input("puzzle-input.txt");
    let mut warehouse = Warehouse::from(reader);
    // println!("Warehous:\n{}", warehouse);
    while let Some(_) = warehouse.move_robot() {}
    // println!("Finished: \n{}", warehouse);
    println!("GPS sum: {}", warehouse.gps_sum());
    let mut wide_warehouse = WideWarehouse::from(&Warehouse::from(read_input("puzzle-input.txt")));
    while let Some(_) = wide_warehouse.move_robot() {}
    println!("Wide warehouse GPS sum: {}", wide_warehouse.gps_sum());
}
#[derive(Clone, Copy, Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
enum Movement {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Movement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(match self {
            Movement::Left => write!(f, "<")?,
            Movement::Right => write!(f, ">")?,
            Movement::Up => write!(f, "^")?,
            Movement::Down => write!(f, "v")?,
        })
    }
}
impl Add<Movement> for Position {
    type Output = Self;
    fn add(self, rhs: Movement) -> Self {
        match rhs {
            Movement::Up => Self {
                x: self.x,
                y: self.y - 1,
            },
            Movement::Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Movement::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
            Movement::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}
#[derive(Debug, Eq, PartialEq)]
enum Space {
    Empty,
    Wall,
    Obstacle,
    Robot,
}
impl Display for Space {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(match self {
            Space::Empty => write!(f, ".")?,
            Space::Wall => write!(f, "#")?,
            Space::Obstacle => write!(f, "O")?,
            Space::Robot => write!(f, "@")?,
        })
    }
}
#[derive(Debug, Clone)]
struct Robot {
    position: Position,
    moves: VecDeque<Movement>,
}
impl<Reader> From<Reader> for Robot
where
    Reader: BufRead,
{
    fn from(mut value: Reader) -> Self {
        let mut input = String::new();
        value.read_to_string(&mut input).unwrap();
        Self {
            position: Position { x: 0, y: 0 },
            moves: input
                .chars()
                .into_iter()
                .filter_map(|c| match c {
                    '<' => Some(Movement::Left),
                    '>' => Some(Movement::Right),
                    'v' => Some(Movement::Down),
                    '^' => Some(Movement::Up),
                    _ => None,
                })
                .collect(),
        }
    }
}
struct Warehouse {
    spaces: Vec<Space>,
    size: Position,
    robot: Robot,
}

impl Warehouse {
    fn at(&self, position: Position) -> Option<&Space> {
        self.spaces.get(position.y * (self.size.x) + position.x)
    }
    fn set(&mut self, position: Position, space: Space) {
        self.spaces[position.y * (self.size.x) + position.x] = space;
    }
    fn try_move(
        &mut self,
        position: Position,
        movement: Movement,
        new_object: Space,
    ) -> Option<Position> {
        let new_pos = position + movement;
        match self.at(new_pos) {
            Some(Space::Wall) => None,
            Some(Space::Empty) => {
                self.set(new_pos, new_object);
                self.set(position, Space::Empty);
                Some(new_pos)
            }
            Some(Space::Obstacle) => match self.try_move(new_pos, movement, Space::Obstacle) {
                Some(_) => {
                    self.set(new_pos, new_object);
                    self.set(position, Space::Empty);
                    Some(new_pos)
                }
                None => None,
            },
            x => unreachable!("{x:?} at {new_pos:?}"),
        }
    }
    pub fn move_robot(&mut self) -> Option<Position> {
        match self.robot.moves.pop_front() {
            Some(movement) => {
                // println!("Move: {movement}");
                match self.try_move(self.robot.position, movement, Space::Robot) {
                    Some(position) => {
                        self.robot.position = position;
                    }
                    None => {}
                }
                Some(self.robot.position)
            }
            None => None,
        }
    }
    fn where_robot(mut self) -> Self {
        let idx = self
            .spaces
            .iter()
            .position(|space| match space {
                Space::Robot => true,
                _ => false,
            })
            .unwrap();
        self.robot.position = Position {
            x: idx % self.size.x,
            y: idx / self.size.x,
        };
        assert_eq!(self.at(self.robot.position), Some(&Space::Robot));
        self
    }

    pub fn gps_sum(&self) -> usize {
        let mut sum = 0;
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                match self.at(Position { x, y }) {
                    Some(Space::Obstacle) => sum += 100 * y + x,
                    _ => continue,
                }
            }
        }
        sum
    }
}
impl Display for Warehouse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                write!(f, "{}", self.at(Position { x, y }).unwrap())?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
impl<Reader> From<Reader> for Warehouse
where
    Reader: BufRead,
{
    fn from(mut reader: Reader) -> Self {
        let reg = Regex::new(r"(?<map>#[\w\W]*#)\n\n(?<movements>[<>^v\n]+)").unwrap();
        let mut input = String::new();
        reader.read_to_string(&mut input).unwrap();
        match reg.captures(&input) {
            None => panic!("input didn't match format"),
            Some(captures) => Self {
                spaces: captures["map"]
                    .chars()
                    .filter_map(|c| match c {
                        '#' => Some(Space::Wall),
                        '.' => Some(Space::Empty),
                        'O' => Some(Space::Obstacle),
                        '@' => Some(Space::Robot),
                        _ => None,
                    })
                    .collect(),
                size: Position {
                    x: captures["map"].lines().take(1).map(|line| line.len()).sum(),
                    y: captures["map"].lines().count(),
                },
                robot: Robot::from(captures["movements"].as_bytes()),
            }
            .where_robot(),
        }
    }
}
struct WideWarehouse {
    map: Vec<char>,
    robot: Robot,
    size: Position,
}
#[derive(Debug)]
struct WallError {}
impl WideWarehouse {
    fn at(&self, position: Position) -> Option<char> {
        Some(self.map[position.y * (self.size.x) + position.x])
    }
    fn set(&mut self, position: Position, char: char) {
        self.map[position.y * (self.size.x) + position.x] = char;
    }

    fn try_move(
        &mut self,
        position: Position,
        movement: Movement,
    ) -> Result<Vec<Position>, WallError> {
        let new_pos = position + movement;
        match (self.at(new_pos), movement) {
            (Some('#'), _) => Err(WallError {}),
            (Some('.'), _) => Ok(vec![position]),
            (Some('[') | Some(']'), Movement::Right | Movement::Left) => {
                match self.try_move(new_pos, movement) {
                    Ok(mut vec) => {
                        vec.push(position);
                        Ok(vec)
                    }
                    Err(e) => Err(e),
                }
            }
            (Some('['), Movement::Up | Movement::Down) => {
                let mut vec = Vec::new();
                vec.append(self.try_move(new_pos, movement)?.as_mut());
                vec.append(
                    self.try_move(
                        Position {
                            x: new_pos.x + 1,
                            y: new_pos.y,
                        },
                        movement,
                    )?
                    .as_mut(),
                );
                vec.push(position);
                Ok(vec)
            }

            (Some(']'), Movement::Up | Movement::Down) => {
                let mut vec = Vec::new();
                vec.append(self.try_move(new_pos, movement)?.as_mut());
                vec.append(
                    self.try_move(
                        Position {
                            x: new_pos.x - 1,
                            y: new_pos.y,
                        },
                        movement,
                    )?
                    .as_mut(),
                );
                vec.push(position);
                Ok(vec)
            }
            x => unreachable!("{x:?} at {new_pos:?}"),
        }
    }
    pub fn move_robot(&mut self) -> Option<Position> {
        match self.robot.moves.pop_front() {
            Some(movement) => {
                // println!("Move: {movement}");
                match self.try_move(self.robot.position, movement) {
                    Ok(vec) => {
                        let old_pos = vec
                            .into_iter()
                            .map(|pos| (pos, self.at(pos).unwrap()))
                            .collect::<Vec<_>>();
                        // clear old pos
                        for (old_pos, _) in old_pos.iter() {
                            self.set(*old_pos, '.');
                        }
                        // put objects into new positions
                        for (new_pos, new_char) in old_pos
                            .into_iter()
                            .map(|(pos, char)| (pos + movement, char))
                        {
                            self.set(new_pos, new_char);
                        }
                        // update robot location
                        self.where_robot();
                        Some(self.robot.position)
                    }
                    Err(_) => Some(self.robot.position),
                }
            }
            None => None,
        }
    }
    fn where_robot(&mut self) {
        let idx = self
            .map
            .iter()
            .position(|space| match space {
                '@' => true,
                _ => false,
            })
            .unwrap();
        self.robot.position = Position {
            x: idx % self.size.x,
            y: idx / self.size.x,
        };
        // x: 11 y:4
        assert_eq!(self.at(self.robot.position).unwrap(), '@');
    }
    pub fn gps_sum(&self) -> usize {
        let mut sum = 0;
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                match self.at(Position { x, y }) {
                    /* Treat right border as edge of map
                     Some('[') if x < self.size.x/2 => sum += 100 * min(y,self.size.y-y-1) + x,
                     Some(']') if x > self.size.x/2 => sum += 100 * min(y,self.size.y-y-1) + (self.size.x-x-1),
                    */
                    Some('[') => sum += 100 * y + x,
                    _ => continue,
                }
            }
        }
        sum
    }
}
impl From<&Warehouse> for WideWarehouse {
    fn from(warehouse: &Warehouse) -> Self {
        let map = format!("{warehouse}");
        let size = Position {
            x: warehouse.size.x * 2,
            y: warehouse.size.y,
        };
        let mut new_robot = warehouse.robot.clone();
        new_robot.position.x = warehouse.robot.position.x * 2;
        Self {
            map: map
                .lines()
                .into_iter()
                .map(|line| {
                    line.chars()
                        .map(|char| match char {
                            '#' => vec!['#', '#'],
                            'O' => vec!['[', ']'],
                            '.' => vec!['.', '.'],
                            '@' => vec!['@', '.'],
                            _ => unreachable!(),
                        })
                        .flatten()
                        .collect::<Vec<char>>()
                })
                .flatten()
                .collect(),
            size,
            robot: new_robot,
        }
    }
}
impl Display for WideWarehouse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                write!(f, "{}", self.at(Position { x, y }).unwrap())?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
fn read_input(name: &str) -> BufReader<File> {
    let mut example_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    example_data.push(format!("resources/{name}"));

    BufReader::new(File::open(example_data).unwrap())
}
#[cfg(test)]
mod tests {

    use crate::{Warehouse, WideWarehouse, read_input};

    #[test]
    fn test_small() {
        let reader = read_input("example-input-small.txt");
        let mut warehouse = Warehouse::from(reader);
        while let Some(_) = warehouse.move_robot() {}
        assert_eq!(warehouse.gps_sum(), 2028)
    }
    #[test]
    fn test_part1() {
        let reader = read_input("example-input.txt");
        let mut warehouse = Warehouse::from(reader);
        while let Some(_) = warehouse.move_robot() {}
        assert_eq!(warehouse.gps_sum(), 10092);
    }
    #[test]
    fn test_part2() {
        let reader = read_input("example-input.txt");
        let mut warehouse = WideWarehouse::from(&Warehouse::from(reader));
        // println!("Warehous:\n{}", warehouse);
        while let Some(_) = warehouse.move_robot() {
            // println!("Robot ar {position:?} warehourse:\n{warehouse}")
        }
        // println!("Warehouse:\n{}", warehouse);
        assert_eq!(warehouse.gps_sum(), 9021);
    }
    #[test]
    fn test_part2_2() {
        let input: &str = r#"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"#;
        let mut warehouse = WideWarehouse::from(&Warehouse::from(input.as_bytes()));
        // println!("Warehouse:\n{}", warehouse);
        while let Some(_position) = warehouse.move_robot() {
            // println!("Robot ar {position:?} warehourse:\n{warehouse}")
        }
    }

    #[test]
    fn test_part2_3() {
        let input: &str = r#"#######
#.....#
#.OO@.#
#.....#
#######

<<"#;

        let mut warehouse = WideWarehouse::from(&Warehouse::from(input.as_bytes()));
        // println!("Warehouse:\n{}", warehouse);
        while let Some(_position) = warehouse.move_robot() {
            // println!("Robot at {position:?} warehourse:\n{warehouse}")
        }
        assert_eq!(warehouse.gps_sum(), 406);
    }

    #[test]
    fn test_part2_4() {
        let input: &str = r#"#######
#.....#
#.O#..#
#..O@.#
#.....#
#######

<v<<^"#;
        let mut warehouse = WideWarehouse::from(&Warehouse::from(input.as_bytes()));
        // println!("Warehouse:\n{}", warehouse);
        while let Some(_position) = warehouse.move_robot() {
            // println!("Robot at {position:?} warehourse:\n{warehouse}")
        }
        assert_eq!(warehouse.gps_sum(), 509);
    }
    #[test]
    fn test_part2_5() {
        let input: &str = r#"#######
#.....#
#.#O..#
#..O@.#
#.....#
#######

<v<^"#;
        let mut warehouse = WideWarehouse::from(&Warehouse::from(input.as_bytes()));
        // println!("Warehouse:\n{}", warehouse);
        while let Some(_position) = warehouse.move_robot() {
            // println!("Robot at {position:?} warehourse:\n{warehouse}")
        }
        assert_eq!(warehouse.gps_sum(), 511);
    }
    #[test]
    fn test_part2_6() {
        let input: &str = r#"######
#....#
#.O..#
#.OO@#
#.O..#
#....#
######

<vv<<^"#;
        let mut warehouse = WideWarehouse::from(&Warehouse::from(input.as_bytes()));
        // println!("Warehouse:\n{}", warehouse);
        while let Some(_position) = warehouse.move_robot() {
            // println!("Robot at {position:?} warehourse:\n{warehouse}")
        }
        assert_eq!(warehouse.gps_sum(), 816);
    }
    #[test]
    fn test_part2_7() {
        let input: &str = r#"#######
#...#.#
#.....#
#.....#
#.....#
#.....#
#.OOO@#
#.OOO.#
#..O..#
#.....#
#.....#
#######

v<vv<<^^^^^"#;
        let mut warehouse = WideWarehouse::from(&Warehouse::from(input.as_bytes()));
        // println!("Warehouse:\n{}", warehouse);
        while let Some(_position) = warehouse.move_robot() {
            // println!("Robot at {position:?} warehourse:\n{warehouse}")
        }
        assert_eq!(warehouse.gps_sum(), 2339);
    }
}
