use itertools::{iproduct, Itertools};
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

#[derive(Debug, Clone)]
struct Memory {
    address: u64,
    value: u64,
}

#[derive(Debug, Clone)]
struct Instr {
    bitmask: String,
    prog: Vec<Memory>,
}

impl Instr {
    fn new() -> Self {
        Instr {
            bitmask: "".to_owned(),
            prog: Vec::new(),
        }
    }
}

fn get_vec(input: &Vec<String>) -> Vec<Instr> {
    let mut res: Vec<Instr> = Vec::new();
    let mut instr = Instr::new();
    for line in input.iter() {
        let val: Vec<&str> = line.split("=").collect();
        let instruction = val[0].trim();
        if instruction.starts_with("mask") {
            if instr.prog.len() > 0 {
                res.push(instr.clone());
                instr = Instr::new();
            }
            instr.bitmask = val[1].trim().to_owned();
        } else {
            let addr = instruction[4..(instruction.len() - 1)].parse().unwrap();
            let value = val[1].trim().parse().unwrap();
            instr.prog.push(Memory {
                address: addr,
                value: value,
            });
        }
    }
    res.push(instr.clone());
    res
}

struct Computer {
    current_mask: String,
    product: Vec<String>, // Number of Xs in current_mask
    memory: HashMap<u64, u64>,
}

impl Computer {
    fn compute(&mut self, program: &Vec<Instr>) -> u64 {
        for val in program.iter() {
            self.current_mask = val.bitmask.to_owned();
            for mem in val.prog.iter() {
                self.set_address_value(mem);
            }
        }
        //println!("{:?}", self.memory);
        self.memory.values().sum()
    }

    fn set_address_value(&mut self, prog: &Memory) {
        let mut num = prog.value;
        for (i, c) in self.current_mask.chars().rev().enumerate() {
            match c {
                '0' => {
                    num &= !(1 << i);
                    //println!("{} {}", i, c);
                }
                '1' => {
                    num |= 1 << i;
                    //println!("{} {}", i, c);
                }
                'X' => (),
                _ => panic!("Unknown character"),
            }
        }
        /*
        println!(
            "Current mask: {} value: {} after mask {}",
            self.current_mask, prog.value, num
        );
        */
        self.memory.insert(prog.address, num);
    }

    // Got function from Stackoverflow. Replicate itertools.product(*iterable, repeat=k) from python
    fn kproduct(&self, seq: String, k: u32) -> Vec<String> {
        match k {
            0 => vec![],
            1 => seq.chars().map(|c| c.to_string()).collect(),
            2 => iproduct!(seq.chars(), seq.chars())
                .map(|(a, b)| format!("{}{}", a, b))
                .collect(),
            _ => iproduct!(self.kproduct(seq.clone(), k - 1), seq.chars())
                .map(|(a, b)| format!("{}{}", a, b))
                .collect(),
        }
    }

    fn compute_cartesian_product(&mut self) {
        let num_X = self.current_mask.chars().filter(|x| *x == 'X').count() as u32;
        self.product = self.kproduct("01".to_string(), num_X);
    }

    fn compute_part2(&mut self, program: &Vec<Instr>) -> u64 {
        for val in program.iter() {
            self.current_mask = val.bitmask.to_owned();
            self.compute_cartesian_product();
            for mem in val.prog.iter() {
                self.set_address_value_part2(mem);
            }
        }
        self.memory.values().sum()
    }

    fn set_address_value_part2(&mut self, prog: &Memory) {
        let mut addr = prog.address;
        let mut X_positions = vec![];
        for (i, c) in self.current_mask.chars().rev().enumerate() {
            match c {
                '1' => addr |= 1 << i,
                'X' => X_positions.push(i),
                _ => (),
            }
        }
        let fixed_addr = addr;
        // Generate the new addresses
        let mut addresses: Vec<u64> = Vec::new();
        for floating in &self.product {
            let mut new_add = fixed_addr;
            for (c, pos) in floating.chars().zip(&X_positions) {
                match c {
                    '0' => {
                        new_add &= !(1 << pos);
                    }
                    '1' => {
                        new_add |= 1 << pos;
                    }
                    _ => (),
                }
            }
            addresses.push(new_add);
        }

        for addr in addresses.into_iter() {
            self.memory.insert(addr, prog.value);
        }
    }
}

fn main() {
    let input = read_input("input").unwrap();
    let program = get_vec(&input);
    let now = Instant::now();
    let mut computer = Computer {
        current_mask: "".to_string(),
        product: vec![],
        memory: HashMap::new(),
    };
    println!("Part 1: {}", computer.compute(&program));
    println!("Runtime {} ms", now.elapsed().as_millis());
    let now = Instant::now();
    let mut computer = Computer {
        current_mask: "".to_string(),
        product: vec![],
        memory: HashMap::new(),
    };
    println!("Part 2: {}", computer.compute_part2(&program));
    println!("Runtime {} ms", now.elapsed().as_millis());
}
