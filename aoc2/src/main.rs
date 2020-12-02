use std::io::{self, BufReader, Error, ErrorKind, Read};
use std::io::prelude::*;
use std::fs::File;
use counter::Counter;

// Structure to hold the password and the character with lower and upper bound on frequency
#[derive(Debug)]
struct Password {
    lo: usize,
    hi: usize,
    c: char,
    pass: String,
}

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

// Parse String that looks like 1-3 a: abcde and return Password { lo: 1, hi: 3, c: a, pass: abcde}
fn parse_input(input: &str) -> Password {
    let vec_colon: Vec<&str> = input.split(":").collect();
    let pass = vec_colon[1].trim();
    let bounds_c = vec_colon[0].trim().as_bytes();
    let c = bounds_c[bounds_c.len() - 1] as char;
    let bounds = std::str::from_utf8(&bounds_c[0..(bounds_c.len() - 1)]).unwrap();
    let bound_vec: Vec<&str> = bounds.split("-").collect();
    let lo = bound_vec[0].trim().parse::<usize>().unwrap();
    let hi = bound_vec[1].trim().parse::<usize>().unwrap();

    return Password { lo: lo, hi: hi, c: c, pass: pass.to_string()}
}

/// Get the number of valid passwords
fn get_num_valid_passwords(input: &Vec<String>) -> u64 {
    let mut counter: u64 = 0;
    for val in input.iter() {
        let pass_struct = parse_input(val);
        let chars_count = pass_struct.pass.chars().collect::<Counter<_>>();
        let c_count = chars_count[&pass_struct.c];
        if c_count >= pass_struct.lo && c_count <= pass_struct.hi {
            counter += 1;
        }
    }
    return counter;
}

/// Get valid passwords - part 2
fn get_num_valid_passwords_2(input: &Vec<String>) -> u64 {
    let mut counter: u64 = 0;
    for val in input.iter() {
        let pass_struct = parse_input(val);
        let (lo, hi, c, pass) = (pass_struct.lo, pass_struct.hi, pass_struct.c, pass_struct.pass);
        let mut local_count = 0;
        for (i, cp) in pass.chars().enumerate() {
            if ((i+1) == lo || (i+1) == hi) && cp == c {
                local_count += 1;
            }
        }
        if local_count == 1 {
            counter += 1
        }
    }
    counter
}

fn main() {
    let input = read_input("input").unwrap();
    println!("Valid passwords: {}", get_num_valid_passwords(&input));
    println!("Valid passwords: {}", get_num_valid_passwords_2(&input));
}
