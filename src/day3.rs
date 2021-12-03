use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1() -> i32 {
    let filename = "puzzleInputs/day3.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    const INPUT_LINE_LEN: usize = 12;
    let mut lines_counter: u32 = 0;
    let mut ones_counter: [u16; INPUT_LINE_LEN] = [0; INPUT_LINE_LEN];

    for line in reader.lines() {
        let line = line.unwrap();
        for i in 0..INPUT_LINE_LEN {
            ones_counter[i] += line.chars().nth(i).unwrap().to_digit(10).unwrap() as u16;
        }
        lines_counter += 1;
    }
    println!("ones_counter {:#?}", ones_counter);

    let mut gamma_rate_arr: Vec<char> = Vec::new();
    let mut epsilon_rate_arr: Vec<char> = Vec::new();
    for i in 0..INPUT_LINE_LEN {
        if ones_counter[i] as u32 > (lines_counter / 2) {
            gamma_rate_arr.push('1');
            epsilon_rate_arr.push('0');
        } else {
            gamma_rate_arr.push('0');
            epsilon_rate_arr.push('1');
        }
    }

    let gamma_rate: String = gamma_rate_arr.into_iter().collect();
    let epsilon_rate_arr: String = epsilon_rate_arr.into_iter().collect();
    println!("gamma_rate: {}", gamma_rate);
    i32::from_str_radix(&gamma_rate, 2).unwrap()
        * i32::from_str_radix(&epsilon_rate_arr, 2).unwrap()
}
