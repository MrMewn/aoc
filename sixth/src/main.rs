use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    solution();
}

fn solution() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut iterator = lines.flatten();
        // let times = &iterator.next()
        //     .unwrap()
        //     .strip_prefix("Time: ")
        //     .unwrap()
        //     .trim()
        //     .split_ascii_whitespace()
        //     .map(|number| number.parse::<u32>())
        //     .flatten()
        //     .collect_vec();
        // let distances = &iterator.next()
        //     .unwrap()
        //     .strip_prefix("Distance: ")
        //     .unwrap()
        //     .trim()
        //     .split_ascii_whitespace()
        //     .map(|number| number.parse::<u32>())
        //     .flatten()
        //     .collect_vec();
        // let races = times.iter()
        //     .zip(distances)
        //     .collect_vec();

        //let result = races.iter().map(|&(time, distance)| count_solutions(time, distance)).fold(1, |acc: u32, x| acc * x);
        let time = iterator.next()
            .unwrap()
            .strip_prefix("Time: ")
            .unwrap()
            .replace(" ", "")
            .parse::<u64>().expect("Failed to parse time.");
        let distance = iterator.next()
            .unwrap()
            .strip_prefix("Distance: ")
            .unwrap()
            .replace(" ", "")
            .parse::<u64>().expect("Failed to parse distance.");
        println!("{}", count_solutions(&time, &distance));
    }
        
}

fn count_solutions(time: &u64, distance: &u64) -> u64 {
    let mut count = 0;
    let mut found = false;
    for t0 in 1..*time {
        if (time - t0) * t0 >= *distance {
            count += 1;
            found = true;
        } else if found {
            break;
        }
    }

    return count;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}