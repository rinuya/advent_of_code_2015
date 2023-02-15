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
        if _line.contains("ab") || _line.contains("cd") || _line.contains("pq") || _line.contains("xy") {
            continue;       
        }

        let mut vowel_counter:u32 = 0;
        let mut double_letter = false;
        let char_vec: Vec<char> = _line.chars().collect();
        for i in 0..char_vec.len() {
            if char_vec[i] == 'a' || char_vec[i] == 'e' || char_vec[i] == 'i' || char_vec[i] == 'o' || char_vec[i] == 'u' {
                vowel_counter += 1;
            }
            if i != 0 {
                if char_vec[i] == char_vec[i-1] {
                    double_letter = true;
                }
            }
        }

        if vowel_counter >= 3 && double_letter == true {
            result += 1;
        }

    }

    let elapsed = start.elapsed();
    println!("Day 5 Part 1: The solution '{:?}' was found in {:.2?}", result, elapsed);
}