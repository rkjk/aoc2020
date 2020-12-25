use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error, ErrorKind};
use std::time::Instant;

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines()
        .map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn get_parsed_input(mut input: Vec<String>) -> HashMap<u64, Vec<Vec<u8>>> {
    let mut map = HashMap::new();
    let mut current_tile = Vec::new();
    let mut current_tile_no = 0;
    for line in input.iter_mut() {
        if line == "" {
            //println!("Inserting {}", current_tile_no);
            map.insert(current_tile_no, current_tile);
            current_tile = Vec::new();
            continue;
        }
        if line.starts_with("Tile") {
            line.truncate(line.len() - 1);
            current_tile_no = line.split(" ").collect::<Vec<&str>>()[1].parse().unwrap();
            continue;
        }
        current_tile.push(line.clone().into_bytes());
    }
    //println!("Inserting {}", current_tile_no);
    map.insert(current_tile_no, current_tile);
    map
}

struct Jigsaw {
    tiles: HashMap<u64, Vec<Vec<u8>>>,
    dimension: usize,
    num_tiles: usize,
}

impl Jigsaw {
    // Rotation here means each row is reversed (So columns are reflected)
    // Rotation is done in place using a mutable reference to the element
    // 1 2 3    3 2 1
    // 4 5 6 => 6 5 4
    // 7 8 9    9 8 7
    fn flip_90(&mut self, tile: u64) {
        let current_tile = self.tiles.get_mut(&tile).unwrap();
        for row in current_tile {
            row.reverse();
        }
    }
    // Flip here means that each column is reversed (So rows are reflected)
    // 1 2 3    7 8 9
    // 4 5 6 => 4 5 6
    // 7 8 9    1 2 3
    fn flip_180(&mut self, tile: u64) {
        let current_tile = self.tiles.get_mut(&tile).unwrap();
        current_tile.reverse();
    }

    // 1 2 3    7 4 1
    // 4 5 6 => 8 5 2
    // 7 8 9    9 6 3
    fn rotate_forward(&mut self, val: u64) {
        let tile = self.tiles.get(&val).unwrap();
        let mut tmp_vec: Vec<Vec<u8>> = (0..tile[0].len())
            .map(|i| tile.iter().map(|v| v[i]).collect())
            .collect();
        for v in tmp_vec.iter_mut() {
            v.reverse();
        }
        self.tiles.insert(val, tmp_vec);
    }

    // Reverse of rotate_forward
    fn rotate_backward(&mut self, val: u64) {
        self.flip_90(val);
        let tile = self.tiles.get(&val).unwrap();
        let mut tmp_vec: Vec<Vec<u8>> = (0..tile[0].len())
            .map(|i| tile.iter().map(|v| v[v.len() - 1 - i]).collect())
            .collect();
        for v in tmp_vec.iter_mut() {
            v.reverse();
        }
        self.tiles.insert(val, tmp_vec);
    }

    fn helper(
        &mut self,
        square: &mut Vec<Vec<u64>>,
        val: u64,
        i: usize,
        j: usize,
        unseen: &mut HashSet<u64>,
    ) -> Option<Vec<Vec<u64>>> {
        match self.check_match(square, val, i, j) {
            true => {
                square[i][j] = val;
                let (mut new_i, mut new_j) = (i, j + 1);
                if new_j == self.num_tiles {
                    new_i = i + 1;
                    new_j = 0;
                }
                match self.backtrack(new_i, new_j, square, unseen) {
                    Some(v) => return Some(v),
                    None => (),
                }
            }
            false => (),
        };
        None
    }

    fn backtrack(
        &mut self,
        i: usize,
        j: usize,
        square: &mut Vec<Vec<u64>>,
        unseen: &mut HashSet<u64>,
    ) -> Option<Vec<Vec<u64>>> {
        //println!("i: {} j: {} Square: {:?}", i, j, square);
        if i == self.num_tiles {
            return Some(square.clone());
        }
        let unseen_vec: Vec<u64> = unseen.iter().copied().collect();
        for val in unseen_vec.iter() {
            unseen.remove(val);
            for _ in 0..4 {
                match self.helper(square, *val, i, j, unseen) {
                    Some(v) => return Some(v),
                    None => (),
                };
                // Flip along horizontal axis
                self.flip_180(*val);
                match self.helper(square, *val, i, j, unseen) {
                    Some(v) => return Some(v),
                    None => (),
                };
                // Flip back
                self.flip_180(*val);
                // Rotate Forward
                self.rotate_forward(*val);
            }
            unseen.insert(*val);
            square[i][j] = 0;
        }
        None
    }

    fn check_match(&self, square: &Vec<Vec<u64>>, val: u64, i: usize, j: usize) -> bool {
        if i == 0 && j == 0 {
            return true;
        }
        let (mut left, mut up) = (true, true);
        let current_tile = self.tiles.get(&val).unwrap();
        if j != 0 {
            left = false;
            let left_tile = self.tiles.get(&square[i][j - 1]).unwrap();
            let left_column: Vec<u8> = left_tile.iter().map(|v| v[v.len() - 1]).collect();
            let right_column: Vec<u8> = current_tile.iter().map(|v| v[0]).collect();
            if left_column
                .iter()
                .zip(right_column.iter())
                .filter(|&(a, b)| a == b)
                .count()
                == left_column.len()
            {
                left = true;
            }
        }
        if i != 0 {
            up = false;
            let up_tile = self.tiles.get(&square[i - 1][j]).unwrap();
            let up_row: Vec<u8> = up_tile[up_tile.len() - 1].clone();
            let down_row = current_tile[0].clone();
            if up_row
                .iter()
                .zip(down_row.iter())
                .filter(|&(a, b)| a == b)
                .count()
                == up_row.len()
            {
                up = true;
            }
        }
        up & left
    }
}

fn main() {
    let input = read_input("input").unwrap();
    let input = get_parsed_input(input);
    let num_tiles = (input.len() as f64).sqrt() as usize;
    println!("Num_tiles: {}", num_tiles);
    let mut square = vec![vec![0; num_tiles]; num_tiles];
    let mut unseen: HashSet<u64> = input.keys().cloned().collect();
    let mut jigsaw = Jigsaw {
        tiles: input,
        num_tiles: num_tiles,
        dimension: 10,
    };
    let square = jigsaw.backtrack(0, 0, &mut square, &mut unseen);
    println!("Part1: {:?}", square);
}
