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
    image: Vec<Vec<u8>>,
}

impl Jigsaw {
    // Rotation here means each row is reversed (So columns are reflected)
    // Rotation is done in place using a mutable reference to the element
    // 1 2 3    3 2 1
    // 4 5 6 => 6 5 4
    // 7 8 9    9 8 7
    fn flip_90(&mut self, tile: u64, flip_image: bool) {
        let current_tile = match flip_image {
            false => self.tiles.get_mut(&tile).unwrap(),
            true => &mut self.image,
        };
        for row in current_tile {
            row.reverse();
        }
    }
    // Flip here means that each column is reversed (So rows are reflected)
    // 1 2 3    7 8 9
    // 4 5 6 => 4 5 6
    // 7 8 9    1 2 3
    fn flip_180(&mut self, tile: u64, flip_image: bool) {
        let current_tile = match flip_image {
            false => self.tiles.get_mut(&tile).unwrap(),
            true => &mut self.image,
        };
        current_tile.reverse();
    }

    // 1 2 3    7 4 1
    // 4 5 6 => 8 5 2
    // 7 8 9    9 6 3
    fn rotate_forward(&mut self, val: u64, rotate_image: bool) {
        let tile = match rotate_image {
            false => self.tiles.get_mut(&val).unwrap(),
            true => &mut self.image,
        };
        let mut tmp_vec: Vec<Vec<u8>> = (0..tile[0].len())
            .map(|i| tile.iter().map(|v| v[i]).collect())
            .collect();
        for v in tmp_vec.iter_mut() {
            v.reverse();
        }
        match rotate_image {
            false => self.tiles.insert(val, tmp_vec),
            true => {
                self.image = tmp_vec;
                None
            }
        };
    }

    // Reverse of rotate_forward
    fn rotate_backward(&mut self, val: u64, rotate_image: bool) {
        match rotate_image {
            true => self.flip_90(0, true),
            false => self.flip_90(val, false),
        };
        let tile = match rotate_image {
            false => self.tiles.get_mut(&val).unwrap(),
            true => &mut self.image,
        };
        let mut tmp_vec: Vec<Vec<u8>> = (0..tile[0].len())
            .map(|i| tile.iter().map(|v| v[v.len() - 1 - i]).collect())
            .collect();
        for v in tmp_vec.iter_mut() {
            v.reverse();
        }
        match rotate_image {
            false => self.tiles.insert(val, tmp_vec),
            true => None,
        };
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
                self.flip_180(*val, false);
                match self.helper(square, *val, i, j, unseen) {
                    Some(v) => return Some(v),
                    None => (),
                };
                // Flip back
                self.flip_180(*val, false);
                // Rotate Forward
                self.rotate_forward(*val, false);
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

    fn remove_borders(&mut self) {
        for (val, tile) in self.tiles.iter_mut() {
            // Remove last row
            (*tile).pop();
            // Remove first row
            *tile = tile[1..].to_vec();
            // Remove first and last column
            let mut new_vec = Vec::new();
            for i in 0..tile.len() {
                let mut tmp_vec = Vec::new();
                for j in 1..tile[0].len() - 1 {
                    tmp_vec.push(tile[i][j]);
                }
                new_vec.push(tmp_vec);
            }
            *tile = new_vec;
        }
        for (_, tile) in self.tiles.iter() {
            assert!(tile.len() == self.dimension - 2);
            assert!(tile[0].len() == self.dimension - 2)
        }
    }

    fn construct_image(&mut self, square: &Vec<Vec<u64>>) {
        let new_dim = self.dimension - 2;
        let mut image = vec![vec![0; new_dim * self.num_tiles]; new_dim * self.num_tiles];
        for i in 0..square.len() {
            for j in 0..square[0].len() {
                let current_tile = self.tiles.get(&square[i][j]).unwrap();
                for k in 0..new_dim {
                    for l in 0..new_dim {
                        image[i * new_dim + k][j * new_dim + l] = current_tile[k][l];
                    }
                }
            }
        }
        self.image = image;
    }

    fn total_hashes(&self) -> usize {
        let mut count = 0;
        for v in self.image.iter() {
            for k in v.iter() {
                if *k as u8 == 35 {
                    count += 1;
                }
            }
        }
        count
    }

    // Apply the given filter at the given offset and check if there is a sea monster
    // If yes: Return the number of hashesm else return None
    //                   #
    // #    ##    ##    ###
    //  #  #  #  #  #  #
    fn convolve(&self, offset_i: usize, offset_j: usize) -> Option<usize> {
        // Filter is 3 x 20 -> Positions in the filter where hashes are needed are listed below
        let filter = vec![
            (0, 18),
            (1, 0),
            (1, 5),
            (1, 6),
            (1, 11),
            (1, 12),
            (1, 17),
            (1, 18),
            (1, 19),
            (2, 1),
            (2, 4),
            (2, 7),
            (2, 10),
            (2, 13),
            (2, 16),
        ];
        for (pos_i, pos_j) in filter.iter() {
            if self.image[offset_i + pos_i][offset_j + pos_j] != 35 as u8 {
                return None;
            }
        }
        Some(15)
    }

    fn find_sea_monster(&mut self) -> usize {
        let mut num_hashes = self.total_hashes();
        for _ in 0..4 {
            //println!("{:?}", self.image[0]);
            for i in 0..(self.image.len() - 3) {
                for j in 0..(self.image[0].len() - 20) {
                    match self.convolve(i, j) {
                        None => (),
                        Some(v) => {
                            num_hashes -= v;
                        }
                    }
                }
            }
            self.flip_180(0, true);
            //println!("{:?}", self.image[0]);
            for i in 0..(self.image.len() - 3) {
                for j in 0..(self.image[0].len() - 20) {
                    match self.convolve(i, j) {
                        None => (),
                        Some(v) => {
                            num_hashes -= v;
                        }
                    }
                }
            }
            self.flip_180(0, true);
            self.rotate_forward(0, true);
        }
        num_hashes
    }
}

fn main() {
    let input = read_input("input").unwrap();
    let input = get_parsed_input(input);
    let num_tiles = (input.len() as f64).sqrt() as usize;
    let mut square = vec![vec![0; num_tiles]; num_tiles];
    let mut unseen: HashSet<u64> = input.keys().cloned().collect();
    let mut jigsaw = Jigsaw {
        tiles: input,
        num_tiles: num_tiles,
        dimension: 10,
        image: vec![],
    };
    let now = Instant::now();
    let square = jigsaw.backtrack(0, 0, &mut square, &mut unseen).unwrap();
    println!("Part1: {:?}", square);
    println!("Time elapsed in ms: {}", now.elapsed().as_millis());
    let now = Instant::now();
    jigsaw.remove_borders();
    jigsaw.construct_image(&square);
    let num_hashes = jigsaw.find_sea_monster();
    println!("Part 2: {}", num_hashes);
    println!("Time elapsed in ms: {}", now.elapsed().as_millis());
}
