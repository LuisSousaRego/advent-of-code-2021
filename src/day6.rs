use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1() -> i32 {
    let filename = "puzzleInputs/day6.txt";
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);

    let mut buf: Vec<u8> = Vec::new();

    let mut total_lantern = 0;
    while reader.read_until(b',', &mut buf).unwrap() != 0 {
        let buf_str = String::from_utf8(buf.clone()).unwrap();
        let age: u32 = buf_str
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
        buf = Vec::new();

        let mut lanterns = vec![age];
        for _ in 0..80 {
            let mut next_day_lanterns = vec![];
            // calculate day cicle
            for l in &lanterns {
                match l {
                    0 => {
                        next_day_lanterns.push(6);
                        next_day_lanterns.push(8);
                    }
                    _ => {
                        next_day_lanterns.push(l - 1);
                    }
                };
            }
            lanterns = next_day_lanterns;
        }
        total_lantern += lanterns.len();
    }

    total_lantern as i32
}

pub fn part2() -> u64 {
    let filename = "puzzleInputs/day6.txt";
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);

    let mut buf: Vec<u8> = Vec::new();

    let mut lantern_per_age: [u64; 9] = [0; 9];
    while reader.read_until(b',', &mut buf).unwrap() != 0 {
        let buf_str = String::from_utf8(buf.clone()).unwrap();
        let age: u8 = buf_str
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<u8>()
            .unwrap();
        buf = Vec::new();
        lantern_per_age[age as usize] += 1;
    }
    for _ in 0..256 {
        lantern_per_age = [
            lantern_per_age[1],
            lantern_per_age[2],
            lantern_per_age[3],
            lantern_per_age[4],
            lantern_per_age[5],
            lantern_per_age[6],
            lantern_per_age[7] + lantern_per_age[0],
            lantern_per_age[8],
            lantern_per_age[0],
        ];
    }
    lantern_per_age.iter().sum::<u64>()
}
