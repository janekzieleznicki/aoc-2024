use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{PathBuf};

fn main() {
    let mut example_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    example_data.push("resources/puzzle-input.txt");
    let calc_dist = total_distance(&mut BufReader::new(File::open(example_data.clone()).unwrap()));
    let similarity_score = similarity_score(&mut BufReader::new(File::open(example_data).unwrap()));
    println!("Total distance: {calc_dist}\n Similarity score: {similarity_score}")
}

pub fn total_distance<Reader>(reader: &mut Reader) -> usize
where Reader: ?Sized + BufRead{
    let (mut left, mut right): (Vec<_>, Vec<_>) = reader.lines().into_iter()
        .filter_map(|line| line.ok())
        .map(|line| {
            let mut split = line.split_whitespace();
            (split.next().unwrap().parse::<usize>().unwrap(), split.next().unwrap().parse::<usize>().unwrap())
        })
        .unzip();
    left.sort_unstable();
    right.sort_unstable();
    left.iter().zip(right.iter())
        .map(|(&l,&r)|l.abs_diff(r))
        .sum()
}
pub fn similarity_score<Reader>(reader: &mut Reader) -> usize
where Reader: ?Sized + BufRead{
    let (mut left, mut right): (Vec<_>, Vec<_>) = reader.lines().into_iter()
        .filter_map(|line| line.ok())
        .map(|line| {
            let mut split = line.split_whitespace();
            (split.next().unwrap().parse::<usize>().unwrap(), split.next().unwrap().parse::<usize>().unwrap())
        })
        .unzip();
    let mut occurrances = HashMap::with_capacity(right.len());
    right.into_iter().for_each(|r| *occurrances.entry(r).or_default() += 1);
    left.iter()
        .map(|l| l * occurrances.get(l).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;
    use crate::{similarity_score, total_distance};

    #[test]
    fn test_distance() {
        let mut example_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        example_data.push("resources/example-input.txt");
        assert_eq!(11, total_distance(&mut BufReader::new(File::open(example_data).unwrap())));
    }
    #[test]
    fn test_similarity_score() {
        let mut example_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        example_data.push("resources/example-input.txt");
        assert_eq!(31, similarity_score(&mut BufReader::new(File::open(example_data).unwrap())));
    }

}
