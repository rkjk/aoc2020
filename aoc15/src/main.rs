use std::collections::HashMap;
use std::time::Instant;

fn brute_force(start: &Vec<u64>, iterations: usize) -> u64 {
    let mut store: HashMap<u64, Vec<usize>> = HashMap::new();
    let mut turn = 1;
    let mut last_seen = 0;
    for val in start.iter() {
        store.insert(*val, vec![turn]);
        last_seen = *val;
        turn += 1;
    }
    //println!("{}", last_seen);
    while turn < iterations + 1 {
        let x = store.get_mut(&last_seen).unwrap();
        //println!("{} {} {}", turn, last_seen, (*x).len());
        match (*x).len() == 1 {
            false => {
                let val = ((*x)[(*x).len() - 1] - (*x)[(*x).len() - 2]) as u64;
                match store.get_mut(&val) {
                    Some(y) => (*y).push(turn),
                    None => {
                        store.insert(val, vec![turn]);
                    }
                };
                last_seen = val;
                //println!("{} {}", val, turn);
            }
            true => {
                match store.get_mut(&0) {
                    Some(x) => (*x).push(turn),
                    None => {
                        store.insert(0, vec![turn]);
                        //println!("0 {}", turn);
                    }
                };
                last_seen = 0;
            }
        }
        turn += 1;
        //println!("{:?}", store);
    }
    last_seen
}

fn main() {
    //let input = vec![3, 1, 2];
    let input = vec![15, 5, 1, 4, 7, 0];
    let now = Instant::now();
    println!("Part 1: {}", brute_force(&input, 2020));
    println!("Runtime {} us", now.elapsed().as_micros());
    let now = Instant::now();
    println!("Part 2: {}", brute_force(&input, 30000000));
    println!("Runtime {} ms", now.elapsed().as_millis());
}
