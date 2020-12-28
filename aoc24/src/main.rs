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

struct Tiles {
    tiles: Vec<String>,
    tile_coords: HashMap<(i64, i64), usize>,
}

impl Tiles {
    fn get_coords(&mut self) {
        let reference: (i64, i64) = (0, 0);
        for tile in self.tiles.iter() {
            let mut current = reference;
            let mut current_char = "".to_string();
            let mut char_it = tile.chars();
            loop {
                let c = char_it.next();
                if c == None {
                    break;
                }
                current_char.push_str(&c.unwrap().to_string());
                //println!("{}", current_char);
                match &current_char[..] {
                    "s" => continue,
                    "n" => continue,
                    _ => (),
                };

                match &current_char[..] {
                    "e" => current.0 += 1,
                    "w" => current.0 -= 1,
                    "se" => current.1 -= 1,
                    "sw" => {
                        current.0 -= 1;
                        current.1 -= 1;
                    }
                    "ne" => {
                        current.0 += 1;
                        current.1 += 1;
                    }
                    "nw" => current.1 += 1,
                    _p => panic!("Unkown pattern {}", _p),
                }
                current_char = "".to_string();
            }
            match self.tile_coords.contains_key(&current) {
                false => {
                    self.tile_coords.insert(current, 1);
                }
                true => {
                    let v = self.tile_coords.get_mut(&current).unwrap();
                    (*v) += 1;
                }
            }
        }
        //println!("{:?}", self.tile_coords);
    }

    fn get_num_black_tiles(&self) -> usize {
        self.tile_coords.values().filter(|v| **v % 2 != 0).count()
    }

    fn get_black_tiles(&self) -> HashSet<(i64, i64)> {
        let mut set = HashSet::new();
        for (k, v) in self.tile_coords.iter() {
            if *v % 2 != 0 {
                set.insert(*k);
            }
        }
        set
    }

    fn run_exhibit(&self) -> usize {
        let mut black_tiles = self.get_black_tiles();
        let neighbors = |x: &i64, y: &i64| {
            vec![
                (*x + 1, *y),
                (*x, *y - 1),
                (*x - 1, *y),
                (*x - 1, *y - 1),
                (*x + 1, *y + 1),
                (*x, *y + 1),
            ]
        };
        for _ in 0..100 {
            let mut new_black_tiles = HashSet::new();
            let mut white_tiles = HashSet::new();
            let mut new_white_tiles = HashSet::new();
            for (x, y) in black_tiles.iter() {
                // Check how many neighbors are black tiles. If 1 or 2, then don't flip to white
                let mut black_neighbor_count = 0;
                for n in neighbors(x, y).iter() {
                    if black_tiles.contains(n) {
                        black_neighbor_count += 1;
                    } else {
                        white_tiles.insert(n.clone());
                    }
                }
                match black_neighbor_count {
                    1 | 2 => new_black_tiles.insert((*x, *y)),
                    _ => new_white_tiles.insert((*x, *y)),
                };
            }
            for (x, y) in white_tiles.iter() {
                let mut black_neighbor_count = 0;
                for n in neighbors(x, y).iter() {
                    if black_tiles.contains(n) {
                        black_neighbor_count += 1;
                    }
                }
                match black_neighbor_count {
                    2 => new_black_tiles.insert((*x, *y)),
                    _ => new_white_tiles.insert((*x, *y)),
                };
            }
            black_tiles = new_black_tiles;
        }
        black_tiles.len()
    }
}

fn main() {
    let input = read_input("input").unwrap();
    let mut tiles = Tiles {
        tiles: input,
        tile_coords: HashMap::new(),
    };
    tiles.get_coords();
    println!("Part 1: {}", tiles.get_num_black_tiles());
    println!("Part 2: {}", tiles.run_exhibit());
}
