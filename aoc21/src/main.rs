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

// Return
// Map of Allergen : Set of possible ingredients,
// Ingredients
fn get_parsed_input_part1(
    input: &Vec<String>,
) -> (HashMap<String, HashSet<String>>, HashMap<String, u64>) {
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();
    //let mut all_ingredients = HashSet::new();
    let mut ingredients_count: HashMap<String, u64> = HashMap::new();
    for line in input.iter() {
        let tmp: Vec<&str> = line.split("(contains ").collect();
        let ing: Vec<&str> = tmp[0].trim().split(" ").collect();
        let ingredients: HashSet<String> = ing.iter().map(|v| v.to_string()).collect();
        for v in ingredients.iter() {
            match ingredients_count.contains_key(v) {
                false => {
                    ingredients_count.insert(v.to_string(), 1);
                }
                true => {
                    let c = ingredients_count.get_mut(v).unwrap();
                    *c += 1;
                }
            }
        }
        /*
        all_ingredients = all_ingredients
            .union(&ingredients)
            .map(|v| v.to_string())
            .collect();
        */
        // Remove trailing ')'
        let aller: String = tmp[1][..(tmp[1].len() - 1)].to_string();
        let allergens: Vec<&str> = aller.split(", ").collect();
        for allergen in allergens {
            let ingredients: HashSet<String> = ing.iter().map(|v| v.to_string()).collect();
            match map.contains_key(allergen) {
                true => {
                    let val = map.get_mut(allergen).unwrap();
                    *val = ingredients
                        .clone()
                        .intersection(val)
                        .map(|v| v.to_owned())
                        .collect();
                }
                false => {
                    map.insert(allergen.to_string(), ingredients);
                }
            }
        }
    }
    (map, ingredients_count)
}

fn check_values_len(map: &HashMap<String, HashSet<String>>) -> bool {
    map.values().filter(|v| v.len() != 1).count() == 0
}

fn part1(map: &mut HashMap<String, HashSet<String>>) {
    let all_keys: Vec<String> = map.keys().map(|v| v.to_owned()).collect();
    while map.values().filter(|v| v.len() != 1).count() != 0 {
        for k in all_keys.clone() {
            if map.values().filter(|v| v.len() != 1).count() == 0 {
                break;
            }
            let val = map.get_key_value(&k);
            let (k, v) = (val.unwrap().0.clone(), val.unwrap().1.clone());
            if v.len() == 1 {
                // get only element in set
                let el: String =
                    v.iter().map(|v| v.to_owned()).collect::<Vec<String>>()[0].to_owned();
                for (k1, v1) in map.iter_mut() {
                    if *k1 != k && v1.contains(&el) {
                        (*v1).remove(&el);
                    }
                }
            }
        }
    }
}

fn main() {
    let input = read_input("input").unwrap();
    let (mut map, ingredients_count) = get_parsed_input_part1(&input);
    part1(&mut map);
    let ingredients_with_allergens: HashSet<String> = map
        .values()
        .map(|v| v.iter().map(|s| s.to_owned()).collect::<Vec<String>>()[0].to_owned())
        .collect();
    let all_ingredients: HashSet<String> = ingredients_count.keys().map(|v| v.to_owned()).collect();
    let difference = all_ingredients.difference(&ingredients_with_allergens);
    let count: u64 = difference.map(|v| ingredients_count.get(v).unwrap()).sum();
    println!("Part 1: {:?}", count);
    let mut ingredients_with_allergens_vec: Vec<(String, String)> = map
        .iter()
        .map(|(k, v)| {
            (
                k.to_owned(),
                v.into_iter().map(|v| v.to_owned()).collect::<Vec<String>>()[0].to_owned(),
            )
        })
        .collect();
    ingredients_with_allergens_vec.sort_by_key(|val| val.0.to_string());
    let canonical_allergens: Vec<String> = ingredients_with_allergens_vec
        .iter()
        .map(|v| v.1.to_string())
        .collect();
    println!("Part 2: {:?}", canonical_allergens.join(","));
}
