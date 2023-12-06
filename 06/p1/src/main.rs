use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("input").expect("File should exist");

    let re_lists = Regex::new(r"([0-9]{1,}(\s|$){1,}){1,}").unwrap();

    let records: Vec<Vec<i32>> = re_lists
        .find_iter(&input)
        .map(|cap| {
            cap
                .as_str()
                .split_whitespace()
                .filter(|number| number.len() > 0)
                .map(|number| i32::from_str_radix(number, 10).expect("Number should be base 10 integer"))
                .collect()
        })
    .collect();

    let mut result = 1;

    for i in 0..records[0].len() {
        // println!("i {}", i);
        let mut num_of_ways_to_beat = 0;
        for btn_hold_time in 1..records[0][i] {
            let my_dist = (records[0][i] - btn_hold_time) * btn_hold_time;
            // print!("{} ?> {} ", my_dist, records[1][i]);
            if my_dist > records[1][i] {
                num_of_ways_to_beat += 1;
            }
        }
        // println!("\n{}", num_of_ways_to_beat);
        result *= num_of_ways_to_beat;
    }

    println!("{}", result);
}
