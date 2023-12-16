use std::collections::{VecDeque, HashMap};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    println!("{}", solution("./test_input.txt"));
}

fn solution(filename: &str) -> u64 {
    let contraption: Vec<Vec<(char, bool)>> = read_input(filename);
    let (width, height) = (contraption.first().unwrap_or(&(vec![] as Vec<(char, bool)>)).len(), contraption.len());
    let mut max_so_far = 0;
    for i in 0..width {
        max_so_far = energy((Direction::South, i, 0), &contraption).max(max_so_far);
        max_so_far = energy((Direction::North, i, height - 1), &contraption).max(max_so_far);
    }
    for j in 0..height {
        max_so_far = energy((Direction::East, 0, j), &contraption).max(max_so_far);
        max_so_far = energy((Direction::West, width - 1, j), &contraption).max(max_so_far);
    }
    return max_so_far;
}

fn read_input(filename: &str) -> Vec<Vec<(char, bool)>> {
    if let Ok(lines) = read_lines(filename) {
        return lines.flatten().map(|line| line.chars().map(|c| (c, false)).collect()).collect();
    }
    vec![]
}

fn energy(entry: (Direction, usize, usize), contraption: &Vec<Vec<(char, bool)>>) -> u64 {
    let mut states: HashMap<(usize, usize), (bool, bool, bool, bool)> = HashMap::new();
    let (width, height) = (contraption.first().unwrap_or(&(vec![] as Vec<(char, bool)>)).len(), contraption.len());
    let mut queue: VecDeque<(Direction, usize, usize)> = VecDeque::from([ entry ]);

    while let Some((direction, i, j)) = queue.pop_front() {
        if !is_energized(direction, (i, j), &states) {
            next(contraption[j][i].0, direction, (i, j), (width, height)).iter().for_each(|ray| queue.push_back(*ray));
            energize(direction, (i, j), &mut states);
        }
    }

    states.len() as u64
}

fn next(symbol: char, direction: Direction, position: (usize, usize), bounds: (usize, usize)) -> Vec<(Direction, usize, usize)> {
    let (i, j) = position;
    let (width, height) = bounds;

    match (direction, symbol) {
        // North
        (Direction::North, '\\') if i > 0 => {
            vec![(Direction::West, i - 1, j)]
        },
        (Direction::North, '/') if i < width - 1 => {
            vec![(Direction::East, i + 1, j)]
        },
        (Direction::North, '-') => {
            [next('\\', direction, position, bounds), next('/', direction, position, bounds)].concat()
        },
        (Direction::North, '|') | (Direction::North, '.') if j > 0 => {
            vec![(Direction::North, i, j - 1)]
        },
        // South
        (Direction::South, '\\') if i < width - 1 => {
            vec![(Direction::East, i + 1, j)]
        },
        (Direction::South, '/') if i > 0 => {
            vec![(Direction::West, i - 1, j)]
        },
        (Direction::South, '-') => {
            [next('\\', direction, position, bounds), next('/', direction, position, bounds)].concat()
        },
        (Direction::South, '|') | (Direction::South, '.') if j < height - 1 => {
            vec![(Direction::South, i, j + 1)]
        },
        // East
        (Direction::East, '\\') if j < height - 1 => {
            vec![(Direction::South, i, j + 1)]
        },
        (Direction::East, '/') if j > 0 => {
            vec![(Direction::North, i, j - 1)]
        },
        (Direction::East, '-') | (Direction::East, '.') if i < width - 1 => {
            vec![(Direction::East, i + 1, j)]
        },
        (Direction::East, '|') => {
            [next('\\', direction, position, bounds), next('/', direction, position, bounds)].concat()
        },
        // West
        (Direction::West, '\\') if j > 0 => {
            vec![(Direction::North, i, j - 1)]
        },
        (Direction::West, '/') if j < height - 1 => {
            vec![(Direction::South, i, j + 1)]
        },
        (Direction::West, '-') | (Direction::West, '.') if i > 0 => {
            vec![(Direction::West, i - 1, j)]
        },
        (Direction::West, '|') => {
            [next('\\', direction, position, bounds), next('/', direction, position, bounds)].concat()
        },
        // Out of bounds
        _ => vec![],
    }
}

fn is_energized(direction: Direction, position: (usize, usize), states: &HashMap<(usize, usize), (bool, bool, bool, bool)>) -> bool {
    match states.get(&position) {
        Some(&state) => orientation(state, direction),
        _ => false,
    }
}

fn orientation(state: (bool, bool, bool, bool), direction: Direction) -> bool {
    match direction {
        Direction::North => state.0,
        Direction::South => state.1,
        Direction::East => state.2,
        Direction::West => state.3,
    }
}

fn update(state: (bool, bool, bool, bool), direction: Direction) -> (bool, bool, bool, bool) {
    let (north, south, east, west) = state;
    match direction {
        Direction::North => (true, south, east, west),
        Direction::South => (north, true, east, west),
        Direction::East =>  (north, south, true, west),
        Direction::West =>  (north, south, east, true),
    }
}

fn empty() -> (bool, bool, bool, bool) {
    (false, false, false, false)
}

fn energize(direction: Direction, position: (usize, usize), states: &mut HashMap<(usize, usize), (bool, bool, bool, bool)>) {
    if let Some(&previous_state) = states.get(&position) {
        states.insert(position, update(previous_state, direction));
    } else {
        states.insert(position, update(empty(), direction));
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use crate::{solution, energy, read_input, Direction};

    #[test]
    fn test_input_energy() {
        let contraption = read_input("./test_input.txt");
        let sum = energy((Direction::East, 0, 0), &contraption);
        assert_eq!(sum, 46);
    }

    #[test]
    fn test_input_max() {
        let sum = solution("./test_input.txt");
        assert_eq!(sum, 51);
    }
}