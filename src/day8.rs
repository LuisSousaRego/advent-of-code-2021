use std::collections::HashSet;
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
    let input = parse_input();
    let mut sum = 0;
    for (signal_patters, output_values) in input {
        let mut found_numbers: [HashSet<char>; 10] = Default::default();
        // find 1, 4, 7, 8
        for sp in &signal_patters {
            match sp.len() {
                2 => {
                    sp.chars().for_each(|c| {
                        found_numbers[1].insert(c);
                    });
                }
                4 => {
                    sp.chars().for_each(|c| {
                        found_numbers[4].insert(c);
                    });
                }
                3 => {
                    sp.chars().for_each(|c| {
                        found_numbers[7].insert(c);
                    });
                }
                7 => {
                    sp.chars().for_each(|c| {
                        found_numbers[8].insert(c);
                    });
                }
                _ => {}
            };
        }

        // find h1
        let mut h1: char = ' ';
        for letter in &found_numbers[7] {
            if !found_numbers[1].contains(&letter) {
                h1 = *letter;
                break;
            }
        }

        // find 6
        for sp in &signal_patters {
            if sp.len() == 6 {
                let mut common = 0;
                for letter in &found_numbers[1] {
                    if sp.contains(*letter) {
                        common += 1;
                    }
                }
                if common == 1 {
                    sp.chars().for_each(|c| {
                        found_numbers[6].insert(c);
                    });
                }
            }
        }

        // find v2 and v4
        let mut v4: char = ' ';
        let v2 = *found_numbers[1]
            .difference(&found_numbers[6])
            .next()
            .unwrap();
        for c in &found_numbers[1] {
            if *c != v2 {
                v4 = *c;
                break;
            }
        }

        // find 2 and 5
        for sp in &signal_patters {
            // 2
            if sp.contains(h1) && sp.contains(v2) && !sp.contains(v4) {
                sp.chars().for_each(|c| {
                    found_numbers[2].insert(c);
                });
            }
            // 5
            if sp.len() == 5 && !sp.contains(v2) {
                sp.chars().for_each(|c| {
                    found_numbers[5].insert(c);
                });
            }
        }

        // find 3
        for sp in &signal_patters {
            if sp.len() == 5 && sp.contains(v2) && sp.contains(v4) {
                sp.chars().for_each(|c| {
                    found_numbers[3].insert(c);
                });
            }
        }

        // find v3
        let v3_v2 = found_numbers[2].difference(&found_numbers[5]);
        let mut v3: char = ' ';
        for v in v3_v2 {
            if v != &v2 {
                v3 = *v;
            }
        }

        // find 9 and 0
        for sp in &signal_patters {
            // 9
            if sp.len() == 6 && !sp.contains(v3) {
                sp.chars().for_each(|c| {
                    found_numbers[9].insert(c);
                });
            }

            // 0
            if sp.len() == 6 && sp.contains(v3) && sp.contains(v2) {
                sp.chars().for_each(|c| {
                    found_numbers[0].insert(c);
                });
            }
        }

        // decode numbers
        let mut decoded = String::new();
        'vals: for ov in &output_values {
            'nums: for f_n in &found_numbers {
                if f_n.len() == ov.len() {
                    for c in ov.chars() {
                        if !f_n.contains(&c) {
                            continue 'nums;
                        }
                    }
                    let decoded_digit = found_numbers.iter().position(|r| r == f_n).unwrap();
                    decoded.push(char::from_digit(decoded_digit as u32, 10).unwrap());
                    continue 'vals;
                }
            }
        }
        sum += decoded.parse::<u32>().unwrap();
    }
    sum
}
