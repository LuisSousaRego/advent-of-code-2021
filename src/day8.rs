use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> Vec<(Vec<String>, Vec<String>)> {
    let filename = "puzzleInputs/day8.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut input: Vec<(Vec<String>, Vec<String>)> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();

        let split: Vec<&str> = line.split('|').collect();
        input.push((
            split[0]
                .split_whitespace()
                .map(|s| String::from(s))
                .collect(),
            split[1]
                .split_whitespace()
                .map(|s| String::from(s))
                .collect(),
        ));
    }
    input
}

pub fn part1() -> u32 {
    let input = parse_input();
    let mut counter = 0;
    for (_signal_patters, output_values) in input {
        for ov in output_values {
            match ov.len() {
                2 | 3 | 4 | 7 => {
                    counter += 1;
                }
                _ => {}
            };
        }
    }
    counter
}

pub fn part2() -> u32 {
    0
}
