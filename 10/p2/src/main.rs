use std::{fs, collections::HashMap, io, iter::Sum};


const DIRECTIONS: [[i32; 2]; 4] = [
    [0, -1],
    [1, 0],
    [0, 1],
    [-1, 0]
];

fn get_pipe(x: usize, y: usize, map: &Vec<&str>) -> char {
    map[y].chars().nth(x).unwrap()
}

fn print_map(map: &Vec<&str>, current_pos: &Vec<usize>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if current_pos[0] == x && current_pos[1] == y {
                print!("*");
            } else {
                print!("{}", get_pipe(x, y, map));
            }
        }
        println!();
    }
}

fn print_left_rights(map: &Vec<Vec<i32>>, distance_map: &Vec<Vec<i32>>, output: &mut Vec<Vec<i32>>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if distance_map[y][x] < 10000 {
                output[y][x] = 0;
                continue;
            }
            if map[y][x] == -1 {
                output[y][x] = -1;
            } else if map[y][x] == 1 {
                output[y][x] = 1;
            } else {
                output[y][x] = 0;
            }
        }
    }
}

fn fill_gaps(l_n_r: &mut Vec<Vec<i32>>, to_fill: i32, distance_map: &Vec<Vec<i32>>){
    let mut filled = false;
    while !filled {
        filled = true;

        for y in 0..l_n_r.len() {
            for x in 0..l_n_r[0].len() {
                if l_n_r[y][x] == to_fill {
                    for dir in DIRECTIONS.iter().enumerate() {
                        let ya = y as i32 + dir.1[1];
                        let xa = x as i32 + dir.1[0];

                        if xa < 0 || xa >= l_n_r[0].len() as i32 || ya < 0 || ya >= l_n_r.len() as i32 {
                            continue;
                        }

                        let yu = ya as usize;
                        let xu = xa as usize;

                        if distance_map[yu][xu] >= 10000 && l_n_r[yu][xu] != to_fill {
                            l_n_r[yu][xu] = to_fill;

                            filled = false;
                        }
                    }
                }
            }
        }
    }
}

fn modulo(x: i32, module: i32) -> i32 {
    let mut result = x;
    while result < 0 {
        result += module;
    }
    return result % module;
}

fn compare_directions(last_dir: usize, curr_dir: usize) -> i32 {
    let diff = modulo(last_dir as i32 - curr_dir as i32, 4);

    println!("diff {}", diff);

    if diff == 1 {
        return -1;
    }

    if diff == 3 {
        return 1;
    }

    if diff == 0 {
        return 0;
    }

    panic!("Wrong direction!");
}

fn main() {
    let input = fs::read_to_string("test2").unwrap();

    let PIPES: HashMap<char, [bool; 4]> = HashMap::from([
        ('.', [false, false, false, false]),
        ('|', [true, false, true, false]),
        ('-', [false, true, false, true]),
        ('L', [false, false, true, true]),
        ('J', [false, true, true, false]),
        ('7', [true, true, false, false]),
        ('F', [true, false, false, true]),
        ('S', [true, true, true, true])
    ]);
    
    let width = input.lines().nth(0).unwrap().len();
    let height = input.lines().count();

    let input_lines = &input.lines().collect();

    let start_y = input.lines().position(|line| {
        line.contains("S")
    }).expect("No line has starting position");
    let start_x = input.lines().nth(start_y).unwrap()
        .chars()
        .position(|c| c == 'S')
        .unwrap();

    
    let max_steps = 10000;
    let mut distance_map: Vec<Vec<i32>> = vec![vec![max_steps + 1; width]; height];
    distance_map[start_y][start_x] = 0;
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
                            let curr_pipe = get_pipe(x, y, input_lines);
                            let pipe = get_pipe(xu, yu, input_lines);
                            if PIPES.get(&pipe).expect("Nonexistent pipe")[dir.0] && PIPES.get(&curr_pipe).expect("Nonexistent pipe")[(dir.0 + 2) % 4]  {
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

    let mut pipe_counter: HashMap<char, i32> = HashMap::from([
        ('.', 0),
        ('|', 0),
        ('-', 0),
        ('L', 0),
        ('J', 0),
        ('7', 0),
        ('F', 0),
        ('S', 0)
    ]);

    let mut current_pos = vec![start_x, start_y];
    println!("pos: {:?}", current_pos);

    let mut reached_max = false;
    let mut turning_number = 0;

    step = 0;

    let mut last_dir = 0;
    let mut curr_dir = 0;
    
    let mut lefts_n_rights: Vec<Vec<i32>> = vec![vec![0; width]; height];
    while !(current_pos[0] == start_x && current_pos[1] == start_y && step != 0) {
        print_map(input_lines, &current_pos);
        *pipe_counter.get_mut(&get_pipe(current_pos[0], current_pos[1], input_lines)).unwrap() += 1;
        for dir in DIRECTIONS.iter().enumerate() {
            let ya = current_pos[1] as i32 + dir.1[1];
            let xa = current_pos[0] as i32 + dir.1[0];

            if xa < 0 || xa >= width as i32 || ya < 0 || ya >= height as i32 || distance_map[ya as usize][xa as usize] !=  step + if reached_max { -1 } else { 1 } {
                continue;
            }
            
            current_pos[0] = xa as usize;
            current_pos[1] = ya as usize;
            distance_map[current_pos[1]][current_pos[0]] = max_steps - 10;
            println!("pos: {:?}", current_pos);
            if step >= 1 {
                println!("\ndziała\n");
                last_dir = curr_dir;
            }
            println!("{}", dir.0);
            curr_dir = dir.0;
            break;
        }

        let direction_right = DIRECTIONS.get(modulo((curr_dir + 1).try_into().unwrap(), 4) as usize).unwrap();
        let direction_left = DIRECTIONS.get(modulo((curr_dir as i32 - 1), 4) as usize).unwrap();
        println!("left {:?} right {:?}", direction_left, direction_right);

        let y0 = direction_right[1] + current_pos[1] as i32;
        let x0 = direction_right[0] + current_pos[0] as i32;
        let y1 = direction_left[1] + current_pos[1] as i32;
        let x1 = direction_left[0] + current_pos[0] as i32;

        if y0 >= 0 && y0 < height.try_into().unwrap() && x0 >=0 && x0 < width.try_into().unwrap() {
            lefts_n_rights[
                y0 as usize
            ][
            x0 as usize
            ] = 1;
        }
        if y1 >= 0 && y1 < height.try_into().unwrap() && x1 >= 0 && x1 < width.try_into().unwrap() {
            lefts_n_rights[
                y1 as usize
            ][
                x1 as usize
            ] = -1;
        }
        if step >= 1 {
            turning_number += compare_directions(last_dir, curr_dir);
        }
        
        step += if reached_max { -1 } else { 1 };
        if step == max {
            reached_max = true;
        }
    }
    
    let mut lefts_n_rights_printed: Vec<Vec<i32>> = vec![vec![0; width]; height];
    print_left_rights(&lefts_n_rights, &distance_map, &mut lefts_n_rights_printed);
    fill_gaps(&mut lefts_n_rights_printed, turning_number.signum(), &distance_map);

    let mut sum = 0;
    for line in &lefts_n_rights_printed {
        for num in line {
            if *num == turning_number.signum() {
                sum += 1;
            }
        }
    }

    println!("{}", sum);

    println!("{}", input);
    lefts_n_rights_printed.iter().for_each(|line| {
        let result = line
        .into_iter()
        .map(|i| if *i == turning_number.signum() {
            "#"
        } else {
            "."
        })
        .collect::<String>();
        println!("{}", result);
        });

}
