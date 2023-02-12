use std::fs::File;
use std::io::Read;
use std::time::Instant;

pub fn run() {
    let start = Instant::now();

    let mut string_input = String::new();
    File::open("src/days/day3/input.txt").unwrap().read_to_string(&mut string_input).unwrap();

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

    let mut grid: Vec<Vec<bool>> = vec![];
    for _ in 0..max_dim[2]+max_dim[3] {
        let mut new_col: Vec<bool> = vec![];
        for _ in 0..max_dim[0]+max_dim[1] {
            new_col.push(false);
        }
        grid.push(new_col);
    }

    let mut counter = 1;
    grid[max_dim[2]][max_dim[0]] = true;
    let mut is_santas_turn = true;
    let mut pos_santa = [max_dim[2], max_dim[0]];
    let mut pos_robo = [max_dim[2], max_dim[0]];

    for c in string_input.chars() {
        if is_santas_turn == true {
            match c {
                '<' => pos_santa[1] -= 1,
                '>' => pos_santa[1] += 1,
                '^' => pos_santa[0] -= 1,
                'v' => pos_santa[0] += 1,
                _ => continue,
            }
            if grid[pos_santa[0]][pos_santa[1]] == false {
                grid[pos_santa[0]][pos_santa[1]] = true;
                counter += 1;
            }
            is_santas_turn = false;

        } else {
            match c {
                '<' => pos_robo[1] -= 1,
                '>' => pos_robo[1] += 1,
                '^' => pos_robo[0] -= 1,
                'v' => pos_robo[0] += 1,
                _ => continue,
            }
            if grid[pos_robo[0]][pos_robo[1]] == false {
                grid[pos_robo[0]][pos_robo[1]] = true;
                counter += 1;
            }
            is_santas_turn = true;
        }
    }

    let elapsed = start.elapsed();
    println!("Day 3 Part 2: The solution '{}' was found in {:.2?}", counter, elapsed);
}