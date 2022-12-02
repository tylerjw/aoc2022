use std::env;
use std::fs;

enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Lose,
    Tie,
    Win,
}

struct Game {
    elf: Shape,
    us: Shape,
}

impl Shape {
    fn new(note: &str) -> Shape {
        match note {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => panic!("Unexpected character: {}", note),
        }
    }

    fn score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn from(elf: &Shape, goal: &Outcome) -> Shape {
        match elf {
            Shape::Rock => match goal {
                Outcome::Win => Shape::Paper,
                Outcome::Tie => Shape::Rock,
                Outcome::Lose => Shape::Scissors,
            },
            Shape::Paper => match goal {
                Outcome::Win => Shape::Scissors,
                Outcome::Tie => Shape::Paper,
                Outcome::Lose => Shape::Rock,
            },
            Shape::Scissors => match goal {
                Outcome::Win => Shape::Rock,
                Outcome::Tie => Shape::Scissors,
                Outcome::Lose => Shape::Paper,
            },
        }
    }
}

impl Outcome {
    fn from_game(game: &Game) -> Outcome {
        match game.elf {
            Shape::Rock => match game.us {
                Shape::Rock => Outcome::Tie,
                Shape::Paper => Outcome::Win,
                Shape::Scissors => Outcome::Lose,
            },
            Shape::Paper => match game.us {
                Shape::Rock => Outcome::Lose,
                Shape::Paper => Outcome::Tie,
                Shape::Scissors => Outcome::Win,
            },
            Shape::Scissors => match game.us {
                Shape::Rock => Outcome::Win,
                Shape::Paper => Outcome::Lose,
                Shape::Scissors => Outcome::Tie,
            },
        }
    }

    fn from_note(note: &str) -> Outcome {
        match note {
            "X" => Outcome::Lose,
            "Y" => Outcome::Tie,
            "Z" => Outcome::Win,
            _ => panic!("Unexpected character: {}", note),
        }
    }

    fn score(&self) -> i32 {
        match self {
            Outcome::Win => 6,
            Outcome::Tie => 3,
            Outcome::Lose => 0,
        }
    }
}

fn score(game: &Game) -> i32 {
    Outcome::from_game(game).score() + game.us.score()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    dbg!(file_path);

    let data = fs::read_to_string(file_path).unwrap();

    let first_total_score: i32 = data
        .split('\n')
        .map(|line| {
            let players: Vec<_> = line.split(' ').collect();
            score(&Game {
                elf: Shape::new(players[0]),
                us: Shape::new(players[1]),
            })
        })
        .sum();

    println!("first total score: {}", first_total_score);

    let real_total_score: i32 = data
        .split('\n')
        .map(|line| {
            let players: Vec<_> = line.split(' ').collect();
            let elf = Shape::new(players[0]);
            let goal = Outcome::from_note(players[1]);
            let us = Shape::from(&elf, &goal);
            score(&Game { elf, us })
        })
        .sum();
    println!("real total score: {}", real_total_score);
}
