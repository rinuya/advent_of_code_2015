use std::fs::File;
use std::io::Read;
use std::time::Instant;

pub fn run() {
    let start = Instant::now();
    
    let mut string_input = String::new();
    File::open("src/days/day1/input.txt").unwrap().read_to_string(&mut string_input).unwrap();

    let mut floor = 0;
    let mut counter = 0;
    
    for (i, c) in string_input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => continue,
        }
        if floor == -1 {
            counter = i+1;
            break;
        }
    }
    
    let elapsed = start.elapsed();
    println!("Day 1 Part 1: The solution '{}' was found in {:.2?}", counter, elapsed);
}