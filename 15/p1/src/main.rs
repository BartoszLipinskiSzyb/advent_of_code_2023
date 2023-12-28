use std::fs;

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
    let input = fs::read_to_string("input").unwrap();

    let result = input.replace("\n", "").split(",")
        .map(|line| {
            hash(line)
        })
    .sum::<i32>();
    println!("result: {}", result);
}
