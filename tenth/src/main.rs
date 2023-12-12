use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter;
use std::path::Path;
use itertools::Itertools;

type Graph = Vec<Vec<(String, bool)>>;

fn main() {
    println!("{:?}", solution("input.txt"));
}

fn solution(filename: &str) -> (i32, i32) {
    if let Ok(lines) = read_lines(filename) {
        let mut graph: Graph = vec![];
        let mut start = (0, 0);
        for result in lines {
            if let Ok(line) = result {
                let row = line.chars()
                    .enumerate()
                    .inspect(|(i, character)| if character == &'S' { start = (*i, graph.len()) })
                    .map(|(_, character)| (character.to_string(), false))
                    .collect_vec();
                graph.push(row);
            }
        }

        //println!("{:?}", graph);
        //println!("{:?}", start);
        let (max, graph) = bfs(start, graph);
        //println!("{}", max);

        //pretty_print(graph.clone());

        let area = area(graph);
        //println!("{}", area);

        return (max, area);
    }

    (-1, -1)
}

/// Breadth-first search, maintain max so far
fn bfs(start: (usize, usize), graph: Graph) -> (i32, Graph) {
    let (_, graph, max) = iter::repeat(420).scan((VecDeque::from([ (start, 0) ]), graph, 0), |(stack, graph, max_so_far), _| {
        if let Some(((i, j), distance)) = stack.pop_front() {
            //println!("dist {}", distance);
            let (symbol, _) = node(i, j, graph);
            graph[j][i] = (symbol, true);

            let next = destinations((i, j), graph);
            next.iter()
                .filter(|(x, y)| !visited(node(*x, *y, graph)))
                .for_each(|&(x, y)| stack.push_back(((x, y), distance + 1)));
            //println!("stack {:?}", stack);
            //println!();

            return Some((stack.clone(), graph.clone(), if distance > *max_so_far { distance } else { *max_so_far }));
        }
        return None;
    }).last().unwrap();
    //println!("{:?}", graph);
    //graph.iter().for_each(|row| println!("{:?}", row));
    return (max, graph);
}

fn area(graph: Graph) -> i32 {
    let (w, h) = (graph.first().unwrap().len(), graph.len());

    (0..w).cartesian_product(0..h).filter(|(i, j)| inside(*i, *j, &graph))/*.inspect(|(i, j)| println!("({}, {})", i, j))*/.map(|_| 1_i32).sum()
}

/// Ray-tracing (compute predicate from crossing number)
fn inside(i: usize, j: usize, graph: &Graph) -> bool {
    if visited(node(i, j, &graph)) {
        return false;
    }

    let wall_count = graph[j][i..].iter().enumerate()
        .filter(|(di, node)| {
            if node.0.as_str() == "S" {
                //println!("start wall {}", start_wall(i + di, j, &graph));
                visited_wall((start_wall(i + di, j, &graph), true))
            } else {
                visited_wall((*node).clone())
            }
        })
        .count();

    //println!("wall count {}", wall_count);
    wall_count % 2 != 0
}

fn destinations(source: (usize, usize), graph: &Graph) -> Vec<(usize, usize)> {
    let (i, j) = source;
    let (w, h) = (graph.first().unwrap().len() as isize, graph.len() as isize);
    //println!("current {:?}", graph[j][i]);

    match symbol(node(i, j, graph)).as_str() {
        /* north, south */
        "|" if in_bounds((i as isize, j as isize - 1), (w, h))
            && in_bounds((i as isize, j as isize + 1), (w, h))
            && visited(node(i, j - 1, graph)) => {
            vec![ (i, j + 1) ]
        },
        "|" if in_bounds((i as isize, j as isize + 1), (w, h))
            && in_bounds((i as isize, j as isize - 1), (w, h))
            && visited(node(i, j + 1, graph)) => {
            vec![ (i, j - 1) ]
        },
        /* west, east */
        "-" if in_bounds((i as isize - 1 , j as isize), (w, h))
            && in_bounds((i as isize + 1, j as isize), (w, h))
            && visited(node(i - 1, j, graph)) => {
            vec![ (i + 1, j) ]
        },
        "-" if in_bounds((i as isize + 1, j as isize), (w, h))
            && in_bounds((i as isize - 1, j as isize), (w, h))
            && visited(node(i + 1, j, graph)) => {
            vec![ (i - 1, j) ]
        },
        /* north, east */
        "L" if in_bounds((i as isize, j as isize - 1), (w, h))
            && in_bounds((i as isize + 1, j as isize), (w, h))
            && visited(node(i, j - 1, graph)) => {
            vec![ (i + 1, j) ]
        },
        "L" if in_bounds((i as isize + 1, j as isize), (w, h))
            && in_bounds((i as isize, j as isize - 1), (w, h))
            && visited(node(i + 1, j, graph)) => {
            vec![ (i, j - 1) ]
        },
        /* north, west */
        "J" if in_bounds((i as isize, j as isize - 1), (w, h))
            && in_bounds((i as isize - 1, j as isize), (w, h))
            && visited(node(i, j - 1, graph)) => {
            vec![ (i - 1, j) ]
        },
        "J" if in_bounds((i as isize - 1, j as isize), (w, h))
            && in_bounds((i as isize, j as isize - 1), (w, h))
            && visited(node(i - 1, j, graph)) => {
            vec![ (i, j - 1) ]
        },
        /* south, west */
        "7" if in_bounds((i as isize, j as isize + 1), (w, h))
            && in_bounds((i as isize - 1, j as isize), (w, h))
            && visited(node(i, j + 1, graph)) => {
            vec![ (i - 1, j) ]
        },
        "7" if in_bounds((i as isize - 1, j as isize), (w, h))
            && in_bounds((i as isize, j as isize + 1), (w, h))
            && visited(node(i - 1, j, graph)) => {
            vec![ (i, j + 1) ]
        },
        /* south, east */
        "F" if in_bounds((i as isize, j as isize + 1), (w, h))
            && in_bounds((i as isize + 1, j as isize), (w, h))
            && visited(node(i, j + 1, graph)) => {
            vec![ (i + 1, j) ]
        },
        "F" if in_bounds((i as isize + 1, j as isize), (w, h))
            && in_bounds((i as isize, j as isize + 1), (w, h))
            && visited(node(i + 1, j, graph)) => {
            vec![ (i, j + 1) ]
        },
        /* start */
        "S" => {
            [ (i as isize, j as isize - 1), (i as isize, j as isize + 1), (i as isize + 1, j as isize), ((i as isize - 1), j as isize) ].iter()
                .filter(|(x, y)| in_bounds((*x as isize, *y as isize), (w, h)))
                .filter(|(x, y)| connected(source, (*x as usize, *y as usize), symbol(node(*x as usize, *y as usize, graph))))
                .map(|(i, j)| (*i as usize, *j as usize))
                .collect_vec()
        },
        /* unconnected ground (and any illegal symbol) */
        _ => {
            vec![]
        }, 
    }
}

fn start_wall(i: usize, j: usize, graph: &Graph) -> String {
    let north = if j > 0 { connected((i, j), north(i, j), symbol(node(north(i, j).0, north(i, j).1, &graph))) } else { false };
    let south = if j < graph.len() { connected((i, j), south(i, j), symbol(node(south(i, j).0, south(i, j).1, &graph))) } else { false };
    let east = if i < graph.first().unwrap().len() { connected((i, j), east(i, j), symbol(node(east(i, j).0, east(i, j).1, &graph))) } else { false };
    let west =  if i > 0 { connected((i, j), west(i, j), symbol(node(west(i, j).0, west(i, j).1, &graph))) } else { false };

    //println!("{} {}", i, j);
    //println!("{:?}", (north, south, east, west));
    match (north, south, east, west) { // TODO: fix bug in type logic
        (true, true, false, false) => "|",
        (true, false, true, false) => "L",
        (true, false, false, true) => "J",
        (false, true, true, false) => "F",
        (false, true, false, true) => "7",
        (false, false, true, true) => "-",
        _ => "."
    }.to_string()
}

fn connected(source: (usize, usize), destination: (usize, usize), character: String) -> bool {
    let direction = (destination.0 as isize - source.0 as isize, destination.1 as isize - source.1 as isize);
    //println!("diff {:?}", direction);

    /*               NORTH
     *              (0, -1)
     * WEST (-1, 0) (0,  0) (1, 0) EAST
     *              (0,  1)
     *               SOUTH
     */

    match character.as_str() {
        "|" => direction == (0, 1) || direction == (0, -1),
        "-" => direction == (1, 0) || direction == (-1, 0),
        "L" => direction == (0, 1) || direction == (-1, 0),
        "J" => direction == (0, 1) || direction == (1, 0),
        "7" => direction == (1, 0) || direction == (0, -1),
        "F" => direction == (0, -1) || direction == (-1, 0),
        _ => false
    }
}

fn north(i: usize, j: usize) -> (usize, usize) {
    (i, j - 1)
}

fn south(i: usize, j: usize) -> (usize, usize) {
    (i, j + 1)
}

fn east(i: usize, j: usize) -> (usize, usize) {
    (i + 1, j)
}

fn west(i: usize, j: usize) -> (usize, usize) {
    (i - 1, j)
}

fn node(i: usize, j: usize, graph: &Graph) -> (String, bool) {
    graph[j][i].clone()
}

fn symbol(node: (String, bool)) -> String {
    node.0
}

fn visited(node: (String, bool)) -> bool {
    node.1
}

fn visited_wall(node: (String, bool)) -> bool {
    match node.0.as_str() {
        "|" | "L" | "J"/* | "7" | "F"*/ => node.1, // if we count both north-facing and south-facing angles, we hit the edge-case 'L7', which corresponds to '-'
        _ => false
    }
}

fn in_bounds(indices: (isize, isize), bounds: (isize, isize)) -> bool {
    let (i, j) = indices;
    let (w, h) = bounds;

    i >= 0 && i < w && j >= 0 && j < h
}

#[allow(dead_code)]
fn pretty_print(graph: Graph) {
    let display_cycle = |(symbol, visited)| if visited { symbol } else { "*".to_string() };
    graph.iter().for_each(|row| println!("{}", format!("{:?}", row.iter().map(|node| display_cycle(node.clone())).format(" "))) );
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
        let (max, area) = solution("test_1_input.txt");
        assert_eq!(max, 4);
        assert_eq!(area, 1);
    }

    #[test]
    fn test_2_input() {
        let (max, area) = solution("test_2_input.txt");
        assert_eq!(max, 8);
        assert_eq!(area, 1);
    }

    #[test]
    fn test_3_input() {
        let (_, area) = solution("test_3_input.txt");
        assert_eq!(area, 4);
    }

    #[test]
    fn test_4_input() {
        let (_, area) = solution("test_4_input.txt");
        assert_eq!(area, 4);
    }

    #[test]
    fn test_5_input() {
        let (_, area) = solution("test_5_input.txt");
        assert_eq!(area, 8);
    }

    #[test]
    fn test_6_input() {
        let (_, area) = solution("test_6_input.txt");
        assert_eq!(area, 10);
    }
}