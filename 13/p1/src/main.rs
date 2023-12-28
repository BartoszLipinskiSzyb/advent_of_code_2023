use std::fs;

fn compare_cols(arr: &Vec<Vec<bool>>, id1: usize, id2: usize) -> bool {
    for y in 0..arr.len() {
        if arr[y][id1] != arr[y][id2] {
            return false;
        }
    }
    return true;
}

fn compare_rows(arr: &Vec<Vec<bool>>, id1: usize, id2: usize) -> bool { 
    for x in 0..arr[0].len() {
        if arr[id1][x] != arr[id2][x] {
            return false;
        }
    }
    return true;
}


fn main() {
    let input = fs::read_to_string("input").unwrap();

    println!("{}", input);

    let mut fields: Vec<Vec<Vec<bool>>> = input.split("\n\n")
        .map(|field| {
            field.lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c == '#')
                        .collect()
                })
            .collect()
        })
    .collect();

    let result = fields.iter()
        .map(|field| {
                (0..field.len()-1).map(|i| {
                    if compare_rows(field, i, i + 1) {
                        for diff in 1.. {
                            if i as i32 - diff < 0 || i + 1 + diff as usize >= field.len() {
                                println!("poziomo {}", i + 1);
                                return i as i32 + 1;
                            }
                            if !compare_rows(field, i - diff as usize, i + 1 + diff as usize) {
                                return 0;
                            }
                        }
                    }
                    0
                }).sum::<i32>() * 100
            +
                (0..field[0].len()-1).map(|i| {
                    if compare_cols(field, i, i + 1) {
                        for diff in 1.. {
                            if i as i32 - diff < 0 || i + 1 + diff as usize >= field[0].len() {
                                println!("pionowo {}", i + 1);
                                return i as i32 + 1;
                            }
                            if !compare_cols(field, i - diff as usize, i + 1 + diff as usize) {
                                return 0;
                            }
                        }
                    }
                    0
                }).sum::<i32>()
        })
    .sum::<i32>();

    println!("{}", result);
}
