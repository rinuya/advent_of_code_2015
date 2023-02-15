use std::fs::File;
use std::io::{ self, BufRead };
use std::time::Instant;

pub fn run() {
    let start = Instant::now();

    let file = File::open("src/days/day5/input.txt").unwrap(); 
    let lines = io::BufReader::new(file).lines(); 
    let mut result:u64 = 0;

    for line in lines {
        let _line = line.unwrap();
        
        if _line.is_empty() {
            break;
        }

        let mut double_pair = false;
        let mut double_w_space = false;
        let char_vec: Vec<char> = _line.chars().collect();

        for i in 0..char_vec.len() {
            if i > 1 {
                if char_vec[i] == char_vec[i-2] {
                    double_w_space = true;
                }
            }
            if i != 0 {
                for j in 0..char_vec.len() {
                    if j == 0 ||  i == j || i == j-1 || i == j+1 {
                        continue;
                    } else {
                        if char_vec[i] == char_vec[j] && char_vec[i-1] == char_vec[j-1] {
                            double_pair = true;
                        }
                    }
                }
            }
        }
        if double_pair == true && double_w_space == true {
            result += 1;
        }
    }

    let elapsed = start.elapsed();
    println!("Day 5 Part 2: The solution '{:?}' was found in {:.2?}", result, elapsed);
}