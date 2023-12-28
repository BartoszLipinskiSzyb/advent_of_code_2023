use std::{fs, usize};

fn hash(word: &str) -> i32 {
    let mut result = 0;
    word.chars().for_each(|c| {
        let ascii = c as i32;
        result += ascii;
        result *= 17;
        result %= 256;
    });
    return result;
}

fn main() {
    let input = fs::read_to_string("input").unwrap().replace("\n", "");

    let mut boxes: Vec<Vec<(&str, i32)>> = vec![vec![]; 256];

    input.split(",")
        .for_each(|line| {
            let label;
            let op;
            let focal_length;

            if line.contains("=") {
                let splitted: Vec<&str> = line.split("=").collect();
                label = splitted[0];
                op = '=';
                focal_length = i32::from_str_radix(splitted[1], 10).expect("Should be 0-9");
            } else {
                label = line.split("-").collect::<Vec<&str>>()[0];
                op = '-';
                focal_length = 0;
            }

            let box_id = hash(label);            

            if op == '-' {
                for (i, lens) in boxes[box_id as usize].iter().enumerate() {
                    if lens.0 == label {
                        boxes[box_id as usize].remove(i);
                        break;
                    }
                }
            } else {
                let mut lens_found = false;
                for (i, lens) in boxes[box_id as usize].iter().enumerate() {
                    if lens.0 == label {
                        boxes[box_id as usize][i].1 = focal_length;
                        lens_found = true;
                        break;
                    }
                }
                if !lens_found {
                    boxes[box_id as usize].push((label, focal_length));
                }
            }
        });

    let result = boxes.iter().enumerate().map(|(box_id, curr_box)| {
        curr_box.iter().enumerate().map(|(lens_id, lens)| {
            (box_id as i32 + 1) * (lens_id as i32 + 1) * lens.1
        }).sum::<i32>()
    }).sum::<i32>();

    println!("{}", result);
}
