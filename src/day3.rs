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
    i32::from_str_radix(&gamma_rate, 2).unwrap()
        * i32::from_str_radix(&epsilon_rate_arr, 2).unwrap()
}

pub fn part2() -> i32 {
    let filename = "puzzleInputs/day3.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // get file into a vec of arrs
    let mut input: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        input.push(line.chars().collect());
    }

    let mut oxygen_generator: Vec<Vec<char>> = input.clone();
    let mut c02_scrubber: Vec<Vec<char>> = input.clone();

    for bit_index in 0..input[0].len() {
        if oxygen_generator.len() > 1 {
            // find the most common
            let mut ones = 0;
            let mut zeros = 0;
            for line in 0..oxygen_generator.len() {
                if oxygen_generator[line][bit_index] == '1' {
                    ones += 1;
                } else {
                    zeros += 1;
                }
            }

            let mut most_common: char = ' ';
            if ones > zeros {
                most_common = '1';
            } else if ones < zeros {
                most_common = '0';
            }

            let mut temp_oxygen_generator: Vec<Vec<char>> = Vec::new();
            for line in 0..oxygen_generator.len() {
                if oxygen_generator[line][bit_index] == most_common
                    || (most_common == ' ' && oxygen_generator[line][bit_index] == '1')
                {
                    temp_oxygen_generator.push(oxygen_generator[line].clone());
                }
            }
            oxygen_generator = temp_oxygen_generator.clone();
        }

        if c02_scrubber.len() > 1 {
            // find the least common
            let mut ones = 0;
            let mut zeros = 0;
            for line in 0..c02_scrubber.len() {
                if c02_scrubber[line][bit_index] == '1' {
                    ones += 1;
                } else {
                    zeros += 1;
                }
            }

            let mut least_common: char = ' ';
            if ones > zeros {
                least_common = '0';
            } else if ones < zeros {
                least_common = '1';
            }

            let mut temp_c02_scrubber: Vec<Vec<char>> = Vec::new();
            for line in 0..c02_scrubber.len() {
                if c02_scrubber[line][bit_index] == least_common
                    || (least_common == ' ' && c02_scrubber[line][bit_index] == '0')
                {
                    temp_c02_scrubber.push(c02_scrubber[line].clone());
                }
            }
            c02_scrubber = temp_c02_scrubber.clone();
        }
    }

    let oxygen_generator_str: String = oxygen_generator[0].clone().into_iter().collect();
    let c02_scrubber_str: String = c02_scrubber[0].clone().into_iter().collect();

    i32::from_str_radix(&oxygen_generator_str, 2).unwrap()
        * i32::from_str_radix(&c02_scrubber_str, 2).unwrap()
}
