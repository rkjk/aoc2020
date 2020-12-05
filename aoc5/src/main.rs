use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error, ErrorKind};

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines()
        .map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn pass_str(pass: &str) -> String {
    pass.to_owned()
        .replace("F", "0")
        .replace("B", "1")
        .replace("L", "0")
        .replace("R", "1")
}

fn get_seat_ID(pass: &str) -> u64 {
    let row_val = u64::from_str_radix(&pass[..pass.len() - 3], 2).unwrap();
    let col_val = u64::from_str_radix(&pass[(pass.len() - 3)..], 2).unwrap();
    row_val * 8 + col_val
}

fn main() {
    let input = read_input("input").unwrap();
    let pass_bin_string: Vec<String> = input.iter().map(|x| pass_str(x)).collect();
    let mut seat_ids: Vec<u64> = pass_bin_string.iter().map(|x| get_seat_ID(x)).collect();
    seat_ids.sort();
    let max_seat_id: u64 = seat_ids[seat_ids.len() - 1];
    println!("Part 1 - Max Seat ID: {}", max_seat_id);

    let mut sub_vec: Vec<u64> = vec![0];
    sub_vec.extend(seat_ids[..(seat_ids.len() - 1)].to_owned());
    let res_vec: Vec<u64> = seat_ids.iter().zip(&sub_vec).map(|(a, b)| a - b).collect();
    let my_seat_idx = res_vec.iter().position(|r| r == &2).unwrap();
    println!("Part 2 - My Seat ID: {}", seat_ids[my_seat_idx] - 1);
}
