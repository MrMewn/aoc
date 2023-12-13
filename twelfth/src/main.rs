use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::{Itertools, repeat_n};
use regex::Regex;

fn main() {
    println!("{}", solution("./input.txt", true));
}

/* 
 * '.' operational
 * '#' damaged
 * '?' unknown
 * 
 * k,l,m size of each ordered grouping of damaged springs
 * - accounts for all damaged springs
 * - total size of group
 * - any grouping has size <= 10 (aka they're not that big)
 */

fn solution(filename: &str, unfolded: bool) -> i64 {
    if let Ok(lines) = read_lines(filename) {
        let mut records: Vec<(String, Vec<i8>)> = vec![];
        for result in lines {
            if let Ok(line) = result {
                let (raw_springs, raw_criteria) = line.split_once(" ").unwrap();
                let (springs, criteria) = (
                    if unfolded {
                        (raw_springs.to_string() + "?").repeat(5).strip_suffix("?").unwrap().to_string()
                    } else {
                        raw_springs.to_string()
                    },
                    if unfolded {
                        raw_criteria.split(",")
                            .map(|number| number.parse::<i8>()).flatten().collect_vec().repeat(5)
                    } else {
                        raw_criteria.split(",")
                            .map(|number| number.parse::<i8>()).flatten().collect_vec()
                    }
                );
                records.push((springs, criteria));
            }
        }

        //println!("{:?}", records);

        let mut sum: i64 = 0;

        for (springs, criteria) in records {
            let (width, height) = (springs.len() + 1, criteria.len() + 1);
            let expressions = (0..height).map(|l| (if l == 0 { 0 } else { criteria[l - 1] }, expression(criteria.iter().take(l).map(|number| *number).collect_vec()))).collect_vec();
            let mut memoize = repeat_n(repeat_n(-1_i64, width).collect_vec(), height).collect_vec();

            for j in 0..height {
                for i in 0..width {
                    let substring = if i == 0 { String::from("") } else { springs[..i].to_string() };
                    memoize[j][i] = evaluate(&expressions[j], substring, (i, j), &memoize);
                    //pretty_print(&memoize);
                }
            }

            sum += memoize[height - 1][width - 1];
        }

        return sum;
    }

    0
}

fn evaluate(expression: &(i8, Regex), substring: String, position: (usize, usize), memoize: &Vec<Vec<i64>>) -> i64 {
    let (i, j) = position;
    let (l, regular_expression) = expression;

    match substring.chars().last() {
        Some('#') => {
            if j == 0 {
                return 0;
            }

            if regular_expression.is_match(&substring) {
                memoize[j - 1][if i - *l as usize == 0 { 0 } else { i - *l as usize - 1 }]
            } else {
                0
            }
        },
        Some('.') => {
            memoize[j][i - 1]
        },
        Some('?') => {
            let damaged = substring.strip_suffix("?").unwrap().to_string() + "#";
            let operational = substring.strip_suffix("?").unwrap().to_string() + ".";

            evaluate(expression, damaged, position, memoize) + evaluate(expression, operational, position, memoize)
        },
        _ => {
            if j == 0 && i == 0 {
                1
            } else {
                0
            }
        },
    }
}

fn expression(criteria: Vec<i8>) -> Regex {
    let inner = criteria.iter().map(|number| format!(r"[#?]{{{number}}}[.?]+")).collect::<String>();
    if !inner.is_empty() {
        let expression = r"^[.?]*".to_string() + inner.strip_suffix("+").unwrap() + "*$";
        return Regex::new(expression.as_str()).unwrap();
    }

    Regex::new(r"^[.?]*$").unwrap()
}

#[allow(dead_code)]
fn pretty_print(memoize: &Vec<Vec<i64>>) {
    memoize.iter().for_each(|row| println!("{:?}", row));
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
    fn test_1_input() {
        let sum = solution("./test_1_input.txt", false);
        assert_eq!(sum, 1);
    }

    
    #[test]
    fn test_1_input_unfolded() {
        let sum = solution("./test_1_input.txt", true);
        assert_eq!(sum, 1);
    }

    #[test]
    fn test_2_input() {
        let sum = solution("./test_2_input.txt", false);
        assert_eq!(sum, 21);
    }

    #[test]
    fn test_2_input_unfolded() {
        let sum = solution("./test_2_input.txt", true);
        assert_eq!(sum, 525152);
    }
}