#![feature(unsigned_signed_diff)]


use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn main() {
    let _reader = read_input("puzzle-input.txt");
    let mut disk_map = DiskMap::from(_reader);
    println!("Compacted checksum: {}", disk_map.compact());
    println!("Defragmented checksum: {}", disk_map.defragment());
}
#[derive(Debug,Copy, Clone)]
enum Fragment {
    Used { file_id: usize, blocks: u8, moved: bool},
    Free { blocks: u8},
}
impl Fragment {
    pub fn blocks(&self) -> u8 {
        match self {
            Fragment::Used { file_id: _, blocks , moved: _} => *blocks,
            Fragment::Free { blocks } => *blocks,
        }
    }
}
impl Display for Fragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Fragment::Used { file_id, blocks , moved: _} => {
                write!(f, "{}", format!("{}",file_id).repeat(*blocks as usize))?
            },
            Fragment::Free { blocks } => {
                write!(f, "{}", ".".repeat(*blocks as usize))?
            }
        }
        Ok(())
    }
}
impl From<(usize, u8)> for Fragment {
    fn from((idx, char): (usize, u8)) -> Fragment {
        match idx {
            i if i%2==1 => Fragment::Free {
                blocks: char
            },
            i if i%2==0 => Fragment::Used {
                file_id: idx/2,
                blocks: char,
                moved: false,
            },
            _ => unreachable!()
        }
    }
}
struct DiskMap{
    fragments: Vec<Fragment>
}
impl DiskMap {
    #[allow(dead_code)]
    fn print_blocks(blocks: &Vec<&Fragment>) -> String {
        blocks.iter().map(|&f| match f {
            Fragment::Free {  blocks: _ } => ".".to_string(),
            Fragment::Used { file_id: id, blocks: _, moved: _} => format!("{}", id)
        }).collect::<String>()
    }
    pub fn compact(&self) -> usize {
        let mut blocks = Vec::new();
        for frag in &self.fragments {
            blocks.append(&mut vec![frag; frag.blocks() as usize]);
        }
        // println!("Disk layout\n{}",Self::print_blocks(&blocks));
        let empty_indices = blocks.iter().enumerate().filter_map(|(idx,f)| match f {
            Fragment::Free {  blocks: _ } => Some(idx),
            _ => None
        }).collect::<Vec<_>>();
        let file_indices = blocks.iter().enumerate().rev().filter_map(|(idx,frag)|match frag {
            Fragment::Used { file_id: _, blocks: _, moved: _} => Some(idx),
            _ => None
        }).collect::<Vec<_>>();
        for (empty,file) in empty_indices.iter().zip(file_indices.iter()) {
            if blocks.iter().rev().take_while(|frag|match frag {
                Fragment::Free { blocks: _} => true,
                _ => false
            }).count() == empty_indices.len(){
                break;
            }
            // println!("Swapping {} with {}", blocks[*empty], blocks[*file]);
            blocks.swap(*empty,*file);
            // println!("Disk layout\n{}",Self::print_blocks(&blocks));
        }
        blocks.into_iter().map_while(|frag|match frag {
            Fragment::Used { file_id: id, blocks: _, moved: _} => Some(id),
            _ => None
        }).enumerate().fold(0, |acc, (idx, frag)|acc+idx*frag)
    }
    fn can_move_file(&self) -> Option<(usize, usize)> {
        for (file_idx, frag) in self.fragments.iter().enumerate().rev().take_while(|(idx,_)|*idx!=0) {
            match frag {
                Fragment::Used {file_id: _, blocks:_,moved:true} => continue,
                Fragment::Used { file_id: _, blocks: file_size,moved: false} => match self.fragments.iter().enumerate().take(file_idx).find_map(|(free_idx,free_frag)|{
                    match free_frag {
                        Fragment::Free { blocks: free_space} if free_space >= file_size => Some(free_idx),
                        _ => None
                    }
                }){
                    Some(free_idx) => return Some((file_idx, free_idx)),
                    _ => continue
                },

                _ => continue
            }
        }
        None
    }
    pub fn defragment(&mut self) -> usize {
        while let Some((file_idx, free_idx)) = self.can_move_file() {
            let mut file = self.fragments[file_idx];
            self.fragments[file_idx] = Fragment::Free {
                 blocks: file.blocks()
            };
            match &mut self.fragments[free_idx] {
                Fragment::Free { blocks: free_space} => {
                    *free_space-=file.blocks();
                },
                _ => unreachable!()
            }
            match &mut file {
                Fragment::Used {file_id: _, blocks: _,  moved } => {
                    *moved=true
                }
                _ => unreachable!()
            }
            self.fragments.insert(free_idx, file);
            // println!("{}",self);
        }
        let mut blocks = Vec::new();
        for frag in &self.fragments {
            blocks.append(&mut vec![frag; frag.blocks() as usize]);
        }
        // dbg!(&blocks);
        blocks.iter().enumerate().filter_map(|(idx,frag)|match frag {
            Fragment::Used { file_id, blocks: _,moved: _} => Some(file_id*idx),
            _ => None
        }).fold(0, |acc, mult|acc+mult)
    }
}
// fn print_layout(fragments: &[Fragment]) -> String {
//     fragments.iter().map(|f| format!("{}", f)).collect::<String>()
// }
impl Display for DiskMap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for frag in &self.fragments {
            write!(f, "{}", frag)?;
        }
        write!(f, "\n")?;
        Ok(())
    }
}

impl<Reader> From<Reader> for DiskMap
where
    Reader: BufRead,
{
    fn from(mut value: Reader) -> Self {
        let mut chars = Vec::new();
        value.read_to_end(chars.as_mut()).unwrap();
        Self{
            fragments: chars.into_iter()
                .map(|char|(char as char).to_digit(10).unwrap() as u8)
                .enumerate()
                .map(Fragment::from )
                .collect()
        }
    }
}

fn read_input(name: &str) -> BufReader<File> {
    let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file.push(format!("resources/{name}"));
    BufReader::new(File::open(file).unwrap())
}
#[cfg(test)]
mod tests {

    use crate::{read_input, DiskMap};

    #[test]
    fn test_part1() {
        let _reader = read_input("example-input.txt");
        let disk_map = DiskMap::from(_reader);
        // println!("{}",disk_map);
        assert_eq!(disk_map.compact(), 1928)
    }
    #[test]
    fn test_part2() {
        let _reader = read_input("example-input.txt");
        let mut disk_map = DiskMap::from(_reader);
        assert_eq!( disk_map.defragment(), 2858)
    }
}
