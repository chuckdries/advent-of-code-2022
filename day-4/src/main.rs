use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Range {
    high: usize,
    low: usize,
}

impl Range {
    pub fn new(range_str: &str) -> Range {
        let mut split = range_str.split("-");
        let low: usize = split.next().unwrap().parse().expect("input numbers must be integers");
        let high: usize = split.next().unwrap().parse().expect("input numbers must be integers");
        Range {
            low,
            high
        }
    }
}

fn elves_overlap_entirely(one: &Range, two: &Range) -> bool {
    (one.high >= two.high && one.low <= two.low) || (two.high >= one.high && two.low <= one.low)
}

fn part_one(reader: &mut BufReader<File>) {
    let mut overlapping_groups: usize = 0;
    for wrapped in reader.lines() {
        let line = wrapped.unwrap();
        let mut split = line.split(",");
        let elf_one = Range::new(split.next().unwrap());
        let elf_two = Range::new(split.next().unwrap());
        if elves_overlap_entirely(&elf_one, &elf_two) {
            overlapping_groups += 1;
        }
    }
    println!("overlapping groups {overlapping_groups}");
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);

    part_one(&mut reader);
}
