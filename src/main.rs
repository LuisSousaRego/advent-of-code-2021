use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1() {
    let filename = "puzzleInputs/day1.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut counter = 0;
    let mut prev_depth = i32::MAX;
    for line in reader.lines() {
        let line = line.unwrap();
        let depth: i32 = line.parse().unwrap();

        if depth > prev_depth {
            counter += 1
        }
        prev_depth = depth
    }
    println!("part 1: {}", counter);
}

fn part2() {
    let filename = "puzzleInputs/day1.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut depth_arr: [i32; 2000] = [0; 2000];
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let depth: i32 = line.parse().unwrap();

        depth_arr[i] = depth
    }

    let mut counter = 0;

    for i in 0..(2000 - 3) {
        let first_three = depth_arr[i] + depth_arr[i + 1] + depth_arr[i + 2];
        let second_three = depth_arr[i + 1] + depth_arr[i + 2] + depth_arr[i + 3];

        if second_three > first_three {
            counter += 1;
        }
    }
    println!("part 2: {}", counter);
}

fn main() {
    part1();
    part2();
}
