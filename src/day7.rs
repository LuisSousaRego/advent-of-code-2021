use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> Vec<u32> {
    let filename = "puzzleInputs/day7.txt";
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf: Vec<u8> = Vec::new();
    let mut positions: Vec<u32> = Vec::new();
    while reader.read_until(b',', &mut buf).unwrap() != 0 {
        let buf_str = String::from_utf8(buf.clone()).unwrap();
        let p: u32 = buf_str
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
        buf = Vec::new();
        positions.push(p);
    }
    positions
}

pub fn part1() -> u32 {
    let positions = parse_input();

    let min = positions.iter().fold(u32::MAX, |a, &b| a.min(b));
    let max = positions.iter().fold(u32::MIN, |a, &b| a.max(b));

    let mut min_fuel = u32::MAX;
    for crab_line in min..max {
        let mut fuel = 0;
        for pos in &positions {
            if pos > &crab_line {
                fuel += pos - crab_line;
            } else if pos < &crab_line {
                fuel += crab_line - pos;
            }
        }
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }
    min_fuel
}

pub fn part2() -> u32 {
    let positions = parse_input();

    let min = positions.iter().fold(u32::MAX, |a, &b| a.min(b));
    let max = positions.iter().fold(u32::MIN, |a, &b| a.max(b));

    let mut min_fuel = u32::MAX;
    for crab_line in min..max {
        let mut fuel = 0;
        for pos in &positions {
            let mut dist = 0;
            if pos > &crab_line {
                dist = pos - crab_line;
            } else if pos < &crab_line {
                dist = crab_line - pos;
            }
            fuel += dist * (dist + 1) / 2;
        }
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }
    min_fuel
}
