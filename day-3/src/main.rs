use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn get_reader() -> BufReader<File> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    reader
}

// map from rust char u8 values to problem values
// a-z: 97-122 -> 1-16
// A-Z: 65-90 -> 27-52
fn get_char_priority(char: char) -> u8 {
    let num = char as u8;
    match num {
        97..=122 => num - 96,
        65..=90 => num - 64 + 26,
        _ => {
            panic!("invalid char value")
        }
    }
}

fn part_one(reader: BufReader<File>) {
    let mut sum: usize = 0;
    for wrapped in reader.lines() {
        let line = wrapped.unwrap();
        let mut first_compartment: HashSet<char> = HashSet::new();
        let chars: Vec<char> = line.chars().collect();
        for i in 0..chars.len() / 2 {
            let char = chars[i];
            first_compartment.insert(char);
        }

        for i in (chars.len() / 2)..chars.len() {
            let char = chars[i];
            if first_compartment.contains(&char) {
                sum += get_char_priority(char) as usize;
                break;
            }
        }
    }
    println!("sum: {sum}")
}

fn main() {
    part_one(get_reader());
}
