use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const INPUT_LENGTH: usize = 198;

#[derive(Debug)]
struct Card {
    count: usize
}

fn main() {
    solution();
}

fn solution() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut cards: Vec<Card> = Vec::with_capacity(INPUT_LENGTH);
        for result in lines {
            if let Ok(line) = result {
                let card = line.split_once(": ").unwrap_or(("", " | ")).1.split_once(" | ").unwrap_or(("", ""));
                let winning_numbers: Vec<u32> = card.0.split_ascii_whitespace().map(|number| number.parse::<u32>().unwrap()).collect();
                let mut numbers: Vec<u32> = card.1.split_ascii_whitespace().map(|number| number.parse::<u32>().unwrap()).collect();
                numbers.sort();
                let winning_count = winning_numbers.iter().filter(|number| numbers.binary_search(number).is_ok()).count();
                cards.push(Card { count: winning_count });
                // if winning_count > 0 {
                //     let points = 2_u32.pow(winning_count-1);
                //     sum += points;
                //     //println!("{}", points);
                // }

            }
        }

        let mut score: [u32; INPUT_LENGTH] = [0; INPUT_LENGTH];
        score[INPUT_LENGTH - 1] = cards[INPUT_LENGTH - 1].count as u32;
        for (i, card) in cards.iter().enumerate().rev() {
            //println!("{}", i);
            //println!("{:?}", card);
            score[i] = card.count as u32 + score[i+1..i+card.count+1].iter().sum::<u32>();
            //println!("{:?}", score[i]);

        }

        println!("{}", score.iter().sum::<u32>() + cards.len() as u32);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}