use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
    time::Instant,
};

use grid::Grid;
use pathfinding::prelude::{dijkstra};

type Coord = (usize, usize);

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);

    assert_eq!(true, valid_step('a', 'b'));
    assert_eq!(false, valid_step('a', 'c'));
    assert_eq!(true, valid_step('c', 'a'));
    assert_eq!(true, valid_step('S', 'b'));
    assert_eq!(true, valid_step('E', 'y'));

    let now = Instant::now();
    let one_result = part_one(&mut reader);
    let elapsed = now.elapsed();
    println!("part one: {one_result} (took {:.2?})", elapsed);

    reader.rewind().unwrap();
    let now = Instant::now();
    let two_result = part_two(&mut reader);
    let elapsed = now.elapsed();
    println!("part two: {two_result} (took {:.2?})", elapsed);
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

    let distance = next as isize - curr as isize;
    distance <= 1
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

    let result = dijkstra(
        &pos_start,
        |pos| get_successors(&mut grid, *pos).into_iter().map(|p| (p, 1)),
        |p| *p == pos_end,
    );
    result.expect("couldn't find path to end").1
}

fn part_two(reader: &mut BufReader<File>) -> usize {
    let mut grid: Grid<char> = Grid::new(0, 0);
    let mut starting_coords: Vec<Coord> = Vec::new();
    let mut pos_end: Coord = (0, 0);
    for (i, wrapped) in reader.lines().enumerate() {
        let line = wrapped.unwrap();
        let chars = line.chars();
        let mut row: Vec<char> = Vec::new();
        for (j, char) in chars.enumerate() {
            if char == 'S' || char == 'a' {
                starting_coords.push((i, j));
            }
            if char == 'E' {
                pos_end = (i, j);
            }
            row.push(char);
        }
        grid.push_row(row);
    }

    let result = starting_coords
        .into_iter()
        .filter_map(|coord| {
            dijkstra(
                &coord,
                |pos| get_successors(&mut grid, *pos).into_iter().map(|p| (p, 1)),
                |p| *p == pos_end,
            )
        })
        .map(|q| q.1)
        .min();
    result.unwrap()
}
