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

fn part_two(reader: &mut BufReader<File>) {
    let mut sum: usize = 0;
    let mut elf_index: usize = 0;
    let mut first_elf: HashSet<char> = HashSet::new();
    let mut second_elf: HashSet<char> = HashSet::new();
    for wrapped in reader.lines() {
        let line = wrapped.unwrap();
        print!("{elf_index} - ");
        for char in line.chars() {
            if elf_index == 0 {
                print!("{char}");
                first_elf.insert(char);
            } else if elf_index == 1 {
                if first_elf.contains(&char) {
                    print!("{char}");
                    second_elf.insert(char);
                }
            } else {
                if second_elf.contains(&char) {
                    sum += get_char_priority(char) as usize;
                    println!("badge is {char}");
                    break;
                }
            }
        }
        println!("");
        if elf_index < 2 {
            elf_index += 1;
        } else {
            // reset index and duplication sets
            elf_index = 0;
            first_elf = HashSet::new();
            second_elf = HashSet::new();
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
