use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::Itertools;

type Map = Vec<(u64, u64, u64)>;

fn main() {
    solution();
}

// It would be more efficient to start with locations probably and move backwards.

fn solution() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut iterator = lines.flatten();
        
        let seeds = iterator.next()
            .unwrap()
            .strip_prefix("seeds: ")
            .unwrap()
            .split_ascii_whitespace()
            .map(|number| number.parse::<u64>())
            .flatten()
            .tuples::<(u64, u64)>()
            .map(|(start, range)| start..(start + range))
            .collect_vec();

        let maps = iterator.skip(1)
            .filter(|line| !line.ends_with(" map:"))
            .map(|line| parse_line(&line))
            .fold(vec![vec![]], |mut maps: Vec<Map>, line| {
                match line {
                    Some((to, from, range)) => {
                        let last = maps.len() - 1;
                        maps[last].push((to, from, range));
                    },
                    _ => maps.push(vec![])
                }
                maps
            });
        
        let mut i: u64 = 0;
        loop {
            let seed = maps.iter().rev()
                .fold(i, |source, map| apply(map, source));

            if seeds.iter().any(|range| range.contains(&seed)) {
                break;
            }
        
            i += 1;
        }
        println!("{}", i);
    }
        
}

fn apply(map: &Map, source: u64) -> u64 {
    for &(to, from, range) in map {
        if (to..(to+range)).contains(&source) {
            return source + from - to;
        }
    }

    return source;
}

fn parse_line(line: &str) -> Option<(u64, u64, u64)> {
    if line.is_empty() {
        return None;
    }

    return line.split_ascii_whitespace().map(str::parse::<u64>).flatten().collect_tuple();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}