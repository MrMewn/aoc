use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    solution();
}

fn solution() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut sum: u32 = 0;

        for line in lines {
            if let Ok(values) = line {
                sum += parse_line(&values);
            }
        }

        println!("{}", sum);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_line_no_words(line: &str) -> u32 {
    match (line.find(is_number), line.rfind(is_number)) {
        (Some(first_pos), Some(last_pos)) => {
            let first_digit = parse_number_word(&line[first_pos..first_pos+1]).to_string();
            let last_digit = parse_number_word(&line[last_pos..last_pos+1]);

            (first_digit + last_digit).parse::<u32>().unwrap()
        },
        _ => 0
    }
}

fn parse_line(line: &str) -> u32 {
    match (find_first_number(&line, line.find(is_number)), find_last_number(&line, line.rfind(is_number))) {
        ((first_pos, first_size), (last_pos, last_size)) => {
            let first_digit = parse_number_word(&line[first_pos..first_pos+first_size]).to_string();
            let last_digit = parse_number_word(&line[last_pos..last_pos+last_size]);

            (first_digit + last_digit).parse::<u32>().unwrap()
        }
    }
}

fn find_first_number(line: &str, first_number_pos: Option<usize>) -> (usize, usize) {
    let bound = match first_number_pos {
        Some(pos) => pos,
        None => line.len()
    };

    for i in 0..bound {
        let next = &line[i..bound];
        let found = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].iter()
            .filter(|&number_word| next.starts_with(number_word))
            .last();

        match found {
            Some(number_word) => return (i, number_word.len()),
            None => continue
        }
    }
    (bound, 1)
}

fn find_last_number(line: &str, last_number_pos: Option<usize>) -> (usize, usize) {
    let bound = match last_number_pos {
        Some(pos) => pos,
        None => 0
    };

    for i in 0..line.len()-bound {
        let next = &line[bound..line.len()-i];
        let found = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].iter()
            .filter(|&number_word| next.ends_with(number_word))
            .last();

        match found {
            Some(number_word) => return (line.len()-i-number_word.len(), number_word.len()),
            None => continue
        }
    }
    (bound, 1)
}

fn is_number(character: char) -> bool {
    character > '/' && character < ':'
}

fn parse_number_word(word: &str) -> &str {
    match word {
        "one" | "1" => "1",
        "two" | "2" => "2",
        "three" | "3" => "3",
        "four" | "4" => "4",
        "five" | "5" => "5",
        "six" | "6" => "6",
        "seven" | "7" => "7",
        "eight" | "8" => "8",
        "nine" | "9" => "9",
        _ => "undefined"
    }
}