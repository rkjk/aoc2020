use std::collections::HashSet;
use std::time::Instant;

struct CupGame {
    circle: Vec<u64>,
    cache: HashSet<Vec<u64>>,
}

impl CupGame {
    fn run_game(&mut self, num_iter: usize) -> Vec<u64> {
        let mut current_cup_ind = 0;
        let min_val = self.circle.iter().min().unwrap().clone();
        let max_val = self.circle.iter().max().unwrap().clone();
        let circle_len = self.circle.len().clone();
        for i in 0..num_iter {
            // Current cup
            let current_cup = self.circle[current_cup_ind];

            // Pick next 3 elements
            let mut picks: Vec<u64> = Vec::new();
            // Remove picks from circle

            // Drain the elements in the range
            if current_cup_ind + 4 <= self.circle.len() {
                picks = self.circle[(current_cup_ind + 1)..(current_cup_ind + 4)].to_vec();
                self.circle
                    .drain((current_cup_ind + 1)..(current_cup_ind + 4));
            } else {
                if current_cup_ind < self.circle.len() {
                    picks.extend(self.circle[(current_cup_ind + 1)..circle_len].to_vec());
                    self.circle.drain((current_cup_ind + 1)..circle_len);
                }
                let low_lim = current_cup_ind + 4 - circle_len;
                picks.extend(self.circle[0..low_lim].to_vec());
                self.circle.drain(0..low_lim);
            }
            assert!(picks.len() == 3);
            current_cup_ind = self.circle.iter().position(|&v| v == current_cup).unwrap();
            //println!("Current_cup: {} picks: {:?}", current_cup, picks,);
            //println!("Circle after: {:?}", self.circle);
            let mut destination_cup = current_cup - 1;
            loop {
                if destination_cup < min_val {
                    destination_cup = max_val;
                }
                if !picks.contains(&destination_cup) {
                    break;
                }
                destination_cup -= 1;
            }
            //println!("Destination cup: {}", destination_cup);
            let destination_ind = self
                .circle
                .iter()
                .position(|&v| v == destination_cup)
                .unwrap();
            //println!(
            //    "destination: {} destination_index: {}",
            //    destination_cup, destination_ind
            //);
            let mut new_vec = self.circle[..(destination_ind + 1)].to_vec();
            new_vec.extend(picks.iter());
            if destination_ind + 1 < self.circle.len() {
                new_vec.extend(self.circle[(destination_ind + 1)..].to_vec());
            }
            self.circle = new_vec;
            //println!("Rebuild circle: {:?}", self.circle);
            if destination_ind > current_cup_ind {
                current_cup_ind = (current_cup_ind + 1) % self.circle.len();
            } else {
                current_cup_ind = (current_cup_ind + 4) % self.circle.len();
            }
            //println!("New current cup index: {}", current_cup_ind);
            //println!("");
        }
        self.circle.clone()
    }
}

fn main() {
    //let input = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
    let input = vec![5, 8, 9, 1, 7, 4, 2, 6, 3];
    let mut cupgame = CupGame {
        circle: input.clone(),
        cache: HashSet::new(),
    };
    let mut res = cupgame.run_game(100);
    let one = res.iter().position(|&v| v == 1).unwrap();
    let mut ans_vec = res[(one + 1)..].to_vec();
    ans_vec.extend(res[..one].to_vec());
    println!("Part 1: {:?}", ans_vec);

    let mut new_input = input.clone();
    for i in 10..1000001 {
        new_input.push(i);
    }
    let mut new_cupgame = CupGame {
        circle: new_input,
        cache: HashSet::new(),
    };
    let res = new_cupgame.run_game(1000);
    println!("{}", res.len());
    let one = res.iter().position(|&v| v == 1).unwrap();
    println!("Part 2: {} {}", res[one + 1], res[one + 2]);
    println!("Part 2: {}", res[one + 1] * res[one + 2]);
}
