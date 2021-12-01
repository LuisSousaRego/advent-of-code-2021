use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "puzzleInputs/day1.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut counter = 0;
    let mut prev_depth = i32::MAX;
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let depth: i32 = line.parse().unwrap();

        if depth > prev_depth {
            counter += 1
        }
        prev_depth = depth
    }
    println!("{}", counter);
}
