use std::{fs, collections::HashMap};

const CARD_DEFINITIONS: [(char, i32); 13] = [
        ('2', 0),
        ('3', 0),
        ('4', 0),
        ('5', 0),
        ('6', 0),
        ('7', 0),
        ('8', 0),
        ('9', 0),
        ('T', 0),
        ('J', 0),
        ('Q', 0),
        ('K', 0),
        ('A', 0)
    ];

fn get_hand_type(hand: &str) -> i32 {
    let mut each_card_amount: HashMap<char, i32> = HashMap::from(CARD_DEFINITIONS);

    hand.chars().for_each(|c| {
        *each_card_amount.get_mut(&c).expect("Undefined card") += 1;
    });

    // //println!("{:?}", each_card_amount);

    if each_card_amount.values().any(|v| *v == 5) {
        return 6;
    }

    if each_card_amount.values().any(|v| *v == 4) {
        return 5;
    }

    if each_card_amount.values().any(|v| *v == 3) {
        if each_card_amount.values().any(|v| *v == 2) {
            return 4;
        }
        return 3;
    }

    // return number of pairs
    return each_card_amount.values().filter(|v| **v == 2).count() as i32;
}

fn is_first_hand_smaller(f: &str, s: &str) -> bool {
    let f_type = get_hand_type(f);
    let s_type = get_hand_type(s);

    if f_type == s_type {
        for i in 0..f.len() {
            let f_pos = CARD_DEFINITIONS.iter().position(|c| c.0 == f.chars().nth(i).unwrap()).unwrap();
            let s_pos = CARD_DEFINITIONS.iter().position(|c| c.0 == s.chars().nth(i).unwrap()).unwrap();
            if f_pos < s_pos {
                return true;
            } else if f_pos > s_pos {
                return false;
            }
        }
        return false;
    } else {
        return f_type < s_type;
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("File should exist");
        
    let mut hands_n_bids = input.lines().map(|line| line.split_whitespace().collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();

    let mut result = 0;
    for rank in 1..=hands_n_bids.len() {
        // //println!("rank: {}", rank);
        let mut smallest_hand_idx = 0;
        for (hand_idx, hand_n_bid) in hands_n_bids.iter().enumerate() {
            //print!("{} <? {} == ", hand_n_bid[0], hands_n_bids[smallest_hand_idx][0]);
            if is_first_hand_smaller(hand_n_bid[0], hands_n_bids[smallest_hand_idx][0]) {
                smallest_hand_idx = hand_idx;
                //println!("true");
            }
        }
        result += i32::from_str_radix(hands_n_bids[smallest_hand_idx][1], 10).expect("Should be base 10 i32") * rank as i32;
        hands_n_bids.remove(smallest_hand_idx);
    }

    println!("{}", result);
}
