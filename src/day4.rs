use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

struct Input {
    numbers: Vec<i32>,
    boards: Vec<Vec<Vec<i32>>>,
}
const BOARD_LEN: usize = 5;

fn parse_input() -> Input {
    let filename = "puzzleInputs/day4.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut numbers: Vec<i32> = Vec::new();
    let mut boards: Vec<Vec<Vec<i32>>> = Vec::new();
    let mut current_board: Vec<Vec<i32>> = Vec::new();

    let mut is_first_line = true;
    for line in reader.lines() {
        let line = line.unwrap();
        if line == "" {
            continue;
        } else if is_first_line {
            let str_nums: Vec<&str> = line.split(',').collect();
            numbers = str_nums
                .into_iter()
                .map(|n| FromStr::from_str(n).unwrap())
                .collect();
            is_first_line = false;
        } else {
            let str_line: Vec<&str> = line.split_whitespace().collect();

            let board_line: Vec<i32> = str_line
                .into_iter()
                .map(|n| FromStr::from_str(n).unwrap())
                .collect();
            current_board.push(board_line.clone());
        }
        if current_board.len() == BOARD_LEN {
            boards.push(current_board.clone());
            current_board = Vec::new();
        }
    }

    Input {
        numbers: numbers.clone(),
        boards: boards.clone(),
    }
}

pub fn part1() -> i32 {
    let input = parse_input();

    let mut score: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 5]; 5]; input.boards.len()];

    for input_number in &input.numbers {
        for board_index in 0..input.boards.len() {
            let board = &input.boards[board_index];
            for r in 0..BOARD_LEN {
                for c in 0..BOARD_LEN {
                    if board[r][c] == *input_number {
                        score[board_index][r][c] = true;

                        // count score
                        let mut row_score = 0;
                        let mut col_score = 0;
                        for i in 0..BOARD_LEN {
                            if score[board_index][r][i] {
                                row_score += 1
                            }
                            if score[board_index][i][c] {
                                col_score += 1
                            }
                        }
                        if row_score == BOARD_LEN || col_score == BOARD_LEN {
                            // board is winner, calculate score
                            let mut sum_of_unmarked = 0;
                            for i in 0..BOARD_LEN {
                                for j in 0..BOARD_LEN {
                                    if !score[board_index][i][j] {
                                        sum_of_unmarked += board[i][j];
                                    }
                                }
                            }
                            return input_number * sum_of_unmarked;
                        }
                    }
                }
            }
        }
    }
    return 0;
}
