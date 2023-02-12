use std::fs::File;
use std::io::Read;
use std::time::Instant;

pub fn run() {
    let start = Instant::now();

    let mut string_input = String::new();
    File::open("src/days/day1/input.txt").unwrap().read_to_string(&mut string_input).unwrap();

    let mut counter = 0;
    for c in string_input.chars() {
        match c {
            '(' => counter += 1,
            ')' => counter -= 1,
            _ => continue,
        }
    }

    let elapsed = start.elapsed();
    println!("Day 1 Part 1: The solution '{}' was found in {:.2?}", counter, elapsed);
}