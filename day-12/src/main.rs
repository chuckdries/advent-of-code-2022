use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
};

use grid::Grid;
use pathfinding::prelude::dijkstra;

type Coord = (usize, usize);

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);

    assert_eq!(true, valid_step('a', 'b'));
    assert_eq!(false, valid_step('a', 'c'));
    assert_eq!(true, valid_step('S', 'b'));
    assert_eq!(true, valid_step('E', 'y'));

    let one_result = part_one(&mut reader);
    println!("part one: {one_result}");
    reader.rewind().unwrap();
}

fn valid_step(_curr: char, _next: char) -> bool {
    let curr = match _curr {
        'S' => 'a',
        'E' => 'z',
        _ => _curr,
    };

    let next = match _next {
        'S' => 'a',
        'E' => 'z',
        _ => _next,
    };

    let distance: isize = curr as isize - next as isize;
    distance.abs() <= 1
}

fn get_successors(grid: &mut Grid<char>, pos: Coord) -> Vec<Coord> {
    let mut successors: Vec<Coord> = Vec::new();
    let (width, height) = grid.size();
    let char = grid.get(pos.0, pos.1).unwrap();
    // right
    if pos.0 + 1 < width {
        let right_step = (pos.0 + 1, pos.1);
        let next_char = grid.get(right_step.0, right_step.1).unwrap();
        if valid_step(*char, *next_char) {
            successors.push(right_step)
        }
    }
    // down
    if pos.1 + 1 < height {
        let down_step = (pos.0, pos.1 + 1);
        let next_char = grid.get(down_step.0, down_step.1).unwrap();
        if valid_step(*char, *next_char) {
            successors.push(down_step);
        }
    }
    // left
    if pos.0 > 0 {
        let left_step = (pos.0 - 1, pos.1);
        let next_char = grid.get(left_step.0, left_step.1).unwrap();
        if valid_step(*char, *next_char) {
            successors.push(left_step);
        }
    }
    // up
    if pos.1 > 0 {
        let up_step = (pos.0, pos.1 - 1);
        let next_char = grid.get(up_step.0, up_step.1).unwrap();
        if valid_step(*char, *next_char) {
            successors.push(up_step);
        }
    }
    successors
}

fn part_one(reader: &mut BufReader<File>) -> usize {
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

    let result = dijkstra(
        &pos_start,
        |pos| get_successors(&mut grid, *pos).into_iter().map(|p| (p, 1)),
        |p| *p == pos_end,
    );
    result.expect("couldn't find path to end").1
}
