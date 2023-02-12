use std::time::Instant;

pub fn run() {
    let start = Instant::now();

    let result = "MD5"; //will deal with this at a later point

    let elapsed = start.elapsed();
    println!("Day 4 Part 1: The solution '{:?}' was found in {:.2?}", result, elapsed);
}