use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::Itertools;
use itertools::FoldWhile::{ Continue, Done };

type Pattern = Vec<Vec<char>>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Orientation {
    Horizontal,
    Vertical,
}

fn main() {
    println!("{}", solution("./input.txt", true));
}

/* 
 * '.' ash
 * '#' rocks
 * 
 * - may ignore outer-most lines when odd # of columns/rows
 * - likely sub-100 lines of each type to get pretty numbers?
 * - single line of reflection in each pattern
 * 
 * sliding window
 * when match found for window size, increase window and try again
 * otherwise stop and check if match touches edge
 * 
 * sliding window of decreasing size
 * on first success check if touches edge
 * 
 * This ended up a mess due to bug-fixing, but I'll probably clean this up sometime.
 */

fn solution(filename: &str, smudged: bool) -> i64 {
    if let Ok(lines) = read_lines(filename) {
        let mut notes: Vec<Pattern> = vec![];
        let mut pattern: Pattern = vec![];

        for result in lines {
            if let Ok(line) = result {
                if line.is_empty() {
                    notes.push(pattern.clone());
                    pattern = vec![];
                    continue;
                }

                pattern.push(line.chars().collect_vec());
            }
        }

        notes.push(pattern.clone());

        let mut sum: i64 = 0;

        for pattern in notes {
            let (width, height) = (pattern.first().unwrap().len(), pattern.len());
            let (old_score, old_orientation) =  pattern_score(&pattern).unwrap();

            let count = if smudged {
                (0..height).cartesian_product(0..width).fold_while(0, |_, (j, i)| {
                    let flipped_pattern = flip((i, j), &pattern);
                    let new_score = new_pattern_score(&flipped_pattern, (old_score, old_orientation));
                    // pretty_print(&flipped_pattern);
                    match new_score {
                        Some(count) => {
                            Done(count)
                        },
                        _ => Continue(0),
                    }
                }).into_inner()
            } else {
                match pattern_score(&pattern)  {
                    Some((score, _)) => score,
                    _ => 0,
                }
            };

            println!("{}", count);

            sum += count as i64;
        }
        
        return sum;
    }

    0
}

/**
 * Bug #1 (FIXED)
 * 
 *  ###..###.
 *  .#....#..
 *  .#....#.# v
 *  ------------ 3
 *  .#....#.# ^
 *  .#....#..
 *  ###..###.
 *  .#....#..
 *  ##.##.###
 *  ...##....
 *  ########.
 *  #.####.##
 *  #.####.##
 *  .######.#
 *  ..####...
 *  .#.##.#.#
 *  #......#.
 *  ..#####..
 *        ^------- This is the smudge at (6, 16)
 *
 *      4
 *     >|<
 *  ###.|.###.
 *  .#..|..#..
 *  .#..|..#.#
 *  .#..|..#.#
 *  .#..|..#..
 *  ###.|.###.
 *  .#..|..#..
 *  ##.#|#.###
 *  ...#|#....
 *  ####|####.
 *  #.##|##.##
 *  #.##|##.##
 *  .###|###.#
 *  ..##|##...
 *  .#.#|#.#.#
 *  #...|...#.
 *  ..##|##...
 * 
 * 
 * Bug #2
 * 
 *  ..#..#..###
 *  #.#.#...### v
 *  -------------- 2
 *  #.#.#...### ^
 *  ..#..#..###
 *  #...#..##..
 *  #..##.#....
 *  .#.###.#.##
 *  #.....#....
 *  ##.##.##..#
 *  #..##.##..#
 *  #.....#....
 * 
 *  ..#..#..###
 *  #.#.#...###
 *  #.#.#...###
 *  ..#..#..###
 *  #...#..##..
 *  #..##.#....
 *  .#.###.#.##
 *  #.....#....
 *  ##.##.##..# v
 *  -------------- 9
 *  ##.##.##..# ^
 *  #.....#....
 */

fn new_pattern_score(pattern: &Pattern, old_pattern_score: (i64, Orientation)) -> Option<i64> {
    let (old_score, old_orientation) = old_pattern_score;

    let row_count = new_reflection(pattern, Orientation::Horizontal, (if old_orientation == Orientation::Horizontal { old_score / 100 } else { old_score } , old_orientation));
    if let Some(score) = row_count {
        if !((score as i64 * 100) == old_score && old_orientation == Orientation::Horizontal) {
            return Some(score as i64 * 100);
        }
    }

    let column_count = new_reflection(pattern, Orientation::Vertical, (if old_orientation == Orientation::Horizontal { old_score } else { old_score } , old_orientation));
    if let Some(score) = column_count {
        if !(score as i64 == old_score && old_orientation == Orientation::Vertical) {
            return Some(score as i64);
        }
    }

    None
}

fn new_reflection(pattern: &Pattern, orientation: Orientation, old_pattern_score: (i64, Orientation)) -> Option<usize> {
    let (old_score, old_orientation) = old_pattern_score;
    let size = match orientation {
        Orientation::Horizontal => pattern.len(),
        Orientation::Vertical => pattern.first().unwrap().len(),
    };

    let reflection = (0..size).step_by(2).fold_while(0, |_, offset| {
        let window_size: usize = size - offset - (size - offset) % 2;
        if window_size < 2 {
            return Done(0);
        }

        let attempt = match orientation {
            Orientation::Horizontal => horizontal(pattern, window_size, old_pattern_score),
            Orientation::Vertical => vertical(pattern, window_size, old_pattern_score),
        };

        if let Some(count) = attempt {
            if !(count as i64 == old_score && orientation == old_orientation) {
                return Done(count);
            }
        }

        Continue(0)
    }).into_inner();

    if reflection > 0 {
        return Some(reflection);
    }

    None
}

fn pattern_score(pattern: &Pattern) -> Option<(i64, Orientation)> {
    let row_count = reflection(pattern, Orientation::Horizontal);
    if let Some(count) = row_count {
        return Some((count as i64 * 100, Orientation::Horizontal));
    }

    let column_count = reflection(pattern, Orientation::Vertical);
    if let Some(count) = column_count {
        return Some((count as i64, Orientation::Vertical));
    }

    None
}

fn reflection(pattern: &Pattern, orientation: Orientation) -> Option<usize> {
    let size = match orientation {
        Orientation::Horizontal => pattern.len(),
        Orientation::Vertical => pattern.first().unwrap().len(),
    };

    let reflection = (0..size).step_by(2).fold_while(0, |_, offset| {
        let window_size: usize = size - offset - (size - offset) % 2;
        if window_size < 2 {
            return Done(0);
        }

        let attempt = match orientation {
            Orientation::Horizontal => horizontal(pattern, window_size, (-1, Orientation::Horizontal)),
            Orientation::Vertical => vertical(pattern, window_size, (-1, Orientation::Vertical)),
        };

        if let Some(count) = attempt {
            return Done(count);
        }

        Continue(0)
    }).into_inner();

    if reflection > 0 {
        return Some(reflection);
    }

    None
}

/**
 * Case 1:
 *  d
 *  c
 *  b i = 2
 *  a
 *  ------------ window_size / 2 + i = 4 / 2 + 2 = 4
 *  a
 *  b j = 5
 * 
 * Case 2:
 *  b i = 0
 *  a
 * ------------ window_size / 2 + i = 4 / 2 + 0 = 2
 *  a
 *  b j = 3
 *  c
 *  d
 */
fn horizontal(pattern: &Pattern, window_size: usize, old_pattern_score: (i64, Orientation)) -> Option<usize> {
    assert!(!pattern.is_empty());
    assert!(window_size > 1);

    let (old_score, old_orientation) = old_pattern_score;

    let height = pattern.len();
    let rows = pattern.iter().cloned().enumerate().collect_vec();
    for window in rows.windows(window_size) {
        //println!("{:?}", window);
        if is_symmetric(window) {
            let (i, _) = window.first().unwrap();
            let (j, _) = window.last().unwrap();
            if (*i == 0 || *j == height - 1) && !(((window_size / 2 + *i) as i64) == old_score && old_orientation == Orientation::Horizontal) {
                return Some(window_size / 2 + *i);
            }
        }
    }
    None
}

fn vertical(pattern: &Pattern, window_size: usize, old_pattern_score: (i64, Orientation)) -> Option<usize> {
    assert!(!pattern.is_empty());
    assert!(window_size > 1);

    let (old_score, old_orientation) = old_pattern_score;

    let width = pattern.first().unwrap().len();
    let columns = (0..width).map(|i| pattern.iter().map(|row| row[i]).collect_vec()).enumerate().collect_vec();
    for window in columns.windows(window_size) {
        if is_symmetric(window) {
            let (i, _) = window.first().unwrap();
            let (j, _) = window.last().unwrap();
            if *i == 0 || *j == width - 1 && !((window_size / 2 + *i) as i64 == old_score && old_orientation == Orientation::Vertical) {
                return Some(window_size / 2 + *i);
            }
        }
    }
    None
}

fn is_symmetric(window: &[(usize, Vec<char>)]) -> bool {
    match window {
        [(_, head), middle @ .., (_, tail)] if head == tail && !middle.is_empty() => is_symmetric(middle),
        [(_, head), (_, tail)] if head == tail => true,
        _ => false
    }
}

fn flip(position: (usize, usize), pattern: &Pattern) -> Pattern {
    let mut new_pattern = pattern.clone();
    let (i, j) = position;
    new_pattern[j][i] = match new_pattern[j][i] {
        '#' => '.',
        '.' => '#',
        _ => {
            eprintln!("Malformed input!");
            '?' /* Yes, I accidentally hit my keyboard at one point without noticing, and created an annoying "bug" for myself to discover later. */
        }
    };
    new_pattern
}

#[allow(dead_code)]
fn pretty_print(pattern: &Pattern) {
    pattern.iter().for_each(|row| println!("{:?}", row));
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
        let sum = solution("./test_input.txt", false);
        assert_eq!(sum, 405);
    }

    #[test]
    fn test_input_smudged() {
        let sum = solution("./test_input.txt", true);
        assert_eq!(sum, 400);
    }

    #[test]
    fn test_bug_1_input() {
        let sum = solution("./test_bug_1_input.txt", true);
        assert_eq!(sum, 4);
    }

    #[test]
    fn test_bug_2_input() {
        let sum = solution("./test_bug_2_input.txt", true);
        assert_eq!(sum, 900);
    }
}