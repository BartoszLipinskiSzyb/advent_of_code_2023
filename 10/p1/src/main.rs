use std::{fs, collections::HashMap};


const DIRECTIONS: [[i32; 2]; 4] = [
    [0, -1],
    [1, 0],
    [0, 1],
    [-1, 0]
];

fn get_pipe(x: usize, y: usize, map: &Vec<&str>) -> char {
    map[y].chars().nth(x).unwrap()
}

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let PIPES: HashMap<char, [bool; 4]> = HashMap::from([
        ('.', [false, false, false, false]),
        ('|', [true, false, true, false]),
        ('-', [false, true, false, true]),
        ('L', [false, false, true, true]),
        ('J', [false, true, true, false]),
        ('7', [true, true, false, false]),
        ('F', [true, false, false, true]),
    ]);
    
    let width = input.lines().nth(0).unwrap().len();
    let height = input.lines().count();
    let mut distance_map: Vec<Vec<i32>> = vec![vec![10002; width]; height];

    let start_y = input.lines().position(|line| {
        line.contains("S")
    }).expect("No line has starting position");
    let start_x = input.lines().nth(start_y).unwrap()
        .chars()
        .position(|c| c == 'S')
        .unwrap();

    distance_map[start_y][start_x] = 0;
    
    let max_steps = 10000;
    let mut step = 0;
    while step < max_steps {
        for y in 0..height {
            for x in 0..width {
                if distance_map[y][x] == step {
                    for dir in DIRECTIONS.iter().enumerate() {
                        let ya = y as i32 + dir.1[1];
                        let xa = x as i32 + dir.1[0];

                        if xa < 0 || xa >= width as i32 || ya < 0 || ya >= height as i32 {
                            continue;
                        }

                        let yu = ya as usize;
                        let xu = xa as usize;

                        if distance_map[yu][xu] > step {
                            let pipe = get_pipe(xu, yu, &input.lines().collect());
                            if PIPES.get(&pipe).expect("Nonexistent pipe")[dir.0] {
                                distance_map[yu][xu] = step + 1;
                            }
                        }
                    }
                }
            }
        }
        step += 1;
    }

    let mut max = 0;
    for y in 0..height {
        for x in 0..width {
            if distance_map[y][x] > max && distance_map[y][x] < max_steps {
                max = distance_map[y][x];
            }
        }
    }

    println!("{}", max);
}
