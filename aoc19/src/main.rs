use regex::Regex;
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

struct Messages {
    rules: HashMap<u64, Vec<String>>,
    messages: Vec<String>,
}

impl Messages {
    fn get_expanded_rule(&self, node: u64) -> String {
        let rule_vals = self.rules.get(&node).unwrap();
        //println!("node: {}, rule_vals: {:?}", node, rule_vals);
        let mut expanded_rule: Vec<String> = Vec::new();
        for val in rule_vals.iter() {
            let new_rule = match val.parse::<u64>() {
                Ok(v) => self.get_expanded_rule(v),
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
        expanded_rule.join(" ").replace(&[' '][..], "").to_string()
    }
}

fn main() {
    let input = read_input("ex1").unwrap();
    let (rules, messages) = get_parsed_input(&input);
    let mut messages = Messages {
        rules: rules,
        messages: messages,
    };
    println!("{:?}", messages.get_expanded_rule(0));
}
