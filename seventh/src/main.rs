use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::Itertools;

type Hand = (u16, String, u16);

const ORDERING: [char; 13] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];

fn main() {
    solution();
}

fn solution() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut hands: Vec<Hand> = vec![];

        for result in lines {
            if let Ok(line) = result {
                let (raw_hand, raw_bid) = line.split_once(" ").unwrap();
                let hand_type = hand_type(&hand(raw_hand));
                hands.push((hand_type, raw_hand.chars().map(|c| apply(c)).collect(), raw_bid.parse::<u16>().expect("Failed to parse bid.")));
            }
        }

        hands.sort();

        let mut sum: u64 = 0;
        for (i, hand) in hands.iter().rev().enumerate() {
            let &(_, _, bid) = hand;
            println!("{} {}", i + 1, hand.1);
            sum += (i + 1) as u64 * bid as u64;
        }

        println!("{}", sum);
    }
}

fn hand(raw_hand: &str) -> Vec<u16> {
    ORDERING.iter().map(|suit| raw_hand.chars().filter(|&card| card.eq(suit)).count() as u16).collect_vec()
}

/**
 * 1: Five of a kind
 * 2: Four of a kind
 * 3: Full house (3+2 of a kind)
 * 4: Three of a kind
 * 5: Two pairs
 * 6: One pair
 * 7: High card (distinct)
 */
fn hand_type(hand: &Vec<u16>) -> u16 {
    if hand[3] > 0 { // handle jokers
        let pos = hand.iter().enumerate().map(|(i, &count)| {
            if i == 3 {
                return 0;
            }
            return count;
        }).position_max().unwrap();
        return match hand[pos] + hand[3] {
            2 => 5,
            3 if hand.iter().enumerate().filter(|(i, _)| i.ne(&3) && i.ne(&pos)).map(|(_, count)| count).contains(&2) => 2,
            3 => 3,
            4 => 1,
            5 => 0,
            _ => 0
        };
    }

    if hand.contains(&5) {
        return 0;
    }

    if hand.contains(&4) {
        return 1;
    }

    if hand.contains(&3) && hand.contains(&2) {
        return 2;
    }

    if hand.contains(&3) {
        return 3;
    }

    if hand.iter().filter(|&x| x.eq(&2)).count() == 2 {
        return 4;
    }

    if hand.contains(&2) {
        return 5;
    }

    return 6;
}

fn apply(suit: char) -> char {
    match suit {
        'A' => 'A',
        'K' => 'B',
        'Q' => 'C',
        'J' => 'N',//'D',
        'T' => 'E',
        '9' => 'F',
        '8' => 'G',
        '7' => 'H',
        '6' => 'I',
        '5' => 'J',
        '4' => 'K',
        '3' => 'L',
        _ => 'M'
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}