use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error, ErrorKind};
use std::time::Instant;

struct XMAS {
    order: VecDeque<i64>,
    dict: HashSet<i64>,
}

impl XMAS {
    fn new() -> Self {
        XMAS {
            order: VecDeque::new(),
            dict: HashSet::new(),
        }
    }

    fn get_first_number(&mut self, preamble: usize, input: &Vec<i64>) -> i64 {
        // Sentinel
        self.order.push_back(-1);
        for i in 0..preamble {
            self.order.push_back(input[i]);
            self.dict.insert(input[i]);
        }
        for val in input[preamble..].iter() {
            let front = self.order.pop_front().unwrap();
            self.dict.remove(&front);
            let mut flag = false;
            for el in self.order.iter() {
                //println!("{} {} {:?} {:?}", *val, el, self.order, self.dict);
                if val - *el != *el && self.dict.contains(&(val - *el)) {
                    self.order.push_back(*val);
                    self.dict.insert(*val);
                    flag = true;
                    break;
                }
            }
            if flag == false {
                return *val;
            }
        }
        -1
    }

    // Brute Force: Get a prefix sum and then check all possible pairs for the total: O(N^2) time and O(N) space
    fn brute_force_prefix_sum(&self, total: i64, input: &Vec<i64>) -> i64 {
        let mut prefix_sum: Vec<i64> = Vec::new();
        let mut dict: HashMap<i64, usize> = HashMap::new();
        prefix_sum.push(input[0]);
        for i in 1..input.len() {
            let last = prefix_sum[prefix_sum.len() - 1];
            let sumval = last + input[i];
            prefix_sum.push(sumval);
            dict.insert(sumval, i);
        }
        //println!("{:?}", prefix_sum);
        for i in 1..prefix_sum.len() {
            for j in 0..i {
                //println!("{} {} {} {}", i, prefix_sum[i], j, prefix_sum[j]);
                if prefix_sum[i] - prefix_sum[j] == total {
                    let mut s_vec = input[(j + 1)..(i + 1)].to_vec();
                    s_vec.sort();
                    return s_vec[0] + s_vec[s_vec.len() - 1];
                }
            }
        }
        -1
    }

    // Use a 2-pointer solution: O(N) time and O(1) space
    fn two_pointer(&self, total: i64, input: &Vec<i64>) -> i64 {
        let mut sumval = 0;
        let (mut left, mut right) = (0, 0);
        while right < input.len() && left <= right {
            //println!("{}", sumval);
            if sumval == total {
                let mut s_vec = input[(left)..(right + 1)].to_vec();
                s_vec.sort();
                return s_vec[0] + s_vec[s_vec.len() - 1];
            } else if sumval < total {
                sumval += input[right];
                right += 1;
            } else {
                sumval -= input[left];
                left += 1;
            }
        }
        -1
    }
}

fn read_input(filename: &str) -> Result<Vec<i64>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines()
        .map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() {
    let input = read_input("input").unwrap();
    let mut xmas = XMAS::new();
    let now = Instant::now();
    let part1 = xmas.get_first_number(25, &input);
    println!("Part 1: {}", part1);
    println!("Runtime {} us", now.elapsed().as_micros());
    let now = Instant::now();
    println!("Part 2: {}", xmas.brute_force_prefix_sum(part1, &input));
    println!("Runtime {} us", now.elapsed().as_micros());
    let now = Instant::now();
    println!("Part 2 optimized: {}", xmas.two_pointer(part1, &input));
    println!("Runtime {} us", now.elapsed().as_micros());
}
