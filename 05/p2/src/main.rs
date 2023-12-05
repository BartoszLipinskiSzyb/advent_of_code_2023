use std::fs;
use regex::Regex;

fn map_seed(seed_input: i128, function: &Vec<Vec<i128>>) -> i128 {
    let mut seed = seed_input;
    for case in function {
        if seed >= case[1] && seed < case[1] + case[2] {
            seed += case[0] - case[1];
            break;
        }
    }

    return seed;
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

    //println!("Seeds: {:?}\n", seed_list);

    let maps_regex = Regex::new(r"map:(\n.{1,})*").unwrap();
    
    let functions: Vec<Vec<Vec<i128>>> = maps_regex.captures_iter(&input).map(|maps| {
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
    }).collect();

    
    let mut smallest: i128 = 999999999999999999999999999999999;

    for seed_command in seed_list.chunks(2) {
        for mut seed in seed_command[0]..seed_command[0]+seed_command[1] {
            for function in &functions {
                seed = map_seed(seed, function);
                //print!("{} ", seed);
            }
            //println!();
            if seed < smallest {
                //println!("Najmniejszy: {}", seed);
                smallest = seed;
            }
        }
    }


    //println!("Seeds: {:?}", seed_list);

    println!("{}", smallest);
}
