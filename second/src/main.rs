use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;

#[derive(Debug)]
struct Game {
    red: u32,
    blue: u32,
    green: u32,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(r: {}, g: {}, b: {})", self.red, self.blue, self.green)
    }
}

const CONFIGURATION: Game = Game { red: 12, green: 13, blue: 14 };
const ZERO: Game = Game { red: 0, blue: 0, green: 0 };

fn main() {
    //println!("{}", validate_game("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"));
    solution();
}

fn solution() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut sum: u32 = 0;

        for line in lines {
            if let Ok(values) = line {
                //println!("{}", validate_game(&values));
                //sum += validate_game(&values);
                sum += powers(&values);
            }
        }

        println!("{}", sum);
    }
}

fn validate_game(line: &str) -> u32 {
    let id = get_id(line);
    let game = get_game(line);
    let max_game = game.iter().fold(ZERO, |a, b| Game { red: cmp::max(a.red, b.red), green: cmp::max(a.green, b.green), blue: cmp::max(a.blue, b.blue) });

    if max_game.red <= CONFIGURATION.red && max_game.green <= CONFIGURATION.green && max_game.blue <= CONFIGURATION.blue {
        return id;
    }
    0
}

fn powers(line: &str) -> u32 {
    let game = get_game(line);
    let max_game = game.iter().fold(ZERO, |a, b| Game { red: cmp::max(a.red, b.red), green: cmp::max(a.green, b.green), blue: cmp::max(a.blue, b.blue) });

    return max_game.red * max_game.blue * max_game.green;
}

fn get_id(line: &str) -> u32 {
    match line.split(": ").filter(|part| part.starts_with("Game")).last() {
        Some(game) => game.split(" ").last().unwrap().parse::<u32>().unwrap(),
        None => 0
    }
}

fn get_game(line: &str) -> Vec<Game> {
    line.split(": ").last().unwrap().split("; ").map(|game_string| get_draw(game_string)).collect()
}

fn get_draw(game_string: &str) -> Game {
    let no_of_red = get_no_of_colour("red", game_string);
    let no_of_green = get_no_of_colour("green", game_string);
    let no_of_blue = get_no_of_colour("blue", game_string);

    Game {
        red: no_of_red,
        green: no_of_green,
        blue: no_of_blue
    }
}

fn get_no_of_colour(colour: &str, game_string: &str) -> u32 {
    let subsets = game_string.split(", ");
    match subsets.filter(|entry| entry.ends_with(colour)).last() {
        Some(colours) => colours.split(" ").take(1).next().unwrap().parse::<u32>().unwrap(),
        None => 0
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}