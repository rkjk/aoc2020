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

// Return a vector od groups where each group is a vector containing passengers' answered questions (String)
fn get_groups(input: &Vec<String>) -> Vec<Vec<String>> {
    let mut groups = Vec::new();
    let mut current_group = Vec::new();
    for line in input.iter() {
        if line.len() == 0 {
            groups.push(current_group.clone());
            current_group = Vec::new();
            continue;
        }
        current_group.push(line.to_string());
    }
    groups.push(current_group);
    groups
}

// Get all unique questions answered by group
fn get_answered_questions(group: &Vec<String>) -> HashSet<char> {
    let mut set = HashSet::new();
    for pass in group.iter() {
        for ques in pass.chars() {
            set.insert(ques);
        }
    }
    set
}

// Get all questions answered by all passengers in the group
fn get_all_answered_questions(group: &Vec<String>) -> Vec<HashSet<char>> {
    let mut vec = Vec::new();
    for pass in group.iter() {
        let mut set = HashSet::new();
        for ques in pass.chars() {
            set.insert(ques);
        }
        vec.push(set);
    }
    vec
}

fn main() {
    let input = read_input("input").unwrap();
    let groups = get_groups(&input);

    // Part 1
    let mut count = 0;
    for group in groups.iter() {
        count += get_answered_questions(group).len();
    }
    println!("part 1: {}", count);

    // Part 2
    let mut count_2 = 0;
    for group in groups.iter() {
        let hashset_vec: Vec<HashSet<char>> = get_all_answered_questions(group);
        let mut intersect = hashset_vec[0].clone();
        for val in hashset_vec[1..].iter() {
            intersect = intersect.intersection(val).copied().collect();
        }
        count_2 += intersect.len();
    }
    println!("Part 2: {}", count_2);
}
