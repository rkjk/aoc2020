use std::collections::{HashSet, VecDeque};
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

fn get_parsed_input(input: &Vec<String>) -> (VecDeque<u64>, VecDeque<u64>) {
    let mut players = vec![VecDeque::new(), VecDeque::new()];
    let mut ind = 0;
    let mut it = input.iter();
    loop {
        let line = it.next();
        match line {
            Some(_) => (),
            None => break,
        };
        let line = line.unwrap();
        if line == "" {
            ind = 1;
            continue;
        }
        if line.starts_with("Player") {
            continue;
        }
        players[ind].push_back(line.parse().unwrap());
    }
    (players.remove(0), players.remove(0))
}

struct Combat {
    player_1: VecDeque<u64>,
    player_0: VecDeque<u64>,
    deck_cache: HashSet<(VecDeque<u64>, VecDeque<u64>)>,
}

impl Combat {
    fn run_brute_force(&self) -> u64 {
        let mut res = 0;
        let (mut player_0, mut player_1) = (self.player_0.clone(), self.player_1.clone());
        while player_0.len() > 0 && player_1.len() > 0 {
            let (c0, c1) = (player_0.pop_front().unwrap(), player_1.pop_front().unwrap());
            match c0 > c1 {
                true => {
                    player_0.push_back(c0);
                    player_0.push_back(c1);
                }
                false => {
                    player_1.push_back(c1);
                    player_1.push_back(c0);
                }
            }
        }
        let player = match player_0.len() == 0 {
            true => player_1,
            false => player_0,
        };
        let mut multiplier = player.len();
        for val in player.iter() {
            res += val * multiplier as u64;
            multiplier -= 1;
        }
        res
    }
}

struct RecursiveCombat {
    player_0: VecDeque<u64>,
    player_1: VecDeque<u64>,
    deck_cache: HashSet<(VecDeque<u64>, VecDeque<u64>)>,
}

impl RecursiveCombat {
    fn check_repeat(&self) -> bool {
        self.deck_cache
            .contains(&(self.player_0.clone(), self.player_1.clone()))
    }

    fn recursive_combat(&mut self) -> u64 {
        while self.player_0.len() > 0 && self.player_1.len() > 0 {
            if self.check_repeat() {
                return 0;
            }
            self.deck_cache
                .insert((self.player_0.clone(), self.player_1.clone()));
            let (c0, c1) = (
                self.player_0.pop_front().unwrap(),
                self.player_1.pop_front().unwrap(),
            );
            // Recurse if both players have at least as many cards as their draw
            if c0 <= self.player_0.len() as u64 && c1 <= self.player_1.len() as u64 {
                let mut player_0_clone = self.player_0.clone();
                player_0_clone.resize(c0 as usize, 0);
                let mut player_1_clone = self.player_1.clone();
                player_1_clone.resize(c1 as usize, 0);
                let mut new_recursive = RecursiveCombat {
                    player_0: player_0_clone,
                    player_1: player_1_clone,
                    deck_cache: HashSet::new(),
                };
                match new_recursive.recursive_combat() {
                    0 => {
                        self.player_0.push_back(c0);
                        self.player_0.push_back(c1);
                    }
                    1 => {
                        self.player_1.push_back(c1);
                        self.player_1.push_back(c0);
                    }
                    _v => panic!("Value not supposed to be returned: {}", _v),
                }
            } else {
                match c0 > c1 {
                    true => {
                        self.player_0.push_back(c0);
                        self.player_0.push_back(c1);
                    }
                    false => {
                        self.player_1.push_back(c1);
                        self.player_1.push_back(c0);
                    }
                }
            }
        }
        if self.player_0.len() != 0 {
            return 0;
        }
        1
    }
}

fn main() {
    let input = read_input("input").unwrap();
    let (player_0, player_1) = get_parsed_input(&input);
    //println!("Player 0: {:?} Player 1: {:?}", player_0, player_1);
    let mut combat = Combat {
        player_0: player_0.clone(),
        player_1: player_1.clone(),
        deck_cache: HashSet::new(),
    };
    println!("Part 1:{}", combat.run_brute_force());
    let mut recursive_combat = RecursiveCombat {
        player_0: player_0,
        player_1: player_1,
        deck_cache: HashSet::new(),
    };
    let v = recursive_combat.recursive_combat();
    let mut res = 0;
    match v {
        0 => {
            println!("Player 1 won: {:?}", recursive_combat.player_0);
            let mut multiplier = recursive_combat.player_0.len();
            for val in recursive_combat.player_0 {
                res += multiplier as u64 * val;
                multiplier -= 1;
            }
        }
        1 => {
            println!("Player 2 won: {:?}", recursive_combat.player_1);
            let mut multiplier = recursive_combat.player_1.len();
            for val in recursive_combat.player_1 {
                res += multiplier as u64 * val;
                multiplier -= 1;
            }
        }
        _v => panic!("Value not supposed to be returned: {}", _v),
    }
    println!("Part 2: {}", res);
}
