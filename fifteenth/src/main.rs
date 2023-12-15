use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::BTreeMap;
use std::collections::HashMap;
use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
enum Op {
    Add,
    Remove,
}

#[derive(Clone, Debug)]
struct Instruction {
    label: String,
    operand: Op,
    argument: u8
}

fn add(label: String, argument: u8) -> Instruction {
    Instruction {
        label: label,
        operand: Op::Add,
        argument: argument,
    }
}

fn remove(label: String) -> Instruction {
    Instruction {
        label: label,
        operand: Op::Remove,
        argument: 0,
    }
}

/**
 * Assuming the input is not constructed specifically against this and since
 * this won't be used for an actual compiler or in-production DS, I've tried just
 * always incrementing the index of new elements based on the largest index in the
 * "box" they're inserted into. That way, I won't need a custom LinkedList (hopefully).
 */

fn main() {
    println!("{}", solution("./input.txt"));
}

fn solution(filename: &str) -> u64 {
    if let Ok(lines) = read_lines(filename) {
        let mut instructions: Vec<Instruction> = vec![];
        for result in lines {
            if let Ok(line) = result {
                line.split(",")
                    .map(|step| parse(step))
                    .for_each(|instruction| instructions.push(instruction));
            }
        }
        
        let mut indexing: HashMap<String, u32> = HashMap::new();
        let mut forest: Vec<BTreeMap<u32, u8>> = (0..256).map(|_| BTreeMap::new()).collect_vec();

        instructions.iter().for_each(|instruction| eval(instruction, &mut indexing, &mut forest));

        return forest.iter().enumerate().map(|(tree_number, tree)| {
            tree.values().enumerate().map(|(slot, focal)| (tree_number as u64 + 1) * (slot as u64 + 1) * *focal as u64).sum::<u64>()
        }).sum();
    }

    0
}

fn parse(step: &str) -> Instruction {
    match step.split_once('=') {
        Some((label, argument)) => add(label.to_string(), argument.parse().unwrap_or_default()),
        None => remove(step.strip_suffix('-').unwrap().to_string()),
    }
}

fn eval(instruction: &Instruction, indexing: &mut HashMap<String, u32>, forest: &mut Vec<BTreeMap<u32, u8>>) {
    match instruction {
        Instruction { label, operand: Op::Add, argument } => {
            let tree = hash(label);
            if let Some(key) = indexing.get(label) {
                forest[hash(label) as usize].insert(*key, *argument);
            } else {
                let new_key = match forest[tree as usize].last_key_value() {
                    Some((key, _)) => key + 1,
                    None => 0,
                };

                indexing.insert(label.clone(), new_key);
                forest[hash(label) as usize].insert(new_key, *argument);
            }
        },
        Instruction { label, operand: Op::Remove, .. } => {
            let tree = hash(label);
            if let Some(key) = indexing.remove(label) {
                forest[tree as usize].remove(&key);
            }
        },
    }
    //println!("{:?}", forest);
}

fn hash(label: &str) -> u8 {
    label.chars().fold(0, |current_value, byte| ((current_value as u16 + byte as u16) * 17) as u8)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use crate::{solution, hash};

    #[test]
    fn test_hash() {
        let sum = hash("HASH");
        assert_eq!(sum, 52);
    }

    #[test]
    fn test_input() {
        let sum = solution("./test_input.txt");
        assert_eq!(sum, 145);
    }
}