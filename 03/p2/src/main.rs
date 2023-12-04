use std::{fs, num};

fn print_checked(checked_positions: &Vec<Vec<bool>>) {
    println!();
    let display_checked = checked_positions
        .iter()
        .map(|line| line
            .iter()
            .map(|val| if *val { '#' } else { '.' })
            .collect::<Vec<_>>()
            )
        .collect::<Vec<_>>();

    for line in display_checked {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn main() {
    let input = fs::read_to_string("input").expect("Should be able to read file");
    println!("{}", input);
    
    let mut checked_positions: Vec<Vec<bool>> = vec![vec![false; input.lines().nth(0).unwrap().chars().count()]; input.lines().count()];
    
    let mut result = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, letter) in line.chars().enumerate() {
            let mut found_numbers: Vec<i32> = vec![];
            if !letter.is_digit(10) && letter != '.' {
                let y_i32: i32 = y as i32;
                let x_i32: i32 = x as i32;
                for ya in y_i32-1..=y_i32+1 {
                    for xa in x_i32-1..=x_i32+1 {
                        println!("Position {}, {} is {}", xa, ya, checked_positions[ya as usize][xa as usize]);
                        if !checked_positions[ya as usize][xa as usize] && ya >= 0 && ya < input.lines().count().try_into().unwrap() && xa >= 0 && xa <= input.lines().nth(0).unwrap().chars().count().try_into().unwrap() {
                            checked_positions[ya as usize][xa as usize] = true;
                            print!("{}, {} : ", xa, ya);
                            
                            let potential_digit = input.lines().nth(ya.try_into().unwrap()).unwrap().chars().nth(xa.try_into().unwrap()).unwrap(); 
                            if potential_digit.is_digit(10) {
                                let mut start = xa;
                                let mut end = xa;

                                loop {
                                    if end + 1 >= input.lines().nth(0).unwrap().chars().count().try_into().unwrap() {
                                        break;
                                    }
                                    if !input.lines().nth(ya.try_into().unwrap()).unwrap().chars().nth((end + 1).try_into().unwrap()).unwrap().is_digit(10) {
                                        break;
                                    }
                                    end += 1;
                                    checked_positions[ya as usize][end as usize] = true;
                                }


                                loop {
                                    if start - 1 < 0 {
                                        break;
                                    }
                                    if !input.lines().nth(ya.try_into().unwrap()).unwrap().chars().nth((start - 1).try_into().unwrap()).unwrap().is_digit(10) {
                                        break;
                                    }
                                    start -= 1;
                                    checked_positions[ya as usize][start as usize] = true;
                                }
                                print!("Znaleziono numer: ");
                                let mut number = String::new();
                                for letter in start..=end {
                                    number.push(input.lines().nth(ya.try_into().unwrap()).unwrap().chars().nth(letter.try_into().unwrap()).unwrap());
                                }
                                println!("{}", number);
                                found_numbers.push(i32::from_str_radix(&number, 10).unwrap());
                                println!();
                            }
                        }
                    }
                }
                println!("{}", letter);
            }

            if found_numbers.len() == 2 && letter == '*' {
                result += found_numbers.iter().product::<i32>();
            }
        }
    }

    println!("{}", result);
}
