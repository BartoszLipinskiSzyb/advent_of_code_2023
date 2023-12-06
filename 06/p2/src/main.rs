use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("input").expect("File should exist");

    let re_lists = Regex::new(r"([0-9]{1,}(\s|$){1,}){1,}").unwrap();

    let record: Vec<i128> = re_lists
        .find_iter(&input)
        .map(|cap| {
            i128::from_str_radix({
                &cap
                    .as_str()
                    .replace(" ", "")
                    .replace("\n", "")
                },
                10).expect("Should be base 10 integer")
        })
    .collect();

    let mut result = 0;

    for btn_held_for in 1..record[0] {
        if (record[0] - btn_held_for) * btn_held_for > record[1] {
            result += 1;
        }
    }

    println!("{}", result);
}
