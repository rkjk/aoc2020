use std::collections::{HashMap, HashSet};
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

struct Tickets {
    ticket_fields: HashMap<String, Vec<Vec<u64>>>,
    your_ticket: Vec<u64>,
    row_vecs: Vec<Vec<u64>>,
}

impl Tickets {
    /// Return HashMap of Ticket fields to valid ranges
    /// Your ticket as a vector
    /// Vector of all columns of other tickets -> So a vector of vectors
    fn parse_input_part1(&mut self, input: &Vec<String>) {
        let mut ind = 0;
        loop {
            // First empty line indicates ticket fields section is over
            if input[ind] == "".to_string() {
                ind += 1;
                break;
            }
            let line = &input[ind];
            let fields: Vec<&str> = line.trim().split(":").collect();
            let f = fields[0].trim().to_owned();
            let ranges: Vec<Vec<u64>> = fields[1]
                .trim()
                .split("or")
                .map(|r| r.trim().split("-").map(|x| x.parse().unwrap()).collect())
                .collect();
            self.ticket_fields.insert(f, ranges);
            ind += 1;
        }
        // Your ticket
        ind += 1;
        self.your_ticket = input[ind]
            .split(",")
            .map(|num| num.parse().unwrap())
            .collect();

        // Nearby tickets
        ind += 3;
        self.row_vecs = input[ind..]
            .iter()
            .map(|line| line.split(",").map(|x| x.parse().unwrap()).collect())
            .collect();
    }

    /// Iterate all nearby tickets and sum each invalid value
    fn part1(&self) -> u64 {
        self.row_vecs
            .iter()
            .map(|nearby_ticket: &Vec<u64>| {
                nearby_ticket
                    .iter()
                    .map(|val| {
                        for range in self.ticket_fields.values() {
                            //println!("{:?}", range);
                            for r in range.iter() {
                                if *val >= r[0] && *val <= r[1] {
                                    return 0;
                                }
                            }
                        }
                        return *val;
                    })
                    .sum::<u64>()
            })
            .sum()
    }

    /// To calculate which field is allocated to which column, do the following:
    /// 1. Remove all unacceptable tickets
    /// 2. Transpose the acceptable tickets to get a column vectors, each column being an unknown field
    /// 3. For each column get all valid fields for that column
    /// 4. Sort this vector by length (So that the the first element represents the column which has the fewest possible choices for fields).
    /// 5. Use backtracking to get the field assignment for each column
    fn part2(&self) -> u128 {
        let accepted_tickets: Vec<Vec<u64>> = self
            .row_vecs
            .iter()
            .cloned()
            .filter(|nearby_ticket: &Vec<u64>| {
                nearby_ticket.iter().all(|val| {
                    for range in self.ticket_fields.values() {
                        //println!("{:?}", range);
                        for r in range.iter() {
                            if *val >= r[0] && *val <= r[1] {
                                return true;
                            }
                        }
                    }
                    return false;
                })
            })
            .collect();
        //println!("{} {}", self.row_vecs.len(), accepted_tickets.len());
        let col_vecs: Vec<Vec<u64>> = (0..accepted_tickets[0].len())
            .map(|i| {
                accepted_tickets
                    .iter()
                    .map(|inner| inner[i].clone())
                    .collect()
            })
            .collect();
        let mut valid_fields: Vec<(Vec<String>, usize)> = Vec::new();
        for (i, col) in col_vecs.iter().enumerate() {
            let mut valid_keys = Vec::new();
            for (k, v) in self.ticket_fields.iter() {
                if self.valid(col, v) {
                    valid_keys.push(k.to_string());
                }
            }
            valid_fields.push((valid_keys, i));
        }
        valid_fields.sort_by(|a, b| a.0.len().cmp(&b.0.len()));
        let v = self.backtrack(&col_vecs, &valid_fields, &mut HashMap::new());

        let mut prod: u128 = 1;
        for (k, v) in v.unwrap().iter() {
            if k.starts_with("departure") {
                prod *= self.your_ticket[*v] as u128;
            }
        }
        prod
    }

    fn valid(&self, col_vec: &Vec<u64>, range: &Vec<Vec<u64>>) -> bool {
        col_vec.iter().all(|x| {
            (*x >= range[0][0] && *x <= range[0][1]) || (*x >= range[1][0] && *x <= range[1][1])
        })
    }

    fn backtrack(
        &self,
        col_vecs: &Vec<Vec<u64>>,
        valid_fields: &Vec<(Vec<String>, usize)>,
        seen_map: &mut HashMap<String, usize>,
    ) -> Option<HashMap<String, usize>> {
        let current_col = seen_map.len();
        if current_col >= self.your_ticket.len() {
            return Some(seen_map.clone());
        }
        for key in valid_fields[current_col].0.iter() {
            if seen_map.get(key) != None {
                continue;
            }
            //seen_vec.push(key.to_string());
            seen_map.insert(key.to_string(), valid_fields[current_col].1);
            match self.backtrack(col_vecs, valid_fields, seen_map) {
                Some(v) => return Some(v),
                None => {
                    seen_map.remove(key);
                }
            }
        }
        None
    }
}

fn main() {
    let input = read_input("input").unwrap();
    let mut tickets = Tickets {
        ticket_fields: HashMap::new(),
        your_ticket: vec![],
        row_vecs: vec![],
    };
    tickets.parse_input_part1(&input);
    let now = Instant::now();
    println!("Part 1: {}", tickets.part1());
    println!("Runtime {} ms", now.elapsed().as_millis());
    let now = Instant::now();
    println!("Part 2: {}", tickets.part2());
    println!("Runtime {} ms", now.elapsed().as_millis());
}
