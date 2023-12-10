use std::{fs, collections::HashMap, usize};

fn is_prime(x: usize) -> bool {
    for i in 2..x/2 {
        if x % i == 0 {
            return false;
        }
    }

    return true;
}

fn main() {
    let input = fs::read_to_string("input").expect("file should exists");

    let lr_inst: Vec<usize> = input.lines().nth(0).expect("file should have 0 line")
        .chars()
        .map(|c| if c == 'R' { 1 } else { 0 })
        .collect();

    let mut nodes: HashMap<String, Vec<String>> = HashMap::new();
    input.lines()
        .filter(|line| {
            line.contains("=")
        })
    .for_each(|line| {
        let splitted_line: Vec<&str> = line
            .split(" = ")
            .collect();

        let branches: Vec<String> = splitted_line[1]
            .replace("(", "")
            .replace(")", "")
            .split(", ")
            .map(|node| {
                node.to_string()
            })
            .collect();

        nodes.insert(splitted_line[0].to_string(), branches);
    });

    //println!("{:?}", nodes);

    let current_nodes: Vec<_> = nodes.keys()
        .map(|k| k.to_string())
        .filter(|k| {
            k.chars().last().unwrap() == 'A'
        })
    .collect();
    
    let z_positions_all: Vec<Vec<(usize, usize)>> = current_nodes.iter()
        .filter(|node| {
            node.chars().last().unwrap() == 'A'
        })
        .map(|mut node| {
            let mut z_positions: Vec<(usize, usize)> = vec![];

            let mut seen_z = 0;
            let mut step = 0;
            while seen_z < 10 {
                //println!("on {}", node);
                let dir_idx = step % lr_inst.len();
                let direction = lr_inst[dir_idx];

                node = &nodes
                    .get(node).expect("Nonexistent key")[direction];

                if node.chars().last().unwrap() == 'Z' {
                    z_positions.push((step, dir_idx));

                    seen_z += 1;
                }

                //println!("step {} {:?}", step, node);

                step += 1;
            }

            if z_positions.windows(3).all(|window| {
                window[2].0 - window[1].0 == window[1].0 - window[0].0
            }) {
                println!("nie dziala");
            };

            z_positions
        }
    ).collect();

    let cycles: Vec<(usize, usize)> = z_positions_all.iter().map(|node| {
        (node[1].0 - node[0].0, node[0].0)
    })
    .collect();

    println!("{:?}", cycles);

    //println!("{:?}", z_positions_all);
}
