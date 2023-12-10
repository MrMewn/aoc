use core::num;
use std::borrow::BorrowMut;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

fn main() {
    solution();
}

// fn solution() {
//     if let Ok(lines) = read_lines("./test_input.txt") {
//         let mut graph: Vec<Vec<char>> = vec![vec![]; 140];
//         let mut numbers: Vec<Number> = vec![];
//         for (i, result) in lines.enumerate() {
//             if let Ok(line) = result {
//                 graph[i].append(line.chars().to_owned().collect::<Vec<char>>().borrow_mut());
//                 numbers.append(&mut find_numbers(&graph[i], i));
//             }
//         }
//         //println!("{:?}", graph);
//         //println!("{:?}", numbers);

//         let mut sum: u32 = 0;
//         for number in numbers {
//             if has_adjacent_symbol(&number, &graph) {
//                 //println!("{}", number.number);
//                 sum += number.number;
//             }
//         }
//         //println!("{}", sum);
//     }
// }

// #[derive(Debug, Clone)]
// struct Number {
//     low: isize,
//     high: isize,
//     row: isize,
//     number: u32
// }

fn solution() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut graph: Vec<Vec<char>> = vec![vec![]; 140];
        let mut symbols: Vec<Symbol> = vec![];
        for (i, result) in lines.enumerate() {
            if let Ok(line) = result {
                graph[i].append(line.chars().to_owned().collect::<Vec<char>>().borrow_mut());
                symbols.append(line.chars().enumerate().filter(|(_, c)| is_symbol(c)).map(|(j, _)| Symbol { row: i, column: j }).collect::<Vec<Symbol>>().as_mut());
            }
        }
        //println!("{:?}", graph);
        //println!("{:?}", symbols);

        let mut sum: i32 = 0;
        for symbol in symbols {
            let numbers = adjacent_numbers(&graph, symbol.row, symbol.column);
            //println!("{:?}", symbol);
            //println!("{:?}", numbers);
            if numbers.len() == 2 {
                let gear_number = numbers.iter().fold(1, |acc, x| acc * x);
                //println!("{:?}", symbol);
                //println!("{:?}", numbers);
                //println!("{}", gear_number);
                sum += gear_number;
            }
        }

        println!("{}", sum);
    }
}

#[derive(Debug)]
struct Symbol {
    row: usize,
    column: usize
}

fn adjacent_numbers(graph: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<i32> {
    let left = graph[i].split_at(j).0.iter().rev().take_while(|c| c.is_numeric()).map(char::to_string).collect::<String>().chars().rev().collect::<String>().parse::<i32>();
    let right = graph[i].split_at(j).1.iter().skip(1).take_while(|c| c.is_numeric()).map(char::to_string).collect::<String>().chars().collect::<String>().parse::<i32>();
    let above = adjacent_numbers_above(graph, i, j);
    let below = adjacent_numbers_below(graph, i, j);

    if left.is_ok() && right.is_ok() {
        return [vec![left.unwrap(), right.unwrap()], above, below].concat();
    }

    if left.is_ok() {
        return [vec![left.unwrap()], above, below].concat();
    }

    if right.is_ok() {
        return [vec![right.unwrap()], above, below].concat();
    }
    
    return [above, below].concat();
}

fn adjacent_numbers_above(graph: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<i32> {
    if i > 0 {
        let left_above = graph[i-1].split_at(j).0.iter()
            .rev()
            .take_while(|c| c.is_numeric())
            .map(char::to_string)
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();
        let right_above = graph[i-1].split_at(j).1.iter()
            .skip(1)
            .take_while(|c| c.is_numeric())
            .map(char::to_string)
            .collect::<String>();

        if graph[i-1][j].is_numeric() {
            match (left_above + (&graph[i-1][j].to_string() as &str) + &right_above).parse::<i32>() {
                Ok(number) => {
                    return vec![number];
                },
                _ => {
                    return vec![];
                }
            }
        }
        if !left_above.is_empty() && !right_above.is_empty() {
            return vec![left_above.parse::<i32>().unwrap(), right_above.parse::<i32>().unwrap()];
        }
        if left_above.is_empty() && !right_above.is_empty() {
            return vec![right_above.parse::<i32>().unwrap()];
        }
        if !left_above.is_empty() && right_above.is_empty() {
            return vec![left_above.parse::<i32>().unwrap()];
        }
    }

    return vec![];
}

fn adjacent_numbers_below(graph: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<i32> {
    if i < graph.len() - 1 {
        let left_below = graph[i+1].split_at(j).0.iter()
            .rev()
            .take_while(|c| c.is_numeric())
            .map(char::to_string)
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();
        let right_below = graph[i+1].split_at(j).1.iter()
            .skip(1)
            .take_while(|c| c.is_numeric())
            .map(char::to_string)
            .collect::<String>();

        if graph[i+1][j].is_numeric() {
            match (left_below + (&graph[i+1][j].to_string() as &str) + &right_below).parse::<i32>() {
                Ok(number) => {
                    return vec![number];
                },
                _ => {
                    return vec![];
                }
            }
        }
        if !left_below.is_empty() && !right_below.is_empty() {
            return vec![left_below.parse::<i32>().unwrap(), right_below.parse::<i32>().unwrap()];
        }
        if left_below.is_empty() && !right_below.is_empty() {
            return vec![right_below.parse::<i32>().unwrap()];
        }
        if !left_below.is_empty() && right_below.is_empty() {
            return vec![left_below.parse::<i32>().unwrap()];
        }
    }

    return vec![];
}

// fn find_numbers(line: &Vec<char>, row: usize) -> Vec<Number> {
//     let mut processing_number = false;
//     let mut low: isize = 0;
//     let mut high: isize;
//     let mut numbers: Vec<Number> = vec![];
//     for (i, character) in line.iter().enumerate() {
//         if processing_number && !character.is_numeric() {
//             high = i as isize;
//             numbers.push(Number { low: low, high: high, row: row as isize, number: (low..high).map(|pos| line[pos as usize].to_string()).reduce(|a, b| a + &b).unwrap().parse::<u32>().unwrap()});
//         }

//         if !processing_number && character.is_numeric() {
//             processing_number = true;
//             low = i as isize;
//         }

//         if !character.is_numeric() {
//             processing_number = false;
//         }
//     }
//     if processing_number {
//         high = line.iter().count() as isize;
//         numbers.push(Number { low: low, high: high, row: row as isize, number: (low..high).map(|pos| line[pos as usize].to_string()).reduce(|a, b| a + &b).unwrap().parse::<u32>().unwrap()});
//     }
//     return numbers;
// }

/* 
 * ....
 * .42.  <-- row
 * ...$
 *  ^-- low
 *   ^-- high
 * 
 * (low-1, row-1)..(high+1, row-1) if row-1 > 0 (and below conds)
 * (low-1, row+1)..(high+1, row+1) if row+1 < #rows (and below conds)
 * (low-1, row) if low-1 > 0, (high+1, row) if high+1 < len
 */
// fn has_adjacent_symbol(number: &Number, graph: &Vec<Vec<char>>) -> bool {
//     if number.low > 0 && is_symbol(&graph[number.row as usize][(number.low - 1) as usize]) {
//         return true;
//     }
//     if ((number.high) as usize) < graph[number.row as usize].len() && is_symbol(&graph[number.row as usize][(number.high) as usize]) {
//         return true;
//     }
//     if number.row > 0 && (number.low-1..number.high+1)
//         .filter(|&i| i >= 0 && (i as usize) < graph[number.row as usize].len())
//         .map(|i| &graph[(number.row - 1) as usize][i as usize])
//         .any(|c| is_symbol(c)) {
//         return true;
//     }
//     if ((number.row + 1) as usize) < graph.len() && (number.low-1..number.high+1)
//         .filter(|&i| i >= 0 && (i as usize) < graph[number.row as usize].len())
//         .map(|i| &graph[(number.row + 1) as usize][i as usize])
//         .any(|c| is_symbol(c)) {
//         return true;
//     }

//     return false;
// }

fn is_symbol(character: &char) -> bool {
    !character.is_numeric() && character.ne(&'.')
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}