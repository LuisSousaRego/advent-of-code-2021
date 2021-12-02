use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1() -> i32 {
    let filename = "puzzleInputs/day2.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut h = 0;
    let mut d = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        let cmd: Vec<&str> = line.split_whitespace().collect();
        let instruction = cmd[0];
        let amount = cmd[1].parse::<i32>().unwrap();
        match instruction {
            "forward" => h += amount,
            "down" => d += amount,
            "up" => d -= amount,
            _ => panic!(),
        }
    }
    d * h
}

pub fn part2() -> i32 {
    let filename = "puzzleInputs/day2.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut h = 0;
    let mut d = 0;
    let mut a = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        let cmd: Vec<&str> = line.split_whitespace().collect();
        let instruction = cmd[0];
        let amount = cmd[1].parse::<i32>().unwrap();

        match instruction {
            "forward" => {
                h += amount;
                d += a * amount;
            }
            "down" => a += amount,
            "up" => a -= amount,
            _ => panic!(),
        }
    }
    d * h
}
