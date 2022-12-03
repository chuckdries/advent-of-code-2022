use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Seek},
};

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

fn part_one(reader: &mut BufReader<File>) {
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
    println!("part one sum: {sum}")
}

enum Elf {
    One,
    Two,
    Three
}

fn part_two(reader: &mut BufReader<File>) {
    let mut sum: usize = 0;
    let mut elf_index = Elf::One;
    let mut first_elf: HashSet<char> = HashSet::new();
    let mut second_elf: HashSet<char> = HashSet::new();
    let mut third_elf: HashSet<char> = HashSet::new();
    for wrapped in reader.lines() {
        let line = wrapped.unwrap();
        match elf_index {
            Elf::One => {
                first_elf = line.chars().collect();
                elf_index = Elf::Two;
            },
            Elf::Two => {
                second_elf = line.chars().collect();
                elf_index = Elf::Three;
            },
            Elf::Three => {
                third_elf = line.chars().collect();

                let first_two_set: HashSet<&char> = HashSet::from(first_elf.intersection(&second_elf).collect::<HashSet<&char>>());
                third_elf.retain(|e| first_two_set.contains(e));
                let badge = third_elf.iter().next().unwrap();

                sum += get_char_priority(*badge) as usize;
                elf_index = Elf::One;
            }
        }
    }
    println!("part two sum: {sum}")
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);

    part_two(&mut reader);
    reader.rewind().unwrap();
    part_one(&mut reader);
}
