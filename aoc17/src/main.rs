use std::collections::{HashMap, HashSet};
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

fn get_starting_configuration(input: &Vec<String>) -> HashSet<(i64, i64, i64)> {
    let mut active = HashSet::new();
    for (i, line) in input.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                active.insert((i as i64, j as i64, 0));
            }
        }
    }
    active
}

type cube = (i64, i64, i64);
type hypercube = (i64, i64, i64, i64);

struct ConwayCubes {
    active: HashSet<cube>,
}

struct ConwayHypercubes {
    active: HashSet<hypercube>,
}

impl ConwayCubes {
    fn run_cycle(&mut self) {
        let mut new_active: HashSet<cube> = HashSet::new();
        let mut old_inactive = HashSet::new();
        for node in self.active.iter() {
            //println!("Active node: {:?}", node);
            let neighbours = self.get_neighbours(node);
            match self.check_inactive(&neighbours) {
                false => {
                    new_active.insert(*node);
                    //println!("Node still active");
                }
                true => {
                    //println!("Node inactive");
                }
            };
            let inactive_nodes: Vec<cube> = neighbours
                .iter()
                .cloned()
                .filter(|val| !self.active.contains(val))
                .collect();
            //println!("Inactive nodes for node {:?} : {:?}", node, inactive_nodes);
            for val in inactive_nodes {
                old_inactive.insert(val);
            }
        }
        for node in old_inactive {
            //println!("Inactive Node: {:?}", node);
            let neighbours = self.get_neighbours(&node);
            match self.check_active(&neighbours) {
                true => {
                    //println!("Activated");
                    new_active.insert(node);
                }
                false => {
                    //println!("Still inactive");
                }
            };
        }
        //println!("{:?}", new_active);
        self.active = new_active;
    }

    fn get_neighbours(&self, node: &cube) -> Vec<cube> {
        let (x, y, z) = node.clone();
        vec![
            // Differnce of 1 in One coordinate
            (x - 1, y, z),
            (x + 1, y, z),
            (x, y - 1, z),
            (x, y + 1, z),
            (x, y, z - 1),
            (x, y, z + 1),
            // Difference of 1 in 2 coordinates
            (x - 1, y - 1, z),
            (x - 1, y + 1, z),
            (x + 1, y - 1, z),
            (x + 1, y + 1, z),
            (x - 1, y, z - 1),
            (x - 1, y, z + 1),
            (x + 1, y, z - 1),
            (x + 1, y, z + 1),
            (x, y - 1, z - 1),
            (x, y - 1, z + 1),
            (x, y + 1, z - 1),
            (x, y + 1, z + 1),
            // Difference of 1 in 3 coordinates
            (x - 1, y - 1, z - 1),
            (x - 1, y - 1, z + 1),
            (x - 1, y + 1, z - 1),
            (x - 1, y + 1, z + 1),
            (x + 1, y - 1, z - 1),
            (x + 1, y - 1, z + 1),
            (x + 1, y + 1, z - 1),
            (x + 1, y + 1, z + 1),
        ]
    }

    fn check_inactive(&self, neighbours: &Vec<cube>) -> bool {
        match neighbours
            .iter()
            .filter(|val| self.active.contains(&val))
            .count()
        {
            2 | 3 => return false,
            _ => return true,
        };
    }

    fn check_active(&self, neighbours: &Vec<cube>) -> bool {
        match neighbours
            .iter()
            .filter(|val| self.active.contains(&val))
            .count()
        {
            3 => return true,
            _ => return false,
        };
    }
}

impl ConwayHypercubes {
    fn run_cycle(&mut self) {
        let mut new_active: HashSet<hypercube> = HashSet::new();
        let mut old_inactive = HashSet::new();
        for node in self.active.iter() {
            //println!("Active node: {:?}", node);
            let neighbours = self.get_neighbours(node);
            match self.check_inactive(&neighbours) {
                false => {
                    new_active.insert(*node);
                    //println!("Node still active");
                }
                true => {
                    //println!("Node inactive");
                }
            };
            let inactive_nodes: Vec<hypercube> = neighbours
                .iter()
                .cloned()
                .filter(|val| !self.active.contains(val))
                .collect();
            //println!("Inactive nodes for node {:?} : {:?}", node, inactive_nodes);
            for val in inactive_nodes {
                old_inactive.insert(val);
            }
        }
        for node in old_inactive {
            //println!("Inactive Node: {:?}", node);
            let neighbours = self.get_neighbours(&node);
            match self.check_active(&neighbours) {
                true => {
                    //println!("Activated");
                    new_active.insert(node);
                }
                false => {
                    //println!("Still inactive");
                }
            };
        }
        //println!("{:?}", new_active);
        self.active = new_active;
    }

    fn get_neighbours(&self, node: &hypercube) -> Vec<hypercube> {
        let (x, y, z, w) = node.clone();
        let mut neighbours = Vec::new();
        for x in -1..2 {
            for y in -1..2 {
                for z in -1..2 {
                    for w in -1..2 {
                        let candidate = (node.0 + x, node.1 + y, node.2 + z, node.3 + w);
                        if candidate != *node {
                            neighbours.push(candidate);
                        }
                    }
                }
            }
        }
        neighbours
    }

    fn check_inactive(&self, neighbours: &Vec<hypercube>) -> bool {
        match neighbours
            .iter()
            .filter(|val| self.active.contains(&val))
            .count()
        {
            2 | 3 => return false,
            _ => return true,
        };
    }

    fn check_active(&self, neighbours: &Vec<hypercube>) -> bool {
        match neighbours
            .iter()
            .filter(|val| self.active.contains(&val))
            .count()
        {
            3 => return true,
            _ => return false,
        };
    }
}

fn main() {
    let input = read_input("input").unwrap();
    let mut input = get_starting_configuration(&input);
    //println!("{:?}", input);
    let mut conway_cubes = ConwayCubes {
        active: input.clone(),
    };
    println!("{}", conway_cubes.active.len());
    for _ in 0..6 {
        conway_cubes.run_cycle();
    }
    println!("{:?}", conway_cubes.active.len());
    let mut hyper_input = HashSet::new();
    for val in input.into_iter() {
        hyper_input.insert((val.0, val.1, val.2, 0));
    }
    let mut conway_hypercubes = ConwayHypercubes {
        active: hyper_input,
    };
    for _ in 0..6 {
        conway_hypercubes.run_cycle();
    }
    println!("{:?}", conway_hypercubes.active.len());
}
