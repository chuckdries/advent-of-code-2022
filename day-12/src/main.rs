use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Seek},
};

use grid::Grid;

type Coord = (usize, usize);

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let one_result = part_one(&mut reader);
    println!("one: {one_result}");
    assert_eq!(true, valid_step('a', 'b'));
    assert_eq!(false, valid_step('a', 'c'));
}

fn valid_step(_prev: char, _current: char) -> bool {
    let current = match _current {
        'S' => 'a',
        'E' => 'z',
        _ => _current,
    };

    let prev = match _prev {
        'S' => 'a',
        'E' => 'z',
        _ => _prev,
    };

    let distance: isize = prev as isize - current as isize;
    distance.abs() <= 1
}

// @returns bool - whether or not it found the end
fn step(
    grid: &Grid<char>,
    visited: &mut HashSet<Coord>,
    target: Coord,
    pos: Coord,
    previous_char: &char,
) -> isize {
    let char = grid.get(pos.0, pos.1).unwrap();
    if !valid_step(*previous_char, *char) {
        return 0;
    }

    visited.insert(pos);

    if pos == target {
        return 1;
    }

    let (width, height) = grid.size();
    // right
    let right_coord = (pos.0 + 1, pos.1);
    let right = if right_coord.0 < width && !visited.contains(&right_coord) {
        step(grid, visited, target, right_coord, char)
    } else {
        0
    };
    let down_coord = (pos.0, pos.1 + 1);
    let down = if down_coord.1 < height && !visited.contains(&down_coord) {
        step(grid, visited, target, down_coord, char)
    } else {
        0
    };
    let left = if pos.0 as isize - 1 > 0 && !visited.contains(&(pos.0 - 1, pos.1)) {
        step(grid, visited, target, (pos.0 - 1, pos.1), char)
    } else {
        0
    };

    let up = if pos.1 as isize - 1 > 0 && !visited.contains(&(pos.0, pos.1 - 1)) {
        step(grid, visited, target, (pos.0, pos.1 - 1), char)
    } else {
        0
    };
    (right + down + left + up) + (right * down * left * up * 1)
}

fn part_one(reader: &mut BufReader<File>) -> isize {
    let mut grid: Grid<char> = Grid::new(0, 0);
    let mut pos_start: Coord = (0, 0);
    let mut pos_end: Coord = (0, 0);
    for (i, wrapped) in reader.lines().enumerate() {
        let line = wrapped.unwrap();
        let chars = line.chars();
        let mut row: Vec<char> = Vec::new();
        for (j, char) in chars.enumerate() {
            if char == 'S' {
                pos_start = (i, j);
            }
            if char == 'E' {
                pos_end = (i, j);
            }
            row.push(char);
        }
        grid.push_row(row);
    }
    println!("start at {:?}, end at {:?}", pos_start, pos_end);

    let mut visited: HashSet<Coord> = HashSet::new();
    step(&grid, &mut visited, pos_end, pos_start, &'S')
}
