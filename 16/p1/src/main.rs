use std::{fs, usize, collections::HashSet};

const DIRS: [[i32; 2]; 4] = [
    [-1, 0],
    [0, 1],
    [1, 0],
    [0, -1]
];

fn modulo(x: i32, m: i32) -> i32 {
    let mut result = x;
    while result < 0 {
        result += m;
    }
    return result % m;
}

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let field: Vec<Vec<char>> = input.lines().map(|line| {
        line.chars().collect()
    })
    .collect();

    let mut max = 0;

    let mut starts: Vec<(usize, usize, usize)> = vec![];

    for x in 0..field[0].len() {
        starts.push((0, x, 2));
        starts.push((field.len() - 1, x, 0));
    }
    for y in 0..field.len() {
        starts.push((y, 0, 1));
        starts.push((y, field[0].len() - 1, 3));
    }

    for start in starts {
        println!("{:?}", start);
        let mut beams: Vec<(usize, usize, usize)> = vec![start];
        let mut energized: Vec<Vec<bool>> = vec![vec![false; field[0].len()]; field.len()];
        let mut result = 0;
        let mut tiles_unchanged = 0;

        while tiles_unchanged < 10 {
            let mut tiles_changed = false;

            for beam in &beams {
                if !energized[beam.0][beam.1] {
                    tiles_changed = true;
                    energized[beam.0][beam.1] = true;
                    result += 1;
                }
            }

            let mut beams_to_add: Vec<(usize, usize, usize)> = vec![];

            for beam in beams.iter_mut() {
                let curr_tile = field[beam.0][beam.1];

                if (curr_tile == '\\' && beam.2 % 2 == 1) || (curr_tile == '/' && beam.2 % 2 == 0) {
                    beam.2 = modulo(beam.2 as i32 + 1, 4) as usize;
                } else if (curr_tile == '/' && beam.2 % 2 == 1) || (curr_tile == '\\' && beam.2 % 2 == 0) {
                    beam.2 = modulo(beam.2 as i32 - 1, 4) as usize;
                } else if (curr_tile == '-' && beam.2 % 2 == 0) || (curr_tile == '|' && beam.2 % 2 == 1) {
                    beams_to_add.push((beam.0, beam.1, modulo(beam.2 as i32 + 1, 4) as usize));
                    beam.2 = modulo(beam.2 as i32 - 1, 4) as usize;
                }
            }
            beams.append(&mut beams_to_add);

            let mut beams_to_delete: Vec<usize> = vec![];
            for (i, beam) in beams.iter_mut().enumerate() {
                let y_new = beam.0 as i32 + DIRS[beam.2][0];
                let x_new = beam.1 as i32 + DIRS[beam.2][1];

                if y_new >= 0 && y_new < field.len() as i32 && x_new >= 0 && x_new < field[0].len() as i32 {
                    beam.0 = y_new as usize;
                    beam.1 = x_new as usize;
                } else {
                    beams_to_delete.push(i);
                }
            }

            let mut sub = 0;
            for beam in beams_to_delete {
                beams.remove(beam - sub);
                sub += 1;
            }

            let set: HashSet<_> = beams.drain(..).collect();
            beams.extend(set.into_iter());

            for beam in &beams {
                if !energized[beam.0][beam.1] {
                    tiles_changed = true;
                    energized[beam.0][beam.1] = true;
                    result += 1;
                }
            }

            //println!("{:?}", beams);

            //for line in &energized {
            //    println!("{:?}", line.iter().map(|v| if *v {'#'} else {'.'}).collect::<Vec<_>>());
            //}

            if result > max {
                max = result;
            }

            if !tiles_changed {
                tiles_unchanged += 1;
            } else {
                tiles_unchanged = 0;
            }
        }
    }

    println!("{}", max);
}
