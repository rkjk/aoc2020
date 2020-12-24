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
        return "(?:".to_string()
            + &expanded_rule.join(" ").replace(&[' '][..], "").to_string()
            + ")";
        /*
        let ors: Vec<bool> = expanded_rule.iter().map(|c| c == "|").collect();
        if !ors.iter().any(|x| *x == true) && !expansion_flag {
            return expanded_rule.join(" ").replace(&[' '][..], "").to_string();
        }
        let mut non_capturing_groups = vec!["(?:".to_string()];
        for i in 0..expanded_rule.len() {
            match ors[i] {
                false => non_capturing_groups.push(expanded_rule[i].to_string()),
                true => {
                    non_capturing_groups.push(")".to_string());
                    non_capturing_groups.push(expanded_rule[i].to_string());
                    non_capturing_groups.push("(?:".to_string());
                }
            }
        }
        non_capturing_groups.push(")".to_string());
        non_capturing_groups
            .join(" ")
            .replace(&[' '][..], "")
            .to_string()
        */
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
}
