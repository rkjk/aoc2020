use std::io::{self, BufReader, Error, ErrorKind, Read};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

fn read_input(filename: &str) -> Result<Vec<u64>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

/// Find the 2 numbers in Vec summing to 2020 and return the product. 
/// Implement Two Sum
fn return_product(input: & Vec<u64>) -> u64 {
    let mut seen = HashSet::new();
    for val in input.iter() {
        let diff = 2020 - val;
        if seen.contains(&diff) {
            return diff * val
        }
        seen.insert(val);
    }
    return 0
}

/// Find 3 numbers summing to 2020 and return product.
/// Implement Three-Sum
fn return_product_3(mut input: Vec<u64>) -> u64 {
    input.sort();
    for i in 0..(input.len() - 2) {
        let diff = 2020 - input[i];
        for j in (i+1)..(input.len() - 1) {
            if input[j] > diff {
                continue;
            }
            let diff2 = diff - input[j];
            for k in j+1..input.len() {
                if diff2 == input[k] {
                    return input[i] * input[j] * diff2;
                }
            }
        }
    }
    return 0;
}

fn main() {
    let filename = "input";
    let input = read_input(filename).unwrap();
    println!("{}", return_product(&input));
    println!("{}", return_product_3(input));
}
