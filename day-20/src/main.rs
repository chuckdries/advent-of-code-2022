use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
    time::Instant,
};

fn main() {
    let file = File::open("sample.txt").unwrap();
    let mut reader = BufReader::new(file);

    let p1 = part_one(&mut reader);
    println!("part one is {p1}");
}

fn part_one(reader: &mut BufReader<File>) -> isize {
    let mut original: Vec<isize> = reader.lines().map(|line| line.unwrap().parse().unwrap()).collect();
    let mut iter_count = 0;
    let mut cursor = 0;
    let len = original.len();
    while iter_count < len {
        let item = original.get(cursor).unwrap();
        if *item == 0 {
            continue;
        }
        let mut new_index = cursor as isize + *item;
        dbg!(item);
        if new_index < 0 {
            new_index = len as isize + new_index;
        }
        if new_index > len as isize {
            new_index = new_index - len as isize;
        }
        if new_index <= cursor as isize{
            cursor += 1;
        }
        original.remove(cursor);
        original.insert(new_index as usize, *item);

        iter_count += 1;

    }
    dbg!(original);
    32
}
