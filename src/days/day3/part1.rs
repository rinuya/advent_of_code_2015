use std::fs::File;
use std::io::Read;
use std::time::Instant;

pub fn run() {
    let start = Instant::now();

    let mut string_input = String::new();
    File::open("src/days/day3/input.txt").unwrap().read_to_string(&mut string_input).unwrap();

    //first step: figure out viable dimensions of grid
    let mut max_dim = [0, 0, 0, 0];
    for c in string_input.chars() {
        match c {
            '<' => max_dim[0] += 1,
            '>' => max_dim[1] += 1,
            '^' => max_dim[2] += 1,
            'v' => max_dim[3] += 1,
            _ => continue,
        }
    }
    //generate grid, populated w false values
    let mut grid: Vec<Vec<bool>> = vec![];
    for _ in 0..max_dim[2]+max_dim[3] {
        let mut new_col: Vec<bool> = vec![];
        for _ in 0..max_dim[0]+max_dim[1] {
            new_col.push(false);
        }
        grid.push(new_col);
    }

    let mut counter = 1;
    let mut pos = [max_dim[0],  max_dim[2]];
    for c in string_input.chars() {
        match c {
            '<' => pos[1] -= 1,
            '>' => pos[1] += 1,
            '^' => pos[0] -= 1,
            'v' => pos[0] += 1,
            _ => continue,
        }
        if grid[pos[0]][pos[1]] == false {
            grid[pos[0]][pos[1]] = true;
            counter += 1;
        }
    }

    let elapsed = start.elapsed();
    println!("Day 3 Part 1: The solution '{}' was found in {:.2?}", counter, elapsed);
}