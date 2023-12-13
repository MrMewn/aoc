use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::Itertools;

fn main() {
    println!("{}", solution("./input.txt", 999999));
}

fn solution(filename: &str, expansion_rate: usize) -> i64 {
    if let Ok(lines) = read_lines(filename) {
        let mut nodes: Vec<Vec<char>> = vec![];
        let mut galaxies: Vec<(usize, usize)> = vec![];

        // Space picture
        for result in lines {
            if let Ok(line) = result {
                let row = line.chars()
                    .enumerate()
                    .inspect(|(i, character)| if !is_empty(*character) { galaxies.push((*i, nodes.len())) })
                    .map(|(_, node)| node)
                    .collect_vec();
                nodes.push(row);
            }
        }

        // Expansion
        let expanded_rows = (0..nodes.len())
            .filter(|j| nodes[*j].iter().all(|character| is_empty(*character)))
            .collect_vec();

        let expanded_columns = (0..nodes.first().unwrap().len())
            .filter(|i| nodes.iter().map(|row| row[*i]).all(is_empty))
            .collect_vec();

        //println!("{:?}", expanded_rows);
        //println!("{:?}", expanded_columns);

        for (i, (x, y)) in galaxies.clone().iter().enumerate() {
            let dx = expanded_columns.iter().filter(|&c| *x > *c).count() * expansion_rate;
            let dy = expanded_rows.iter().filter(|&r| *y > *r).count() * expansion_rate;
            galaxies[i] = (*x + dx, *y + dy);
        }

        //println!("{:?}", nodes);
        //println!("{:?}", galaxies);

        // All pairs of galaxies
        let pairs = galaxies.iter().combinations(2).map(|combination| (combination[0].clone(), combination[1].clone())).collect_vec();

        //println!("{:?}", pairs);

        let mut sum: i64 = 0;
        for (g, h) in pairs {
            let distance = manhattan(g, h);
            //println!("{}", distance);
            sum += distance;
        }

        return sum;
    }

    0
}

fn manhattan(source: (usize, usize), target: (usize, usize)) -> i64 {
    let (x_0, y_0) = (source.0 as i64, source.1 as i64);
    let (x_1, y_1) = (target.0 as i64, target.1 as i64);
    (x_1 - x_0).abs() + (y_1 - y_0).abs()
}

fn is_empty(character: char) -> bool {
    match character {
        '.' => true,
        _ => false,
    }
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
        let sum = solution("./test_input.txt", 1);
        assert_eq!(sum, 374);
    }

    #[test]
    fn test_input_tenfold() {
        let sum = solution("./test_input.txt", 9);
        assert_eq!(sum, 1030);
    }

    #[test]
    fn test_input_hundredfold() {
        let sum = solution("./test_input.txt", 99);
        assert_eq!(sum, 8410);
    }
}