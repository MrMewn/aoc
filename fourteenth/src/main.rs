use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Shape {
    Cubic,
    Round,
    Void,
}

type Platform = Vec<Vec<Shape>>;

fn main() {
    println!("{}", solution("./input.txt"));
}

/* 
 * `#` cubic rocks
 * `O` round rocks
 * `.` empty space
 * 
 * - 100 x 100 input
 */

fn solution(filename: &str) -> i64 {
    if let Ok(lines) = read_lines(filename) {
        let mut platform: Platform = vec![];

        for result in lines {
            if let Ok(line) = result {
                platform.push(line.chars().map(shape).collect_vec());
            }
        }

        //records = transpose(records);

        //pretty_print(&record);
        // let mut data: Vec<i64> = vec![];
        // let n = 100;
        // for _ in 0..n {
        //     cycle_n(&mut platform, 10);
        //     data.push(north_load(&platform));
        // }
        //println!("{:?}", data);

        /* I'm not going to pretend I understand why this behaviour is periodic like this, but
         * I managed to guess so by plotting the behaviour, so voilá.
         * I guess one rigid solution might be to write some code that figures out when the cycle repeats,
         * then use that to compute the actual value at one million cycles.
         */
        cycle_n(&mut platform, 1000000);
        //pretty_print(&platform);
        println!("load {}", north_load(&platform));
        
    }

    0
}

fn cycle_n(platform: &mut Platform, n: i64) {
    for _ in 0..n {
        cycle(platform);
    }
}
/**
 * Completes a cycle (^<v>) of tilts.
 */
fn cycle(platform: &mut Platform) {
    north(platform);
    west(platform);
    south(platform);
    east(platform);
}

/**
 * Tilts the platform north.
 */
fn north(platform: &mut Platform) {
    let (w, h) = (platform.first().unwrap().len(), platform.len());
    for i in 0..w {
        let mut prev = 0;
        for j in 0..h {
            match platform[j][i] {
                Shape::Void => /* do nothing */ (),
                Shape::Cubic => {
                    prev = j + 1;
                },
                Shape::Round => {
                    let temp = platform[prev][i].clone();
                    platform[prev][i] = platform[j][i];
                    platform[j][i] = temp;
                    prev = prev + 1;
                },
            }
        }
    }
}

/**
 * Tilts the platform west.
 */
fn west(platform: &mut Platform) {
    let (w, h) = (platform.first().unwrap().len(), platform.len());
    for j in 0..h {
        let mut prev = 0;
        for i in 0..w {
            match platform[j][i] {
                Shape::Void => /* do nothing */ (),
                Shape::Cubic => {
                    prev = i + 1;
                },
                Shape::Round => {
                    let temp = platform[j][prev].clone();
                    platform[j][prev] = platform[j][i];
                    platform[j][i] = temp;
                    prev = prev + 1;
                },
            }
        }
    }
}

/**
 * Tilts the platform east.
 */
fn east(platform: &mut Platform) {
    let (w, h) = (platform.first().unwrap().len(), platform.len());
    for j in 0..h {
        let mut prev = 0;
        for i in 0..w {
            match platform[j][w - i - 1] {
                Shape::Void => /* do nothing */ (),
                Shape::Cubic => {
                    prev = i + 1;
                },
                Shape::Round => {
                    let temp = platform[j][w - prev - 1].clone();
                    platform[j][w - prev - 1] = platform[j][w - i - 1];
                    platform[j][w - i - 1] = temp;
                    prev = prev + 1;
                },
            }
        }
    }
}

/**
 * Tilts the platform south.
 */
fn south(platform: &mut Platform) {
    let (w, h) = (platform.first().unwrap().len(), platform.len());
    for i in 0..w {
        let mut prev = 0;
        for j in 0..h {
            match platform[h - j - 1][i] {
                Shape::Void => /* do nothing */ (),
                Shape::Cubic => {
                    prev = j + 1;
                },
                Shape::Round => {
                    let temp = platform[h - prev - 1][i].clone();
                    platform[h - prev - 1][i] = platform[h - j - 1][i];
                    platform[h - j - 1][i] = temp;
                    prev = prev + 1;
                },
            }
        }
    }
}

#[allow(dead_code)]
fn north_load(platform: &Platform) -> i64 {
    let (w, h) = (platform.first().unwrap().len(), platform.len());
    
    (0..w).cartesian_product(0..h).fold(0, |load, (i, j)| {
        match platform[j][i] {
            Shape::Void => load,
            Shape::Cubic => load,
            Shape::Round => load + (h - j) as i64,
        }
    })
}

#[allow(dead_code)]
fn compute_tilt_north_load(column: &Vec<Shape>) -> i64 {
    let (column_load, _) = column.iter()
        .rev()
        .enumerate()
        .map(|(i, &shape)| (i as i64, shape))
        .rev()
        .fold((0, column.len() as i64 - 1), |(load, level), (i, space)| {
        match space {
            Shape::Void => (load, level),
            Shape::Cubic => (load, i - 1),
            Shape::Round => (load + level + 1, level - 1),
        }
    });

    //println!("{}", column_load);

    column_load
}

fn shape(character: char) -> Shape {
    match character {
        '#' => Shape::Cubic,
        'O' => Shape::Round,
        '.' => Shape::Void,
        _ => {
            eprintln!("Malformed input!");
            Shape::Void
        },
    }
}

fn character(shape: Shape) -> char {
    match shape {
        Shape::Cubic => '#',
        Shape::Round => 'O',
        Shape::Void => '.',
    }
}

#[allow(dead_code)]
fn pretty_print(platform: &Platform) {
    platform.iter().for_each(|column| println!("{:?}", column.iter().map(|&shape| character(shape)).collect_vec()));
    println!();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn test_input() {
        let sum = solution("./test_input.txt");
        assert_eq!(sum, 136);
    }
}