use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error, ErrorKind};

type Passport = HashMap<String, String>;

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines()
        .map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

// Read Input file and produce a vector of passports
fn get_passports(input: &Vec<String>) -> Vec<Passport> {
    let mut passports = Vec::new();
    let mut passport = HashMap::new();

    for line in input.iter() {
        if line == "" {
            passports.push(passport);
            passport = HashMap::new();
            continue;
        }
        let key_values = line.split(" ");
        for kvs in key_values {
            let kv: Vec<&str> = kvs.split(":").collect();
            passport.insert(kv[0].to_string(), kv[1].to_string());
        }
    }
    if passport.len() > 0 {
        passports.push(passport);
    }
    passports
}

/// Get valid Passports. Check for following fields
///
/// byr (Birth Year)
/// iyr (Issue Year)
/// eyr (Expiration Year)
/// hgt (Height)
/// hcl (Hair Color)
/// ecl (Eye Color)
/// pid (Passport ID)
/// cid (Country ID) - Optional
fn get_valid_passports(passports: &Vec<Passport>) -> Vec<Passport> {
    let mut valid_passports: Vec<Passport> = Vec::new();
    let keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for pass in passports {
        match keys.iter().all(|k| pass.contains_key(&k[..])) {
            true => valid_passports.push(pass.clone()),
            false => (),
        }
    }
    valid_passports
}

fn check_number_range(s: &str, l: usize, lo: u64, hi: u64) -> bool {
    if s.len() != l {
        return false;
    }
    let parsed_s = s.parse::<u64>();
    match parsed_s {
        Ok(val) => {
            if val < lo || val > hi {
                return false;
            }
        }
        Err(_) => {
            println!("Parsing Error: {}", s);
            return false;
        }
    }
    return true;
}

fn check_passport_validity(pass: &Passport) -> bool {
    let (byr, iyr, eyr) = (
        pass.get("byr").unwrap(),
        pass.get("iyr").unwrap(),
        pass.get("eyr").unwrap(),
    );
    if !check_number_range(byr, 4, 1920, 2002)
        || !check_number_range(iyr, 4, 2010, 2020)
        || !check_number_range(eyr, 4, 2020, 2030)
    {
        println!("byr: {} iyr: {} eyr: {}", byr, iyr, eyr);
        return false;
    }
    let hgt = pass.get("hgt").unwrap();
    match hgt.chars().last() {
        Some('m') => match hgt.chars().nth(hgt.len() - 2) {
            Some('c') => {
                if !check_number_range(&hgt[0..(hgt.len() - 2)], 3, 150, 193) {
                    println!("false number range: {}", hgt);
                    return false;
                }
            }
            Some(_) => {
                println!("not a number: {}", hgt);
                return false;
            }
            None => {
                println!("None: {}", hgt);
                return false;
            }
        },
        Some('n') => match hgt.chars().nth(hgt.len() - 2) {
            Some('i') => {
                if !check_number_range(&hgt[0..(hgt.len() - 2)], 2, 59, 76) {
                    println!("false number range: {}", hgt);
                    return false;
                }
            }
            Some(_) => {
                println!("not a number: {}", hgt);
                return false;
            }
            None => {
                println!("None: {}", hgt);
                return false;
            }
        },
        Some(_) => return false,
        None => return false,
    }

    let valid_ecl = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let ecl = pass.get("ecl").unwrap();
    if !valid_ecl.iter().any(|x| x == ecl) {
        //println!("ECL not in valid values: {}", ecl);
        return false;
    }
    let hcl = pass.get("hcl").unwrap();
    let hcl_re = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
    if !hcl_re.is_match(hcl) {
        println!("hcl regex: {}", hcl);
        return false;
    }

    let pid = pass.get("pid").unwrap();
    let pid_re = Regex::new(r"^[0-9]{9}$").unwrap();
    if !pid_re.is_match(pid) {
        println!("pid regex: {}", pid);
        return false;
    }
    true
}

fn main() {
    let input = read_input("input").unwrap();
    println!("Num passports: {}", input.len());
    let passports = get_passports(&input);
    //println!(
    //   "part 1 - Valid Passports: {}",
    //    get_valid_passports(&passports).len()
    //);
    let valid_passports = get_valid_passports(&passports);

    let mut count = 0;
    for pass in valid_passports {
        if check_passport_validity(&pass) {
            count += 1;
            //println!("{:?}", pass);
        }
    }
    println!("part 2: {}", count);
}
