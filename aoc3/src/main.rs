use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error};
use std::ops::Rem;

// Read Input file and spit out a Vector containing each row as a vector of byes. Since the input has only '.' and '#' this well make it easy to index into.
fn read_input(filename: &str) -> Vec<Vec<u8>> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines()
        .map(|l| l.and_then(|v| Ok(v.into_bytes())))
        .collect::<Result<Vec<Vec<u8>>, Error>>()
        .unwrap()
}

/// Find number of trees in path by traversing right 3 down 1 every step.
fn get_num_trees(input: &Vec<Vec<u8>>, right_slope: usize, down_slope: usize) -> usize {
    let (mut right, mut down) = (0, 0);
    let mut num_trees = 0;
    let num_cols = input[0].len();

    let tree = "#".as_bytes()[0];

    while down < input.len() {
        //println!("{} {}", right, down);
        let val = input[down][right].clone();
        if val == tree {
            num_trees += 1;
        }
        right = (right + right_slope) % (num_cols);
        down += down_slope;
    }
    num_trees
}

fn main() {
    let input = read_input("input");
    //println!("{:?}", input);

    println!("Part 1: {}", get_num_trees(&input, 3, 1));

    // Part 2
    println!(
        "Part 2: {}",
        get_num_trees(&input, 3, 1)
            * get_num_trees(&input, 1, 1)
            * get_num_trees(&input, 5, 1)
            * get_num_trees(&input, 7, 1)
            * get_num_trees(&input, 1, 2)
    )
}
