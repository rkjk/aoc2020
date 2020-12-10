use std::cmp::min;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error, ErrorKind};
use std::time::Instant;

fn read_input(filename: &str) -> Result<Vec<i64>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines()
        .map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() {
    let mut input = read_input("input").unwrap();

    // Part 1 - Sort the input and get the difference between consecutive elements
    let now = Instant::now();
    input.sort();
    let (mut count_1, mut count_3) = (0, 0);
    match input[0] {
        1 => count_1 += 1,
        3 => count_3 += 1,
        _ => (),
    };
    for i in 1..input.len() {
        match input[i] - input[i - 1] {
            1 => count_1 += 1,
            3 => count_3 += 1,
            _ => (),
        }
    }
    count_3 += 1;
    println!("Part 1: {}", count_1 * count_3);
    println!("Runtime {} us", now.elapsed().as_micros());

    // Part 2 - Method is similar to Fibonacci. For every element el, number of ways of getting to el #el is the sum of #[el - 1] if el - 1 exists,
    // #[el -2] if el - 2 exists, and #[el - 3] if el - 3 exists.
    // For example: If we have [0, 1, 4, 5, 6, 7] =>
    // #[0] = 1
    // #[1] = #[0],
    // #[4] = #[1],
    // #[5] = #[4],
    // #[6] = #[5] + #[4],
    // #[7] = #[6] + #[5] + #S[4]
    let now = Instant::now();
    let mut num_steps: Vec<i64> = Vec::new();
    let mut inp = vec![0];
    inp.extend(&input);
    let input = inp;
    num_steps.push(1);
    num_steps.push(1);
    for i in 2..input.len() {
        //println!("{:?}", num_steps);
        let mut tmp = 0;
        let permitted_jolts = vec![input[i] - 1, input[i] - 2, input[i] - 3];
        for k in 0..min(3, num_steps.len()) {
            if permitted_jolts.contains(&input[i - k - 1]) {
                tmp += num_steps[num_steps.len() - k - 1];
            }
        }
        num_steps.push(tmp);
    }
    println!("Part 2: {}", num_steps[num_steps.len() - 1]);
    println!("Runtime {} us", now.elapsed().as_micros());
}
