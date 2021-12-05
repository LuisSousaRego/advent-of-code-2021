use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input(ignore_diagnoals: bool) -> Vec<((i32, i32), (i32, i32))> {
    let filename = "puzzleInputs/day5.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut points: Vec<((i32, i32), (i32, i32))> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let line_split: Vec<&str> = line.split(" -> ").collect();
        let mut first_split = line_split[0].split(',');
        let mut second_split = line_split[1].split(',');

        let point_pair: ((i32, i32), (i32, i32)) = (
            (
                first_split.next().unwrap().parse().unwrap(),
                first_split.next().unwrap().parse().unwrap(),
            ),
            (
                second_split.next().unwrap().parse().unwrap(),
                second_split.next().unwrap().parse().unwrap(),
            ),
        );

        if ignore_diagnoals
            && !(point_pair.0 .0 == point_pair.1 .0 || point_pair.0 .1 == point_pair.1 .1)
        {
            continue;
        } else {
            points.push(point_pair);
        }
    }

    points
}

pub fn part1() -> i32 {
    let points = parse_input(true);
    let mut points_with_lines: HashMap<(i32, i32), i32> = HashMap::new();

    for (first_point, second_point) in points {
        if first_point.0 == second_point.0 {
            // vertical line
            let range: (i32, i32);
            if first_point.1 > second_point.1 {
                range = (second_point.1, first_point.1 + 1)
            } else {
                range = (first_point.1, second_point.1 + 1)
            }

            let x = first_point.0;
            for y in range.0..range.1 {
                match points_with_lines.get(&(x, y)) {
                    Some(&n) => points_with_lines.insert((x, y), n + 1),
                    _ => points_with_lines.insert((x, y), 1),
                };
            }
        } else {
            //horizontal line
            let range: (i32, i32);
            if first_point.0 > second_point.0 {
                range = (second_point.0, first_point.0 + 1)
            } else {
                range = (first_point.0, second_point.0 + 1)
            }

            let y = first_point.1;
            for x in range.0..range.1 {
                match points_with_lines.get(&(x, y)) {
                    Some(&n) => points_with_lines.insert((x, y), n + 1),
                    _ => points_with_lines.insert((x, y), 1),
                };
            }
        }
    }

    // count intersections
    let mut sum: i32 = 0;
    for (_, &number_of_lines) in points_with_lines.iter() {
        if number_of_lines > 1 {
            sum += 1;
        }
    }
    sum
}

pub fn part2() -> i32 {
    0
}
