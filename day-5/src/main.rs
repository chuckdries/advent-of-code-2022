use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader};

struct Token {
    column: usize,
    letter: char,
}

fn tokenize_line(line: &str, re: &Regex) -> Option<Vec<Token>> {
    Some(Vec::new())
}

// for parsing - the the column of the value is (i - 2) / 4
fn part_one<'a>(reader: &mut BufReader<File>) -> isize {
    let re = Regex::new(r"[A-Z]").unwrap();
    let mut lines = reader.lines();

    let mut stacks: Vec<Vec<&'a char>> = Vec::new();

    for wrapped in lines {
        let line = wrapped.unwrap();
        match tokenize_line(&line, &re) {
            Some(tokens) => {
                for token in tokens {
                    let maybe_stack = stacks.get(token.column);
                    match maybe_stack {
                        Some(&mut stack) => stack.push(token.letter),
                        None => {
                            for i in stacks.len()..token.column {
                                let mut nv: Vec<&'a char> = Vec::new();
                                stacks.push(nv)
                            }
                        }
                    }
                }
            }
            None => {
                break;
            }
        }
    }

    32 as isize
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut iter = reader.lines();

    for wrapped in &mut iter {
        let line = wrapped.unwrap();
        if line == "" {
            println!("found empty line");
            break;
        }
    }

    for wrapped in &mut iter {
        let line = wrapped.unwrap();
        println!("{line}")
    }

    // let p1 = part_one(&mut reader);
    // println!("p1 answer {p1}");
}
