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

fn parse_only_bus(input: &String) -> Vec<u64> {
    let mut res = Vec::new();
    for val in input.split(",") {
        if *val != "x".to_owned() {
            res.push(val.parse().unwrap());
        }
    }
    res
}

fn parse_bus_indices(input: &String) -> Vec<(u64, u64)> {
    let mut res = Vec::new();
    for (i, val) in input.split(",").enumerate() {
        if *val != "x".to_owned() {
            res.push((val.parse().unwrap(), i as u64));
        }
    }
    res
}

fn get_next_bus(depart_time: u64, buses: &Vec<u64>) -> u64 {
    let mut wait_time = 1000;
    let mut bus = 0;
    for val in buses.iter() {
        let (div, rem) = (depart_time / *val, depart_time % (*val));
        let wait = match rem {
            0 => 0,
            _ => (div + 1) * (*val) - depart_time,
        };
        if wait < wait_time {
            wait_time = wait;
            bus = *val;
        }
    }
    wait_time * bus
}

fn main() {
    let input = read_input("input").unwrap();
    let depart_time: u64 = input[0].parse().unwrap();
    let buses = parse_only_bus(&input[1]);
    println!("Part 1: {}", get_next_bus(depart_time, &buses));

    let mut bus_indices = parse_bus_indices(&input[1]);
    println!("{:?}", bus_indices);

    // Did not code Part 2: Use Chinese Remainder Theorem with the following inputs
    // Remainder    Modulus
    //   1            17
    //   25           41
    //   627          643
    //   22           23
    //   10           13
    //   13           29
    //   386          433
    //   21           37
    //   10           19
    //
    // This will provide t + 1 where t is the required timestamp.
    // Used : https://www.dcode.fr/chinese-remainder
    // Ans: t = 760171380521445
}
