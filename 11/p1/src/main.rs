use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("No such file");

    let image = input.lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c == '#'
                })
            .collect::<Vec<bool>>()
        })
    .collect::<Vec<Vec<bool>>>();

    let height = image.len();
    let width = image[0].len();

    let mut empty_cols: Vec<usize> = vec![];
    let mut empty_rows: Vec<usize> = vec![];

    for (i, line) in image.iter().enumerate() {
        if line.iter().all(|c| !*c) {
            empty_rows.push(i);
        }
    }

    'col_scan: for x in 0..width {
        for y in 0..height {
            if image[y][x] {
                continue 'col_scan;
            }
        }
        empty_cols.push(x);
    }

    //println!("{:?}", empty_rows);
    //println!("{:?}", empty_cols);
    //println!("{}", input);

    let mut galaxies: Vec<(usize, usize)> = vec![];
    for y in 0..height {
        for x in 0..width {
            if image[y][x] {
                galaxies.push((y, x));
            }
        }
    }
    
    // change variable space_size to 2 to solve for part 1
    let space_size = 1000000;
    let mut result = 0;
    for galaxy_start in &galaxies {
        for galaxy_end in &galaxies {
            if galaxy_end != galaxy_start {
                //println!("{:?} {:?}", galaxy_start, galaxy_end);

                let num_of_empty_cols = empty_cols.iter()
                    .filter(|col| {
                        galaxy_start.1.min(galaxy_end.1) < **col && **col < galaxy_start.1.max(galaxy_end.1)
                    })
                .count();
                let distance_x = galaxy_start.1.abs_diff(galaxy_end.1) + num_of_empty_cols * space_size - num_of_empty_cols;

                let num_of_empty_rows = empty_rows.iter()
                    .filter(|col| {
                        galaxy_start.0.min(galaxy_end.0) < **col && **col < galaxy_start.0.max(galaxy_end.0)
                    })
                .count();
                let distance_y = galaxy_start.0.abs_diff(galaxy_end.0) + num_of_empty_rows * (space_size - 1);

                result += distance_x + distance_y;
            }
        }
    }

    println!("{}", result / 2);
}
