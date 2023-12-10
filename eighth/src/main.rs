use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use num::integer::lcm;

fn main() {
    solution();
}

fn solution() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut instructions: Vec<char> = vec![];
        let mut graph = HashMap::<String, (String, String)>::new();
        
        for (i, result) in lines.enumerate() {
            match i {
                0 => {
                    if let Ok(line) = result {
                        instructions = line.chars().collect();
                    }
                },
                1 => continue,
                _ => {
                    if let Ok(line) = result {
                        let (node, connections) = line.strip_suffix(")")
                            .unwrap()
                            .split_once(" = (")
                            .unwrap();
                        graph.insert(node.to_string(), connections.split_once(", ").map(|(a, b)| (a.to_string(), b.to_string())).unwrap());
                    }
                }
            }
        }

        let mut nodes: Vec<&String> = graph.keys().filter(|&key| key.ends_with("A")).collect();

        let result = nodes.iter().map(|&node| z_period(&graph, &instructions, node)).reduce(|acc, count| lcm(acc, count)).unwrap();

        println!("{}", result);
    }
}

fn z_period(graph: &HashMap<String, (String, String)>, instructions: &Vec<char>, node: &String) -> u64 {
    let mut current_node = node;
    let mut count: u64 = 0_u64;

    while !current_node.ends_with("Z") {
        let decision = graph.get(current_node).unwrap();
        match instructions[count as usize % instructions.len()] {
            'L' => current_node = &decision.0,
            _ => current_node = &decision.1,
        }

        count += 1;
    }

    return count;
}



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}