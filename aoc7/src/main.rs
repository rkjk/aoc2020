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

// Parse Each line as a HashMap entry -> Key: Bag, Value: HashMap<Bag, number of bags>
fn get_parsed_input(input: &Vec<String>) -> HashMap<String, HashMap<String, u64>> {
    let mut set = HashMap::new();
    for line in input.iter() {
        let tmp: Vec<&str> = line.split("contain").collect();
        let source: Vec<&str> = tmp[0].trim().split(" ").collect();
        //println!("{:?}", source);
        let source = source[..2].join(" ");

        let target: Vec<&str> = tmp[1].trim().split(",").collect();
        //println!("{:?}", target);
        let mut target_map = HashMap::new();
        for tar in target.iter() {
            let tmp_in: Vec<&str> = tar.trim().split(" ").collect();
            if tmp_in[0] == "no" {
                continue;
            }
            let num = tmp_in[0].parse().unwrap();
            let bag = tmp_in[1..3].join(" ");
            target_map.insert(bag, num);
        }
        set.insert(source, target_map);
    }
    set
}

// Find the number of bags that eventually contain a Shiny Gold Bag. Basically DFS
fn get_bags_contain_shiny_gold(
    current_node: String,
    visited: &mut HashMap<String, bool>,
    input: &HashMap<String, HashMap<String, u64>>,
) -> bool {
    let vis = visited.get(&current_node);
    match vis {
        Some(v) => return *v,
        None => (),
    };
    let val = input.get(&current_node);
    match val {
        Some(v) => {
            for key in v.keys() {
                if key == "shiny gold" {
                    return true;
                }
                if get_bags_contain_shiny_gold(key.clone(), visited, input) {
                    visited.insert(current_node, true);
                    return true;
                }
            }
        }
        None => return false,
    }
    visited.insert(current_node, false);
    false
}

// Find the total number of bags contained in a Shiny Gold Bag. DFS again
fn get_num_bags_in_shiny_gold(
    current_node: String,
    visited: &mut HashMap<String, u64>,
    input: &HashMap<String, HashMap<String, u64>>,
) -> u64 {
    let vis = visited.get(&current_node);
    match vis {
        Some(v) => return *v,
        None => (),
    };
    let val = input.get(&current_node);
    let mut sumval = 0;
    match val {
        Some(v) => {
            for key in v.keys() {
                let multiplier = v.get(key).unwrap();
                sumval += multiplier;
                let num_bags = get_num_bags_in_shiny_gold(key.to_string(), visited, input);
                //println!("{} {} {}", key, multiplier, num_bags);
                sumval += multiplier * num_bags;
            }
        }
        None => return 0,
    };
    visited.insert(current_node, sumval);
    sumval
}

fn main() {
    let input = read_input("input").unwrap();
    let input = get_parsed_input(&input);
    let mut visited: HashMap<String, bool> = HashMap::new();
    let mut count = 0;
    for node in input.keys() {
        if get_bags_contain_shiny_gold(node.to_string(), &mut visited, &input) {
            count += 1;
        }
    }
    println!("Part 1: {}", count);

    let mut visited: HashMap<String, u64> = HashMap::new();

    println!(
        "Part 2: {}",
        get_num_bags_in_shiny_gold("shiny gold".to_owned(), &mut visited, &input)
    );
}
