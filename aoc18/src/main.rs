use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error, ErrorKind};
use std::time::Instant;

fn read_input(filename: &str) -> Result<Vec<Vec<u8>>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines()
        .map(|l| l.and_then(|v| Ok(v.into_bytes())))
        .collect()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum StackVal {
    Add,
    Mult,
    Num(u64),
}

fn evaluate_expression(input: &Vec<u8>) -> u64 {
    let mut stack: Vec<StackVal> = Vec::new();
    let mut result: u64 = 0;
    let mut number = 0;
    let mut operator: StackVal = StackVal::Add;
    let is_num = |x: &u8| x >= &48 && x <= &57;
    let to_num = |x: &u8| (*x - 48) as u64;
    for c in input.iter() {
        //println!("{} {} {:?} {:?}", result, number, operator, stack);
        if is_num(c) {
            number = to_num(c);
        } else if c == &43 || c == &42 {
            // '+' or '*'
            if number != 0 {
                match operator {
                    StackVal::Add => result += number,
                    StackVal::Mult => result *= number,
                    StackVal::Num(v) => panic!("unexpected result {}", v),
                }
            }
            number = 0;
            match c {
                &43 => operator = StackVal::Add,
                &42 => operator = StackVal::Mult,
                _ => (),
            };
        } else if c == &40 {
            // '('
            stack.push(StackVal::Num(result));
            stack.push(operator);
            result = 0;
            operator = StackVal::Add;
        //println!("Stack at ( {:?}", stack);
        } else if c == &41 {
            // ')'
            //println!("Stack at ) {:?}", stack);
            //println!(
            //    "Result, operator, Number at ): {} {:?} {}",
            //    result, operator, number
            //);
            if number != 0 {
                match operator {
                    StackVal::Add => result += number,
                    StackVal::Mult => result *= number,
                    StackVal::Num(v) => panic!("Unexpected value {}", v),
                };
            }
            match stack.pop().unwrap() {
                StackVal::Add => {
                    if let StackVal::Num(val) = stack.pop().unwrap() {
                        result += val;
                    }
                }
                StackVal::Mult => {
                    if let StackVal::Num(val) = stack.pop().unwrap() {
                        result *= val;
                    }
                }
                StackVal::Num(v) => panic!("Unexpected value {}", v),
            }
            number = 0;
        } else {
        }
    }
    if number != 0 {
        match operator {
            StackVal::Add => result += number,
            StackVal::Mult => result *= number,
            StackVal::Num(v) => panic!("Unexpected number: {}", v),
        }
    }
    result
}

fn evaluate_expression_part2(mut input: Vec<u8>) -> u64 {
    // Remove all spaces
    input.retain(|v| v != &32);
    //println!("{:?}", input);
    let mut stack: Vec<StackVal> = Vec::new();
    let mut result: u64 = 0;
    let mut number = 0;
    let mut operator: StackVal = StackVal::Add;
    let is_num = |x: &u8| x >= &48 && x <= &57;
    let to_num = |x: &u8| (*x - 48) as u64;
    for (i, c) in input.iter().enumerate() {
        //println!("{} {} {:?} {:?}", result, number, operator, stack);
        //println!("{} {} {:?}", i, c, operator);
        if is_num(c) {
            number = to_num(c);
        } else if c == &43 || c == &42 {
            // '+' or '*'
            if number != 0 {
                match operator {
                    StackVal::Add => {
                        result += number;
                        number = 0;
                    }
                    StackVal::Mult => {
                        if c == &43 {
                            stack.push(StackVal::Num(result));
                            stack.push(StackVal::Mult);
                            result = number;
                        //number = 0;
                        } else {
                            result *= number;
                            //number = 0;
                        }
                    }
                    StackVal::Num(v) => panic!("unexpected result {}", v),
                }
            }
            number = 0;
            match c {
                &43 => operator = StackVal::Add,
                &42 => operator = StackVal::Mult,
                _ => (),
            };
        } else if c == &40 {
            // '('
            if result != 0 {
                stack.push(StackVal::Num(result));
                stack.push(operator);
                result = 0;
                operator = StackVal::Add;
            }
        //println!("Stack at ( {:?}", stack);
        } else if c == &41 {
            // ')'
            //println!("Stack at ) {:?}", stack);
            //println!(
            //    "Result, operator, Number at ): {} {:?} {}",
            //    result, operator, number
            //);
            if number != 0 {
                match operat
                or {
                    StackVal::Add => result += number,
                    StackVal::Mult => result *= number,
                    StackVal::Num(v) => panic!("Unexpected value {}", v),
                };
            }
            match stack.pop().unwrap() {
                StackVal::Add => {
                    if let StackVal::Num(val) = stack.pop().unwrap() {
                        result += val;
                    }
                }
                StackVal::Mult => {
                    if let StackVal::Num(val) = stack.pop().unwrap() {
                        result *= val;
                    }
                }
                StackVal::Num(v) => panic!("Unexpected value {}", v),
            }
            number = 0;
        } else {
            //println!("hello");
        }
    }
    if number != 0 {
        match operator {
            StackVal::Add => result += number,
            StackVal::Mult => result *= number,
            StackVal::Num(v) => panic!("Unexpected number: {}", v),
        }
    }
    if stack.len() != 0 {
        //println!("{:?}", stack);
        let mut ind: i64 = stack.len() as i64 - 1;
        while ind >= 0 {
            let op = stack[ind as usize];
            if let StackVal::Num(val) = stack[ind as usize - 1] {
                match op {
                    StackVal::Add => result += val,
                    StackVal::Mult => result *= val,
                    _ => panic!("Not an operator"),
                }
            }
            ind -= 2;
        }
    }
    result
}

fn main() {
    let input = read_input("input").unwrap();
    let testcases = vec![
        //"1 + 2 * 3 + 4 * 5 + 6",
        //"1 + (2 * 3) + (4 * (5 + 6))",
        //"2 * 3 + (4 * 5)",
        //"5 + (8 * 3 + 9 + 3 * 4 * 3)",
        //"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
        //"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
        "(6 + 9 * 9 + 7 * (5 * 7 * 6 + 9 * 5 + 2) + 3) * (2 + 4 * (8 * 3 + 2 * 4 * 2 * 8) + 9) * (6 + 6 * 2 * 6) + 5" // Right answer: 2027330665920
    ];
    let mut sum = 0;
    for v in input.iter() {
        sum += evaluate_expression(&v);
    }
    println!("Part 1: {}", sum);
    for v in testcases.iter() {
        println!(
            "{}",
            evaluate_expression_part2(String::from(*v).into_bytes())
        )
    }
    let mut sum = 0;
    for v in input.into_iter() {
        sum += evaluate_expression_part2(v);
    }
    println!("part 2: {}", sum);
}
