use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("File should exist");

    let result: u32 = input
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| {
            char::to_digit(
                line.chars().nth(
                    line.chars().position(
                        |char| char.is_digit(10)
                    ).expect("Line should containt at least one digit")
                ).expect("Found index should exist in vector"), 10
            ).expect("Should be able to convert char to integer") * 10
            + char::to_digit(
                line.chars().rev().nth(
                    line.chars().rev().position(
                        |char| char.is_digit(10)
                    ).expect("Line should containt at least one digit")
                ).expect("Found index should exist in vector"), 10
            ).expect("Should be able to convert char to integer")
    }).sum();

    println!("{}", result);
}
