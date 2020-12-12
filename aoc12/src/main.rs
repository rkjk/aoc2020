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

type Direction = (String, i64);

fn parse_input(input: &Vec<String>) -> Vec<Direction> {
    let mut directions = Vec::new();
    for line in input.iter() {
        let mut c = line.trim().chars();
        let d = (
            c.next().unwrap().to_string(),
            c.collect::<String>().parse().unwrap(),
        );
        directions.push(d);
    }
    directions
}

pub const PI: f64 = std::f64::consts::PI;

struct Position {
    cur_pos: (i64, i64),
    cur_angle: i64,
    waypoint: (i64, i64),
}

impl Position {
    fn update_position(&mut self, action: &str, magnitude: i64) {
        let actual_action = match action {
            "F" => match self.cur_angle {
                0 => "E",
                90 => "N",
                180 => "W",
                270 => "S",
                _ => action,
            },
            _ => action,
        };
        let add_pos = match actual_action {
            "N" => (0, magnitude),
            "S" => (0, -1 * magnitude),
            "E" => (magnitude, 0),
            "W" => (-1 * magnitude, 0),
            _ => (0, 0),
        };
        self.cur_pos.0 += add_pos.0;
        self.cur_pos.1 += add_pos.1;

        match actual_action {
            "L" => self.cur_angle = (self.cur_angle + magnitude) % 360,
            "R" => self.cur_angle = (self.cur_angle + 360 - magnitude) % 360,
            _ => (),
        };
    }

    fn update_waypoint(&mut self, action: &str, magnitude: i64) {
        //println!("Current waypoint: {:?}", self.waypoint);
        match action {
            "N" => self.waypoint.1 += magnitude,
            "S" => self.waypoint.1 -= magnitude,
            "E" => self.waypoint.0 += magnitude,
            "W" => self.waypoint.0 -= magnitude,
            "F" => {
                //println!("Old position: {:?}", self.cur_pos);
                self.cur_pos.0 += self.waypoint.0 * magnitude;
                self.cur_pos.1 += self.waypoint.1 * magnitude;
                //println!("New position: {:?}", self.cur_pos);
            }
            "L" => self.change_waypoint(magnitude, true),
            "R" => self.change_waypoint(magnitude, false),
            _ => panic!("Wrong state"),
        };
    }

    fn change_waypoint(&mut self, magnitude: i64, anti_clockwise: bool) {
        let (mag_e, mag_n) = (self.waypoint.0.abs(), self.waypoint.1.abs());
        let mut dir_e = match self.waypoint.0 < 0 {
            true => true,
            false => false,
        };
        let mut dir_n = match self.waypoint.1 < 0 {
            true => true,
            false => false,
        };
        //println!("dir_e: {} dor_n:{}", dir_e, dir_n);
        for _ in 0..(magnitude / 90) {
            let new_dir_e = dir_n ^ anti_clockwise;
            let new_dir_n = dir_e ^ (!anti_clockwise);
            dir_e = new_dir_e;
            dir_n = new_dir_n;
            //println!("dir_e: {} dor_n:{}", dir_e, dir_n);
        }
        let val_e = match dir_e {
            true => -1,
            false => 1,
        };
        let val_n = match dir_n {
            true => -1,
            false => 1,
        };
        //println!("Old direction: {:?}", self.waypoint);
        self.waypoint.0 = match (magnitude / 90) % 2 == 0 {
            true => mag_e * val_e,
            false => mag_n * val_e,
        };
        self.waypoint.1 = match (magnitude / 90) % 2 == 0 {
            true => mag_n * val_n,
            false => mag_e * val_n,
        };
        //println!("New direction: {:?}", self.waypoint);
    }

    fn sin(&self) -> f64 {
        let cur_angle_radians = (self.cur_angle as f64).to_radians();
        cur_angle_radians.sin()
    }

    fn cos(&self) -> f64 {
        let cur_angle_radians = (self.cur_angle as f64).to_radians();
        cur_angle_radians.cos()
    }

    fn manhattan(&self) -> i64 {
        self.cur_pos.0.abs() + self.cur_pos.1.abs()
    }
}

fn main() {
    let input = read_input("input").unwrap();
    let input = parse_input(&input);
    let now = Instant::now();
    let mut position = Position {
        cur_pos: (0, 0),
        cur_angle: 0,
        waypoint: (0, 0),
    };
    for v in input.iter() {
        position.update_position(&v.0, v.1);
    }
    println!("Part 1: {}", position.manhattan());
    println!("Runtime {} us", now.elapsed().as_micros());

    let now = Instant::now();
    position.cur_pos = (0, 0);
    position.cur_angle = 0;
    position.waypoint = (10, 1);
    for v in input.iter() {
        position.update_waypoint(&v.0, v.1);
    }
    println!("Part 2: {}", position.manhattan());
    println!("Runtime {} us", now.elapsed().as_micros());
}
