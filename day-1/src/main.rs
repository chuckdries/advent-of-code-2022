use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug)]
struct Elf {
    calories: usize,
    index: usize,
}

fn main() {
    let mut elves: Vec<Elf> = Vec::new();

    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut highest_calories = 0;
    let mut calories = 0;
    for (i, line_wrapped) in reader.lines().enumerate() {
        let line = line_wrapped.unwrap();
        if line == "" {
            elves.push(Elf { calories, index: i });
            if calories > highest_calories {
                highest_calories = calories;
            }
            calories = 0;
        } else {
            calories += line.parse::<usize>().unwrap();
        }
    }

    println!("highest calories: {}", highest_calories);

    elves.sort_by_key(|e| e.calories);


    let max = elves.pop().unwrap().calories;
    let second = elves.pop().unwrap().calories;
    let third = elves.pop().unwrap().calories;

    println!("sums {max} {second} {third}");
    
    let top_three = max + second + third;

    println!("top three calories: {}", top_three);

}
