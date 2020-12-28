use std::collections::HashMap;
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

    fn get_black_tiles(&self) -> usize {
        self.tile_coords.values().filter(|v| **v % 2 != 0).count()
    }
}

fn main() {
    let input = read_input("input").unwrap();
    let mut tiles = Tiles {
        tiles: input,
        tile_coords: HashMap::new(),
    };
    tiles.get_coords();
    println!("Part 1: {}", tiles.get_black_tiles());
}
