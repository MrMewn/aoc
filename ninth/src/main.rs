use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::Itertools;

fn main() {
    solution();
}

fn solution() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut histories: Vec<Vec<i32>> = vec![];
        for result in lines {
            if let Ok(line) = result {
                histories.push(line.split_ascii_whitespace().map(|number| number.parse::<i32>()).flatten().collect_vec());
            }
        }

        //println!("{:?}", histories);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}