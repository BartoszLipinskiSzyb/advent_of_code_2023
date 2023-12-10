use std::{fs, collections::HashMap};

fn main() {
    let input = fs::read_to_string("input").expect("file should exists");

    let lr_inst = input.lines().nth(0).expect("file should have 0 line");

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

    println!("{:?}", nodes);

    let mut step = 0;
    let mut current_node = "AAA";

    while current_node != "ZZZ" {
        let direction: usize = if lr_inst.chars().nth(step % lr_inst.len()).unwrap() == 'R' { 1 } else { 0 };

        current_node = &nodes.get(current_node).expect("Node doesn't exist")[direction];

        step += 1;
    }

    println!("{}", step);
}
