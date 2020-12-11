use std::cmp::min;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error, ErrorKind};
use std::time::Instant;

fn read_input(filename: &str) -> Result<Vec<Vec<i64>>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines()
        .map(|l| {
            l.and_then(|v| {
                Ok(v.chars()
                    .map(|c| match c {
                        'L' => 1,
                        '.' => 0,
                        _ => 5,
                    })
                    .collect::<Vec<i64>>())
            })
        })
        .collect()
}

struct Grid {
    grid: Vec<Vec<i64>>,
}

impl Grid {
    fn new() -> Self {
        Grid { grid: vec![] }
    }

    fn run_one_iteration(
        &mut self,
        occupied_threshold: i64,
        occupied_func: fn(&Grid, i64, i64) -> i64,
    ) -> bool {
        //let mut new_grid: Vec<Vec<i64>> = vec![vec![0; self.grid[0].len()]; self.grid.len()];
        let mut new_grid = self.grid.clone();
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                match self.grid[i][j] {
                    2 => {
                        match occupied_func(&self, i as i64, j as i64) >= occupied_threshold {
                            true => new_grid[i][j] = 1,
                            false => (),
                        };
                    }
                    1 => match occupied_func(&self, i as i64, j as i64) == 0 {
                        true => new_grid[i][j] = 2,
                        false => (),
                    },
                    _ => (),
                };
            }
        }
        //println!("Old grid: {:?}", self.grid);
        //println!("New grid: {:?}", new_grid);
        let flag = self.grid == new_grid;
        self.grid = new_grid;
        flag
    }

    fn get_num_occupied_seats(&self, i: i64, j: i64) -> i64 {
        let mut count = 0;
        let indices = vec![
            (i - 1, j),
            (i + 1, j),
            (i, j + 1),
            (i, j - 1),
            (i + 1, j + 1),
            (i + 1, j - 1),
            (i - 1, j + 1),
            (i - 1, j - 1),
        ];
        for index in indices.iter() {
            if index.0 < 0 || index.0 > (self.grid.len() - 1) as i64 {
                continue;
            }
            if index.1 < 0 || index.1 > (self.grid[0].len() - 1) as i64 {
                continue;
            }
            match self.grid[index.0 as usize][index.1 as usize] {
                2 => count += 1,
                _ => (),
            };
        }
        count
    }

    fn get_num_occupied_seats_los(&self, i: i64, j: i64) -> i64 {
        let mut count = 0;
        let ius = i as usize;
        let jus = j as usize;
        let row_size = self.grid.len();
        let col_size = self.grid[0].len();
        /*
        println!(
            "row_size: {}, ius: {}, col_size: {}, jus: {}",
            row_size, ius, col_size, jus
        );
        */
        let row_up: Vec<i64> = (0..ius).rev().map(|k| self.grid[k][jus]).collect();
        let row_down: Vec<i64> = ((ius + 1)..(row_size))
            .map(|k| self.grid[k][j as usize])
            .collect();
        let col_left: Vec<i64> = (0..jus).rev().map(|k| self.grid[ius][k]).collect();
        let col_right: Vec<i64> = ((jus + 1)..(col_size)).map(|k| self.grid[ius][k]).collect();
        let lim = min(ius, jus);
        let diag_up_left: Vec<i64> = ((ius - lim)..ius)
            .rev()
            .zip((jus - lim..jus).rev())
            .map(|(k, l)| self.grid[k][l])
            .collect();
        let lim = min(ius, col_size - 1 - jus);
        let diag_up_right: Vec<i64> = ((ius - lim)..ius)
            .rev()
            .zip((jus + 1)..(jus + lim + 1))
            .map(|(k, l)| self.grid[k][l])
            .collect();
        let lim = min(row_size - ius - 1, jus);
        let diag_down_left: Vec<i64> = ((ius + 1)..(ius + lim + 1))
            .zip(((jus - lim)..jus).rev())
            .map(|(k, l)| self.grid[k][l])
            .collect();
        let lim = min(row_size - ius - 1, col_size - jus - 1);
        let diag_down_right: Vec<i64> = ((ius + 1)..(ius + lim + 1))
            .zip((jus + 1)..(jus + lim + 1))
            .map(|(k, l)| self.grid[k][l])
            .collect();
        for (i, v) in vec![
            row_up,
            row_down,
            col_left,
            col_right,
            diag_up_left,
            diag_up_right,
            diag_down_left,
            diag_down_right,
        ]
        .iter()
        .enumerate()
        {
            count += match self.check_arr_occupied(&v) {
                true => 1,
                false => 0,
            };
            //println!("i: {} v: {:?}, count: {}", i, v, count);
        }
        //println!("");
        count
    }

    fn check_arr_occupied(&self, inp: &Vec<i64>) -> bool {
        for v in inp.iter() {
            if *v == 2 {
                return true;
            } else if *v == 1 {
                return false;
            }
        }
        false
    }
}

fn main() {
    let input = read_input("input").unwrap();
    //println!("{} {}", input.len(), input[input.len() - 1].len());
    let mut grid = Grid::new();
    grid.grid = input.clone();
    let now = Instant::now();
    while true {
        if grid.run_one_iteration(4, Grid::get_num_occupied_seats) {
            break;
        }
    }
    println!(
        "Part 1: {}",
        grid.grid.into_iter().flatten().fold(0, |acc, x| {
            if x == 2 {
                return acc + 1;
            }
            acc
        })
    );
    println!("Runtime {} ms", now.elapsed().as_millis());
    let now = Instant::now();
    grid.grid = input.clone();
    //println!("{:?}", grid.grid);
    let mut i = 0;
    while true {
        if i == 1000 {
            break;
        }
        if grid.run_one_iteration(5, Grid::get_num_occupied_seats_los) {
            break;
        }
        i += 1;
        //println!("After {} iteration: {:?}", i, grid.grid);
    }
    println!(
        "Part 2: {}",
        grid.grid.into_iter().flatten().fold(0, |acc, x| {
            if x == 2 {
                return acc + 1;
            }
            acc
        })
    );
    println!("Runtime {} ms", now.elapsed().as_millis());
}
