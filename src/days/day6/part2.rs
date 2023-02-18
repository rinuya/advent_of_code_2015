use std::fs::File;
use std::io::{ self, BufRead };
use std::time::Instant;


pub fn run() {
    let start = Instant::now();

    let file = File::open("src/days/day6/input.txt").unwrap(); 
    let lines = io::BufReader::new(file).lines(); 
    let mut result:i64 = 0;
    let mut grid = vec![vec![0; 1000]; 1000];

    for line in lines {
        let _line = line.unwrap();
        if _line.is_empty() {
            break;
        }

        let _line = _line.split(" ").collect::<Vec<_>>();
        match _line[1] {
            "on" => {
                let start = _line[2].split(",").map(|word| word.parse().unwrap()).collect::<Vec<i32>>();
                let end = _line[4].split(",").map(|word| word.parse().unwrap()).collect::<Vec<i32>>();

                for i in start[0]..end[0]+1 {
                    for j in start[1]..end[1]+1 {             
                        grid[i as usize][j as usize] += 1;
                    }
                }
            },
            "off" => {
                let start = _line[2].split(",").map(|word| word.parse().unwrap()).collect::<Vec<i32>>();
                let end = _line[4].split(",").map(|word| word.parse().unwrap()).collect::<Vec<i32>>();

                for i in start[0]..end[0]+1 {
                    for j in start[1]..end[1]+1 {
                        if grid[i as usize][j as usize] > 0 {
                            grid[i as usize][j as usize] -= 1;
                        }    
                    }
                }
            },
            _ => {
                let start = _line[1].split(",").map(|word| word.parse().unwrap()).collect::<Vec<i32>>();
                let end = _line[3].split(",").map(|word| word.parse().unwrap()).collect::<Vec<i32>>();

                for i in start[0]..end[0]+1 {
                    for j in start[1]..end[1]+1 {  
                        grid[i as usize][j as usize] += 2;
                    }
                }
            },
        }
    }
    for i in 0..1000{
        for j in 0..1000 {
            result += grid[i][j];
        }
    }

    let elapsed = start.elapsed();
    println!("Day 6 Part 2: The solution '{:?}' was found in {:.2?}", result, elapsed);
}