use std::{fs, collections::HashMap};
use regex::Regex;

const MAX_CUBE_AMOUNTS: [i32; 3] = [12, 13, 14];

fn main() {
    let file = fs::read_to_string("input").expect("Should be able to read file");
    let input: Vec<&str> = file.lines().filter(|line| line.len() > 0).collect();

    let my_regex = Regex::new(r"[0-9]{1,}\s[rgb]").expect("Should be valid regex");
    let mut result = 0;

    let mut colors: HashMap<&str, usize> = HashMap::new();
    colors.insert("r", 0);
    colors.insert("g", 1);
    colors.insert("b", 2);

    for (i, line) in input.iter().enumerate() {
        println!("{:?}", line);
        let mut min_cubes_needed: Vec<i32> = vec![0, 0, 0];
        for cap in my_regex.captures_iter(line) {
            let my_match = cap.get(0).unwrap().as_str();

            let splitted_match: Vec<&str> = my_match.split_whitespace().collect();
            let amount = i32::from_str_radix(splitted_match[0], 10).unwrap();

            let index = *colors.get(splitted_match[1]).expect("No such color");

            min_cubes_needed[index] = amount.max(min_cubes_needed[index]);
        }
        result += min_cubes_needed.iter().product::<i32>();
        println!("{:?}", min_cubes_needed);
    }

    println!("{}", result);
}
