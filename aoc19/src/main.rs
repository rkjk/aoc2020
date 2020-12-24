use regex::Regex;
use std::cmp::max;
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

fn get_parsed_input(input: &Vec<String>) -> (HashMap<u64, Vec<String>>, Vec<String>) {
    let mut rules = HashMap::new();
    let mut messages = Vec::new();

    let mut line = input.into_iter();
    loop {
        let n = line.next().unwrap();
        if n == &"".to_string() {
            break;
        }
        let ll: Vec<&str> = n.split(":").collect();
        let rule = ll[1].replace(&['"'][..], "");
        let rule: Vec<&str> = rule.trim().split(" ").collect();
        let rule = rule.into_iter().map(|c| c.to_string()).collect();
        rules.insert(ll[0].parse::<u64>().unwrap(), rule);
    }
    for val in line {
        messages.push(val.clone());
    }
    (rules, messages)
}

fn get_parsed_input_parsed2(input: &Vec<String>) -> (HashMap<u64, Vec<String>>, Vec<String>) {
    let mut rules = HashMap::new();
    let mut messages = Vec::new();

    let mut line = input.into_iter();
    loop {
        let n = line.next().unwrap();
        if n == &"".to_string() {
            break;
        }
        let ll: Vec<&str> = n.split(":").collect();
        let rule = ll[1].replace(&['"'][..], "");
        let rule: Vec<&str> = rule.trim().split(" ").collect();
        let rule = rule.into_iter().map(|c| c.to_string()).collect();
        rules.insert(ll[0].parse::<u64>().unwrap(), rule);
    }
    let mut max_len = 0;
    for val in line {
        max_len = max(max_len, val.len());
        messages.push(val.clone());
    }
    // Update rules for 8 and 11
    // Rule 8 is basically (42)+31
    let mut rule_8 = vec!["42".to_string(), "+".to_string()];
    // Rule 11 Bongu: This is a recursive regex like: 42 31 | 42 42 31 31 | 42 42 42 31 31 31 |....... infinity
    // But we don't have to go out to infinity since the length of our strings are capped.
    // We could find the max-length of the substrings 42 and 31 and use that to get a limit on 11.
    // Instead start from 3 (i.e upto 42 42 42 | 31 31 31 say) and keep going up till the number of matches does not change
    // In our case, 6 is good enough i.e 41 32 | ..... | 42 42 42 42 42 42 31 31 31 31 31 31.
    let mut rule_11 = Vec::new();
    for i in 1..6 {
        for _ in 0..i {
            rule_11.push("42".to_string());
        }
        for _ in 0..i {
            rule_11.push("31".to_string())
        }
        rule_11.push("|".to_string());
    }
    rule_11.pop();
    rules.insert(8, rule_8);
    rules.insert(11, rule_11);
    //println!("{:?}", rule_8);
    //println!("{:?}", rule_11);
    (rules, messages)
}

struct Messages {
    rules: HashMap<u64, Vec<String>>,
    messages: Vec<String>,
}

impl Messages {
    fn get_expanded_rule(&self, node: u64) -> String {
        let rule_vals = self.rules.get(&node).unwrap();
        //println!("node: {}, rule_vals: {:?}", node, rule_vals);
        let mut expanded_rule: Vec<String> = Vec::new();
        let mut expansion_flag = false;
        for val in rule_vals.iter() {
            let new_rule = match val.parse::<u64>() {
                Ok(v) => {
                    expansion_flag = true;
                    self.get_expanded_rule(v)
                }
                Err(_) => {
                    //println!("Not number: {}", val);
                    (*val).to_string()
                }
            };
            expanded_rule.push(new_rule);
        }
        //println!(
        //    "node: {}, rule_vals: {:?}, expanded: {:?}",
        //    node, rule_vals, expanded_rule
        //);
        "(?:".to_string() + &expanded_rule.join(" ").replace(&[' '][..], "").to_string() + ")"
    }

    fn get_num_matches(&self, rule: Regex) -> u64 {
        self.messages.iter().filter(|v| rule.is_match(v)).count() as u64
    }
}

fn main() {
    let input = read_input("input").unwrap();
    let (rules, messages) = get_parsed_input(&input);
    let mut messages = Messages {
        rules: rules,
        messages: messages,
    };
    let rule = messages.get_expanded_rule(0);
    let rule = "^".to_string() + &rule + "$";
    let rule = Regex::new(&rule).unwrap();
    let num_matches = messages.get_num_matches(rule);
    println!("Part 1: {}", num_matches);
    let input = read_input("input").unwrap();
    let (rules2, messages2) = get_parsed_input_parsed2(&input);
    let mut messages = Messages {
        rules: rules2,
        messages: messages2,
    };
    let rule = messages.get_expanded_rule(0);
    let rule = "^".to_string() + &rule + "$";
    let rule = Regex::new(&rule).unwrap();
    let num_matches = messages.get_num_matches(rule);
    println!("Part 2: {}", num_matches);
}
