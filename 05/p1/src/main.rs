use std::fs;
use regex::Regex;

fn map_seed(seed: &mut i128, function: &Vec<Vec<i128>>) {
    for case in function {
        if *seed >= case[1] && *seed < case[1] + case[2] {
            *seed += case[0] - case[1];
            break;
        }
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Should be able to read file");

    // parsing input
    let mut seed_list: Vec<i128> = input
        .lines()
        .nth(0).expect("File should have line 0")
        .replace("seeds: ", "")
        .split_whitespace()
        .map(|num| i128::from_str_radix(num, 10).expect("Seed not a base 10 number"))
        .collect();

    println!("Seeds: {:?}\n", seed_list);

    let maps_regex = Regex::new(r"map:(\n.{1,})*").unwrap();

    let functions = maps_regex.captures_iter(&input).map(|maps| {
        maps
            .get(0).unwrap()
            .as_str()
            .replace("map:\n", "")
            .lines()
            .map(|map| 
                map.split_whitespace()
                .map(|num| 
                    i128::from_str_radix(num, 10).unwrap()
                )
                .collect::<Vec<_>>())
            .collect::<Vec<_>>()
    });

    // applying functions to get location
    for function in functions {
        for seed in &mut seed_list {
            //print!("Mapping seed {} to ", seed);
            map_seed(seed, &function);
            //println!("{}", seed);
        }
    }

    println!("Seeds: {:?}", seed_list);

    println!("{}", seed_list.iter().min().expect("seed_list should have some values"));
}
