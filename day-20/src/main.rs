use std::{
    // collections::HashSet,
    fs::File,
    io::{
        BufRead,
        BufReader,
        // Seek
    },
    // time::Instant,
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let p1 = part_one(&mut reader);
    println!("part one is {p1}");

    // reader.rewind().unwrap();

    // let mut s: HashSet<isize> = HashSet::new();
    // for i in reader.lines() {
    //     let val: isize = i.unwrap().parse().unwrap();
    //     if s.contains(&val) {
    //         println!("found dupe {val}");
    //     }
    //     s.insert(val);
    // }
}

fn debug_vec(vector: &Vec<isize>) {
    for i in vector {
        print!("{i}, ");
    }
    println!("");
}

fn part_one(reader: &mut BufReader<File>) -> isize {
    let mut original: Vec<isize> = reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    let mut iter_count = 0;
    let mut cursor = 0;
    let len = original.len();
    while iter_count < len {
        debug_vec(&original);
        dbg!(cursor);
        let item = original[cursor];
        dbg!(item);
        if item == 0 {
            continue;
        }
        let mut new_index = cursor as isize + 1 + item;
        if new_index < 0 {
            new_index = len as isize + new_index;
        }
        if new_index > len as isize {
            new_index = new_index - len as isize;
        }
        dbg!(new_index);
        original.remove(cursor);
        original.insert((new_index - 1) as usize, item);

        if new_index <= (cursor as isize + 1) {
            cursor += 1;
        }
        iter_count += 1;
    }
    dbg!(original);
    32
}
