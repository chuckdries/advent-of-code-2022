use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
};

struct Range {
    high: usize,
    low: usize,
}

impl Range {
    pub fn new(range_str: &str) -> Range {
        let mut split = range_str.split("-");
        let low: usize = split.next().unwrap().parse().unwrap();
        let high: usize = split.next().unwrap().parse().unwrap();
        Range {
            low,
            high
        }
    }
}

fn elves_overlap_entirely(one: &Range, two: &Range) -> bool {
    (one.high >= two.high && one.low <= two.low) || (two.high >= one.high && two.low <= one.low)
}

fn part_one(reader: &mut BufReader<File>) -> usize {
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
    println!("overlapping groups part 1 {overlapping_groups}");
    overlapping_groups
}

fn elves_overlap_at_all(one: &Range, two: &Range) -> bool {
    (one.high >= two.low && one.high <= two.high) ||
    (one.low >= two.low && one.low <= two.high) ||
    (two.high >= one.low && two.high <= one.high) ||
    (two.low >= one.low && two.low <= one.high)
}

fn part_two(reader: &mut BufReader<File>) -> usize{
    let mut overlapping_groups: usize = 0;
    for wrapped in reader.lines() {
        let line = wrapped.unwrap();
        let mut split = line.split(",");

        let elf_one = Range::new(split.next().unwrap());
        let elf_two = Range::new(split.next().unwrap());

        if elves_overlap_at_all(&elf_one, &elf_two) {
            overlapping_groups += 1;
        }
    }
    println!("overlapping groups part 2 {overlapping_groups}");
    overlapping_groups
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let p1 = part_one(&mut reader);
    reader.rewind().unwrap();

    let p2 = part_two(&mut reader);

    // check for correct answer
    assert_eq!(p1, 305);
    assert_eq!(p2, 811);
}