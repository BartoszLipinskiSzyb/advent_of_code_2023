use std::{fs, usize, io};

const DIRS: [[i32; 2]; 4] = [
    [1, 0],
    [0, -1],
    [-1, 0],
    [0, 1]
];

fn main() {
    let input = fs::read_to_string("test").unwrap();
    println!("{}", input);

    let mut plate: Vec<Vec<char>> = input.lines()
        .map(|line| {
            line.chars()
                .collect()
        })
    .collect();

    println!("{:?}", plate);

    for step in 0..1000000000 {
        //println!("{}", step);
        for (y, line) in plate.clone().iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                let xa = x as i32 + DIRS[step % 4][1];
                let ya = y as i32 + DIRS[step % 4][0];
                if *c == 'O' && ya > 0 && ya < plate.len() as i32 - 1 && xa > 0 && xa < plate[0].len() as i32 - 1 {
                    if plate[ya as usize][xa as usize] == '.' {
                        plate[y][x] = '.';
                        plate[ya as usize][xa as usize] = 'O';
                    }
                }
            }
        }
        for line in &plate {
            for c in line {
                print!("{}", c);
            }
            println!();
        }
        let mut a = String::new();
        io::stdin().read_line(&mut a).unwrap();
    }


    let mut result = 0;
    for (y, line) in plate.clone().iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'O' {
                result += plate.len() - y;
            }
        }
    }

    println!("{}", result);
}
