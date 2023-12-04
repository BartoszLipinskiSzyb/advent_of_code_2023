use std::fs;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("input").expect("Should be able to read file");

    let my_numbers_reg = Regex::new(r"(\s{1,}[0-9]{1,}){1,}$").expect("Should be correct regex");
    let winning_numbers_reg = Regex::new(r":(\s{1,}[0-9]{1,})*").expect("Should be correct regex");
    let mut amount_of_cards: Vec<i32> = vec![1; input.lines().count()];

    let input_arr: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            let capture = my_numbers_reg
                .captures(line).expect("Should find something")
                .get(0).unwrap()
                .as_str();

            Regex::new(r"^ ").unwrap()
                .replace_all(capture, "")
                .replace("  ", " ")
                .split_whitespace()
                .map(|num| {
                    i32::from_str_radix(num, 10).unwrap()
                })
                .collect::<Vec<_>>()
        })
    .collect();

    let winning_arr: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            let capture = winning_numbers_reg
                .captures(line).expect("Should find something")
                .get(0).unwrap()
                .as_str();

            capture
                .replace(": ", "")
                .replace("  ", " ")
                .split_whitespace()
                .map(|num| {
                    i32::from_str_radix(num, 10).unwrap()
                })
                .collect::<Vec<_>>()
        })
    .collect();

    for line_idx in 0..winning_arr.len() {
        let mut matching_amount = 0;
        for winning_num in &winning_arr[line_idx] {
            if input_arr[line_idx].contains(&winning_num) {
                matching_amount += 1;
            }
        }
    
        for add_line_idx in line_idx+1..line_idx+1+matching_amount {
            amount_of_cards[add_line_idx] += amount_of_cards[line_idx];
        }
    }

    println!("{}", amount_of_cards.iter().sum::<i32>());
}
