#![feature(unsigned_signed_diff)]

use std::cmp::{Ordering, PartialEq, max};
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::path::PathBuf;
use std::sync::atomic::AtomicUsize;
use std::time::Instant;

fn main() {
    let _reader = read_input("puzzle-input.txt");

    let race_condition = RaceCondition::from(_reader);

    let start = Instant::now();
    let save_count = race_condition.cheat_saves(2);
    println!(
        "There are {} 2 ps cheats that save more than 100 | {:?}",
        save_count
            .iter()
            .filter_map(|(saves, count)| if *saves >= 100 { Some(count) } else { None })
            .sum::<usize>()
        , start.elapsed()
    );

    let start = Instant::now();
    let save_count = race_condition.cheat_saves(20);
    println!(
        "There are {} 20ps cheats that save at least 100 picoseconds | {:?}",
        save_count
            .iter()
            .filter_map(|(saves, count)| if *saves >= 100 { Some(count) } else { None })
            .sum::<usize>()
        , start.elapsed()
    );
}
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Position {
    x: isize,
    y: isize,
}
impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Tile {
    Wall,
    Start,
    End,
    Path,
}
struct RaceCondition {
    map: Vec<Tile>,
    start: Position,
    end: Position,
    size: Position,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Wall => write!(f, "#"),
            Tile::Start => write!(f, "S"),
            Tile::End => write!(f, "E"),
            Tile::Path => write!(f, "."),
        }
    }
}
impl Add<&Position> for &Position {
    type Output = Position;
    fn add(self, rhs: &Position) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
struct EndUnreachable {}
impl RaceCondition {
    fn at(&self, pos: Position) -> Option<&Tile> {
        if pos.x < 0 || pos.y < 0 {
            None
        } else if pos.x > self.size.x || pos.y > self.size.y {
            None
        } else {
            self.map
                .get(pos.y as usize * (self.size.x as usize) + pos.x as usize)
        }
    }
    fn pos_from_index(&self, index: usize) -> Position {
        Position {
            x: index as isize % self.size.x,
            y: index as isize / self.size.x,
        }
    }
    pub fn where_end(mut self) -> Self {
        self.end = self
            .map
            .iter()
            .position(|t| t == &Tile::End)
            .map(|pos| self.pos_from_index(pos))
            .unwrap();
        assert_eq!(self.at(self.end), Some(&Tile::End));
        self
    }
    pub fn where_start(mut self) -> Self {
        self.start = self
            .map
            .iter()
            .rposition(|t| t == &Tile::Start)
            .map(|pos| self.pos_from_index(pos))
            .unwrap();
        assert_eq!(self.at(self.start), Some(&Tile::Start));
        self
    }
    fn next_nodes<T: FnMut(&Tile) -> bool + 'static>(
        &self,
        from: &Position,
        mut cond: T,
    ) -> impl Iterator<Item = Position> {
        const MOVES: [Position; 4] = [
            Position { x: 1, y: 0 },
            Position { x: -1, y: 0 },
            Position { x: 0, y: 1 },
            Position { x: 0, y: -1 },
        ];
        MOVES
            .iter()
            .map(move |vec| from + vec)
            .filter(move |pos| match self.at(*pos) {
                Some(t) => cond(t),
                None => false,
            })
    }
    fn distance_from_end(
        &self,
        end: Position,
        cost: usize,
        mut path_costs: &mut HashMap<Position, usize>,
    ) {
        for next_pos in self.next_nodes(&end, |t| t == &Tile::Path || t == &Tile::Start) {
            if !path_costs.contains_key(&next_pos) {
                path_costs.insert(next_pos, cost + 1);
                self.distance_from_end(next_pos, cost + 1, &mut path_costs);
            }
        }
    }

    fn path_costs(&self) -> HashMap<Position, usize> {
        let mut costs: HashMap<Position, usize> = HashMap::new();
        costs.insert(self.end, 0);
        self.distance_from_end(self.end, 0, &mut costs);
        // for (key, val) in costs.iter() {
        //     println!("Time from {} : {}", key, val);
        // }
        costs
    }
    fn cheats_from(&self, pos: &Position) -> Vec<Position> {
        self.next_nodes(pos, |t| t == &Tile::Wall)
            .into_iter()
            .map(|pos| {
                self.next_nodes(&pos, |t| t == &Tile::Path || t == &Tile::End)
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }
    fn cheats_from_local(
        &self,
        mut end_positions: &mut HashSet<(Position, Position, usize)>,
        start: &Position,
        intermediate: &Position,
        cost: usize,
    ) {
        match cost {
            0 => {}
            _ => {
                for next in self.next_nodes(intermediate, |t| t == &Tile::Path || t == &Tile::End) {
                    if &next != start {
                        end_positions.insert((*start, next, cost - 1));
                    }
                }
                for next in self.next_nodes(intermediate, |t| t == &Tile::Wall) {
                    self.cheats_from_local(end_positions, start, &next, cost - 1);
                }
            }
        }
    }
    fn cheats_with_length(&self, picoseconds_left: usize) -> HashSet<(Position, Position, usize)> {
        let mut end_positions = HashSet::new();
        for pos in self
            .map
            .iter()
            .enumerate()
            .filter_map(|(idx, tile)| match tile {
                Tile::Path | Tile::Start => Some(self.pos_from_index(idx)),
                _ => None,
            })
        {
            self.cheats_from_local(&mut end_positions, &pos, &pos, picoseconds_left);
        }
        end_positions
            .into_iter()
            .filter_map(|(start, end, left)| match left {
                x if x == picoseconds_left - 1 => None, //this is no cheat if we jump to next path
                x => Some((start, end, picoseconds_left - x)),
            })
            .collect()
    }
    fn cheats(&self) -> HashSet<(Position, Position)> {
        self.map
            .iter()
            .enumerate()
            .filter_map(|(idx, tile)| match tile {
                Tile::Path | Tile::Start => Some(self.pos_from_index(idx)),
                _ => None,
            })
            .map(|pos| {
                self.cheats_from(&pos)
                    .iter()
                    .filter_map(|jump| match jump {
                        j if j == &pos => None,
                        _ => Some((pos, *jump)),
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect()
    }
    fn saves_exactly(&self, jump_lenght: usize, pico_saved: usize) -> usize {
        let costs = self.path_costs();
        let saved_time = |start: Position, end: Position, cost| {
            let cost_start = costs.get(&start).unwrap();
            let cost_end = costs.get(&end).unwrap();
            if cost_start < cost_end {
                None
            } else {
                if cost_start.abs_diff(*cost_end) < pico_saved {
                    None
                } else {
                    Some(cost_start - cost_end - cost)
                }
            }
        };
        let cheats = self.cheats_with_length(jump_lenght);

        cheats
            .into_iter()
            .filter_map(|(start, end, cost)| saved_time(start, end, cost))
            .filter(|val| val == &pico_saved)
            .count()
    }
    fn reconstruct_path(
        came_from: &HashMap<Position, Position>,
        mut position: Position,
    ) -> Vec<Position> {
        let mut path = VecDeque::new();
        while let Some(pos) = came_from.get(&position) {
            position = *pos;
            path.push_front(*pos)
        }
        path.into_iter().collect()
    }
    fn best_path(&self) -> Option<Vec<Position>> {
        let mut open_set = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let mut g_score = HashMap::new();

        g_score.insert(self.start, 0);
        open_set.push(HeapEntry {
            pos: self.start,
            cost: 0,
        });

        while let Some(HeapEntry {
            pos: current,
            mut cost,
        }) = open_set.pop()
        {
            cost += 1;
            if current == self.end {
                return Some(Self::reconstruct_path(&came_from, current));
            }
            for next in self.next_nodes(&current, |tile| tile == &Tile::End || tile == &Tile::Path)
            {
                if cost <= *g_score.entry(next).or_insert(usize::MAX) {
                    came_from.insert(next, current);
                    // println!("Current path: \n{}\n",self.print_with_marked_path(&came_from, next));
                    g_score.insert(next, cost);
                    let new = HeapEntry {
                        pos: next,
                        cost: cost,
                    };
                    if open_set.iter().find(|&&e| e == new).is_none() {
                        open_set.push(new);
                    }
                }
            }
        }
        None
    }
    fn count_valid_cheats(
        &self,
        valid_cheats: &mut HashMap<(Position, Position), usize>,
        costs: &HashMap<Position, usize>,
        mut arrival_time: &mut HashMap<Position, usize>,
        start: &Position,
        cheat_duration: usize,
        intermediate: &Position,
        cheat_time_left: usize,
    ) {
        match self.at(*intermediate) {
            Some(Tile::Path) | Some(Tile::End) => {
                if intermediate != start {
                    let save = *costs.get(start).unwrap() as isize
                        - cheat_duration as isize
                        - *costs.get(intermediate).unwrap() as isize;
                    if save > 0 {
                        match valid_cheats.entry((*start, *intermediate)).or_insert(0) {
                            x => *x = max(*x, save as usize),
                        }
                    }
                }
            }
            _ => {}
        };
        match cheat_time_left {
            0 => {}
            _ => {
                for next in self.next_nodes(intermediate, |t| {
                    t == &Tile::Wall || t == &Tile::Path || t == &Tile::End || t == &Tile::Start {}
                }) {
                    match arrival_time.get(&next) {
                        Some(prev_time) if prev_time <= &cheat_duration => {
                            // this is a new slower cheat, ignore it
                            continue;
                        }
                        _ => {
                            arrival_time.insert(next, cheat_duration);
                            self.count_valid_cheats(
                                valid_cheats,
                                &costs,
                                arrival_time,
                                start,
                                cheat_duration + 1,
                                &next,
                                cheat_time_left - 1,
                            );
                        }
                    }
                }
            }
        }
    }
    fn cheat_saves(&self, jump_lenght: usize) -> HashMap<usize, usize> {
        let costs = self.path_costs();
        let mut saves_count = HashMap::new();
        {
            for pos in self.best_path().unwrap() {

                self.count_valid_cheats(
                    &mut saves_count,
                    &costs,
                    &mut HashMap::new(),
                    &pos,
                    0,
                    &pos,
                    jump_lenght,
                );
            }
        }
        let mut res_map = HashMap::new();
        for ((_, _), saves) in saves_count {
            *res_map.entry(saves).or_insert(0) += 1;
        }
        res_map
    }
}
#[derive(Eq, PartialEq, Copy, Clone, Debug, Ord, Hash)]
struct HeapEntry {
    pos: Position,
    cost: usize,
}
impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cost.cmp(&other.cost).reverse())
    }
}
impl From<char> for Tile {
    fn from(c: char) -> Tile {
        match c {
            '#' => Tile::Wall,
            '.' => Tile::Path,
            'S' => Tile::Start,
            'E' => Tile::End,
            _ => panic!("Unknown tile: {}", c),
        }
    }
}
impl<Reader> From<Reader> for RaceCondition
where
    Reader: BufRead,
{
    fn from(reader: Reader) -> Self {
        let mut rc = Self {
            map: vec![],
            start: Position { x: 0, y: 0 },
            end: Position { x: 0, y: 0 },
            size: Position { x: 0, y: 0 },
        };
        for line in reader.lines().map_while(|line| line.ok()) {
            rc.map.extend(line.chars().map(Tile::from));
            rc.size.x = line.len() as isize;
            rc.size.y += 1;
        }
        rc.where_start().where_end()
    }
}
impl Display for RaceCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                write!(f, "{}", self.at(Position { x, y }).unwrap())?;
            }
            writeln!(f, "")?
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
    use crate::{RaceCondition, read_input};

    #[test]
    fn test_part1() {
        let _reader = read_input("example-input.txt");
        let race_condition = RaceCondition::from(_reader);
        assert_eq!(race_condition.path_costs().len(), 85);
        assert_eq!(race_condition.saves_exactly(2, 64), 1);
        assert_eq!(race_condition.saves_exactly(2, 40), 1);
        assert_eq!(race_condition.saves_exactly(2, 38), 1);
        assert_eq!(race_condition.saves_exactly(2, 20), 1);
        assert_eq!(race_condition.saves_exactly(2, 12), 3);
        assert_eq!(race_condition.saves_exactly(2, 10), 2);
        assert_eq!(race_condition.saves_exactly(2, 8), 4);
        assert_eq!(race_condition.saves_exactly(2, 6), 2);
        assert_eq!(race_condition.saves_exactly(2, 4), 14);
        assert_eq!(race_condition.saves_exactly(2, 2), 14);

        let save_count = race_condition.cheat_saves(2);
        let mut sorted = save_count.iter().collect::<Vec<_>>();
        sorted.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
        for (key, value) in sorted.iter() {
            println!("There are {value} cheats that save {key:?} picoseconds.")
        }
        assert_eq!(1, save_count[&64]);
        assert_eq!(1, save_count[&40]);
        assert_eq!(1, save_count[&38]);
        assert_eq!(1, save_count[&20]);
        assert_eq!(3, save_count[&12]);
        assert_eq!(2, save_count[&10]);
        assert_eq!(4, save_count[&8]);
        assert_eq!(2, save_count[&6]);
        assert_eq!(14, save_count[&4]);
        assert_eq!(14, save_count[&2]);
    }
    #[test]
    fn test_part2() {
        let _reader = read_input("example-input.txt");
        let race_condition = RaceCondition::from(_reader);
        let save_count = race_condition.cheat_saves(20);
        let mut sorted = save_count.iter().collect::<Vec<_>>();
        sorted.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
        for (key, value) in sorted.iter() {
            println!("There are {value} cheats that save {key:?} picoseconds.")
        }

        assert_eq!(3, save_count[&76]);
        assert_eq!(4, save_count[&74]);
        assert_eq!(22, save_count[&72]);
        assert_eq!(12, save_count[&70]);
        assert_eq!(14, save_count[&68]);

        assert_eq!(12, save_count[&66]);
        assert_eq!(19, save_count[&64]);
        assert_eq!(20, save_count[&62]);

        assert_eq!(12, save_count[&66]);
        assert_eq!(19, save_count[&64]);
        assert_eq!(20, save_count[&62]);


        assert_eq!(23, save_count[&60]);
        assert_eq!(25, save_count[&58]);
        assert_eq!(39, save_count[&56]);

        assert_eq!(29, save_count[&54]);
        assert_eq!(31, save_count[&52]);
        assert_eq!(32, save_count[&50]);
    }
}
