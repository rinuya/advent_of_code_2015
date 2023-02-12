use std::fs::File;
use std::io::{ self, BufRead };
use std::time::Instant;

pub fn run() {  
    let start = Instant::now();

    let file = File::open("src/days/day2/input.txt").unwrap(); 
    let lines = io::BufReader::new(file).lines(); 
    let mut result:u64 = 0;

    for line in lines {
        let _line = line.unwrap();

        if _line.is_empty() {
            break;
        }

        let mut dim: Vec<u64> = _line.split("x").map(|x| x.parse::<u64>().unwrap()).collect();
        result += dim[0] * dim[1] * dim[2]; // w*l*h

        let max = dim.iter().max().unwrap(); //find max val
        dim.remove(dim.iter().position(|x| *x == *max).unwrap()); //remove max val by getting index w .position()
        result += 2*dim[0] + 2*dim[1];
    }

    let elapsed = start.elapsed();
    println!("Day 2 Part 2: The solution '{}' was found in {:.2?}", result, elapsed);
}