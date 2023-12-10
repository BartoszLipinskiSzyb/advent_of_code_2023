use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("File should exist");
    
    let mut result = 0;
    input.lines().for_each(|history| {
        let mut values: Vec<Vec<i32>> = vec![];

        values.push(history
            .split_whitespace()
            .map(|num| i32::from_str_radix(num, 10).expect("Num should be i32 base 10"))
            .rev()
            .collect()
        );
        
        let mut current_values_idx: usize = 0;
        while !values[current_values_idx].iter()
            .all(|val| *val == 0)
        {
            values.push(values[current_values_idx].windows(2)
                .map(|pair| {
                    pair[1] - pair[0]
                })
                .collect()
            );
            current_values_idx += 1;
        }

        while current_values_idx >= 1 {
            let result_val = values[current_values_idx - 1].last().unwrap() + values[current_values_idx].last().unwrap();
            values[current_values_idx - 1].push(result_val);
            current_values_idx -= 1;
        }

        result += values[0].last().unwrap();

        println!("{:?}", values);
    });

    println!("{}", result);
}
