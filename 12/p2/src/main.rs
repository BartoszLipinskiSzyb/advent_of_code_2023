use std::{fs, usize, env};
use regex::Regex;

fn count_possible(row: &str, re: &Regex) -> i32 {
    let mut sum = 0;

    if !row.contains("?") {
        //println!("Possible: {}", row);
        return 1;
    }

    let replaced = row.replacen("?", ".", 1);
    if re.is_match(&replaced) {
        sum += count_possible(&replaced, re);
    }

    let replaced2 = row.replacen("?", "#", 1);
    if re.is_match(&replaced2) {
        sum += count_possible(&replaced2, re);
    }

    return sum;
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let input = fs::read_to_string(args[1].to_string()).expect("No file");

    let mut result = 0;
    for line in input.lines() {
        let splitted_line: Vec<&str> = line.split_whitespace().collect();

        let row = splitted_line[0];


        let mut pattern = splitted_line[1]
            .split(",")
            .map(|num| usize::from_str_radix(num, 10)
                .expect("Should be base 10 i32"))
            .collect::<Vec<usize>>();


        let new_row = row.to_owned() + "?" + row + "?" + row + "?" + row + "?" + row;
        println!("{}", new_row);
        let mut new_pattern: Vec<usize> = vec![];
        for i in 0..5 {
            new_pattern.append(&mut pattern.clone());
        }
        println!("{:?}", new_pattern);

        let mut re_str = "^[.?]*".to_string();
        for (i, num) in new_pattern.iter().enumerate() {
            re_str += &("[?#]{".to_string() + &num.to_string().to_owned() + "}[.?]");
            if i < new_pattern.len() - 1 {
                re_str += "{1,}";
            } else {
                re_str += "*$";
            }
        }
        let re = Regex::new(re_str.as_str().try_into().unwrap()).unwrap();

        result += count_possible(&new_row, &re);
        //println!();
    }

    println!("{}", result);
}
