/*
opp self
A   X   ROCK
B   Y   PAPER
C   Z   SCISSORS
*/
use std::fs::File;
use std::io::{prelude::*, BufReader};

enum Move {
    Rock,
    Paper,
    Scissors,
}

enum MatchResult {
    Win,
    Loss,
    Tie,
}

fn compare_moves(self_move: &Move, opponent_move: &Move) -> MatchResult {
    match self_move {
        Move::Rock => match opponent_move {
            Move::Scissors => MatchResult::Win,
            Move::Rock => MatchResult::Tie,
            Move::Paper => MatchResult::Loss,
        },
        Move::Paper => match opponent_move {
            Move::Rock => MatchResult::Win,
            Move::Paper => MatchResult::Tie,
            Move::Scissors => MatchResult::Loss,
        },
        Move::Scissors => match opponent_move {
            Move::Paper => MatchResult::Win,
            Move::Scissors => MatchResult::Tie,
            Move::Rock => MatchResult::Loss,
        },
    }
}

fn part_one(reader: BufReader<File>) {
    let mut score: usize = 0;
    for wrapped in reader.lines() {
        let line = wrapped.unwrap();
        let mut split = line.split_ascii_whitespace();

        let opponent_move = match split.next().unwrap() {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => {
                panic!("invalid move for opponent")
            }
        };

        let self_move = match split.next().unwrap() {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => {
                panic!("invalid move for self")
            }
        };

        score += match self_move {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        };

        match compare_moves(&self_move, &opponent_move) {
            MatchResult::Win => score += 6,
            MatchResult::Tie => score += 3,
            MatchResult::Loss => score += 0,
        }
    }
    println!("{score}");
}

fn get_reader() -> BufReader<File> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    reader
}

fn main() {
    part_one(get_reader());
}
