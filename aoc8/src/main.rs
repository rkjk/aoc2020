use std::collections::HashSet;
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

type Op = (String, i64);

struct Console {
    accumulator: i64,
    instructions: Vec<Op>,
}

impl Console {
    fn run_one_cycle(&mut self) -> bool {
        let mut seen: HashSet<usize> = HashSet::new();
        // Instruction Index
        let mut ind: i32 = 0;
        while ind >= 0 as i32
            && ind < self.instructions.len() as i32
            && seen.get(&(ind as usize)) == None
        {
            let uind = ind as usize;
            seen.insert(uind);
            let instr = self.instructions[uind].0.clone();
            //println!("{} {:?} {}", ind, self.instructions[uind], self.accumulator);
            match &instr[..] {
                "acc" => {
                    self.accumulator += self.instructions[uind].1;
                    ind += 1
                }
                "nop" => ind += 1,
                "jmp" => ind += self.instructions[uind].1 as i32,
                _ => panic!("Unknown state"),
            }
        }
        if ind == self.instructions.len() as i32 {
            return true;
        }
        false
    }

    fn get_nop_jmp_indices(&self) -> Vec<usize> {
        let mut res = Vec::new();
        for (i, val) in self.instructions.iter().enumerate() {
            if &val.0[..] == "nop" || &val.0[..] == "jmp" {
                res.push(i);
            }
        }
        res
    }
}

fn get_parsed_input(input: &Vec<String>) -> Vec<Op> {
    let mut instructions = Vec::new();
    for line in input.iter() {
        let tup: Vec<&str> = line.trim().split(" ").collect();
        let op = (tup[0].to_string(), tup[1].parse().unwrap());
        instructions.push(op);
    }
    instructions
}

fn main() {
    let input = read_input("input").unwrap();
    let input = get_parsed_input(&input);

    let mut console = Console {
        accumulator: 0,
        instructions: input,
    };
    console.run_one_cycle();
    println!(
        "Part 1: Accumulator before second cycle: {}",
        console.accumulator
    );

    let nop_jmp = console.get_nop_jmp_indices();
    for ind in nop_jmp.iter() {
        let orig_val;
        console.accumulator = 0;
        match &console.instructions[*ind].0[..] {
            "nop" => {
                console.instructions[*ind].0 = "jmp".to_string();
                orig_val = "nop".to_string()
            }
            "jmp" => {
                console.instructions[*ind].0 = "nop".to_string();
                orig_val = "jmp".to_string();
            }
            _ => panic!("State not possible"),
        };
        match console.run_one_cycle() {
            true => {
                println!("Part 2: {}", console.accumulator);
                break;
            }
            false => console.instructions[*ind].0 = orig_val,
        };
    }
}
