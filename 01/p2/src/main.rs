use std::{fs, str::from_utf8};

const NUMBERS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn find_first_occurence(line_original: &str, multiplier: u32, rev: bool) -> u32 {
    let line = if rev { 
        line_original.chars()
            .rev()
            .collect::<Vec<char>>() 
    } else { 
        line_original.chars()
            .collect()
    };

    let numbers = if rev {
        NUMBERS.map(
            |number| number.chars()
            .rev()
            .collect::<Vec<char>>()
            )
    } else { 
        NUMBERS.map(
            |number| number.chars()
            .collect::<Vec<char>>()) 
    };

    for (i, c) in line.iter().enumerate() {
        if c.is_digit(10) {
            return c.to_digit(10).unwrap() * multiplier;
        }

        for (num, number) in numbers.iter().enumerate() {
            if line[i..(i+number.len()).min(line.len())].to_vec() == *number {
                return (num as u32 + 1) * multiplier;
            }
        }
    }

    return 0;
}

fn main() {
    let input = fs::read_to_string("input").expect("File should exist");
    println!("{:?}", input); 
    
    let mut result = 0;

    for line in input.lines() {
        result += find_first_occurence(&line, 10, false);
        result += find_first_occurence(&line, 1, true);
    }

    println!("{}", result);
}
