use std::fs::File;
use std::io::{self, BufRead};
use std::iter;
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
        let mut sum: i32 = 0;
        for history in histories {
            let differences = differences(history.clone());
            //println!("{:?} {:?}", histories[0], differences);
            let prediction = predict_history_backwards(history.clone(), differences);
            //println!("{}", prediction);
            sum += prediction;
        }

        println!("{}", sum);
    }
}

fn differences(history: Vec<i32>) -> Vec<Vec<i32>> {
    iter::repeat(69).scan(history, |state, _| {
        if state.iter().all(|value| value == &0) {
            return None;
        }

        state.iter().tuple_windows::<(_,_)>().map(|window| window.1 - window.0).collect_vec().clone_into(state);
        //println!("{:?}", state);

        Some(state.clone())
    }).collect_vec()
}

#[allow(dead_code)]
fn predict_history_forwards(first: Vec<i32>, next: Vec<Vec<i32>>) -> i32 {
    next.iter().rev().fold(0, |prediction, values| values.last().unwrap() + prediction) + first.last().unwrap()
}

fn predict_history_backwards(first: Vec<i32>, next: Vec<Vec<i32>>) -> i32 {
    first.first().unwrap() - next.iter().rev().fold(0, |prediction, values| values.first().unwrap() - prediction)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}