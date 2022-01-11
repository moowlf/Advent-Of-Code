use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Target {
    x: [i64; 2],
    y: [i64; 2],
}

fn parse_file(filename: &str) -> Target {
    // Open file
    let file = File::open(filename).expect("File not found!");
    let mut reader = BufReader::new(file);

    // read line
    let mut data = String::new();
    reader.read_line(&mut data).expect("Failed to read line!");

    // parse
    let re = Regex::new(r"(-*\d+)").unwrap();
    let captures: Vec<i64> = re
        .find_iter(&data)
        .map(|grp| grp.as_str().parse().unwrap())
        .collect();

    Target {
        x: [captures[0], captures[1]],
        y: [captures[2], captures[3]],
    }
}

fn main() {
    let input = parse_file("./inputs/input.txt");
    problem(&input);
}

fn problem(input: &Target) {
    let mut possible_velocities = Vec::<(i64, i64)>::new();

    let (x_minv, x_maxv) = if input.x[0] > 0 {
        (1, input.x[1])
    } else {
        (-1, input.x[0])
    };

    let mut max_altitude = 0;
    for vx in 1..=x_maxv {
        for vy in -400..1000 {
            let (mut x, mut y) = (0, 0);
            let (mut curr_vx, mut curr_vy) = (vx, vy);
            let mut current_highest_altitude = 0;

            loop {
                x += curr_vx;
                y += curr_vy;

                curr_vx += if curr_vx < 0 {
                    1
                } else if curr_vx == 0 {
                    0
                } else {
                    -1
                };
                curr_vy -= 1;

                if current_highest_altitude < y {
                    current_highest_altitude = y;
                }

                if x >= input.x[0] && x <= input.x[1] && y <= input.y[1] && y >= input.y[0] {
                    possible_velocities.push((vx, vy));

                    if max_altitude < current_highest_altitude {
                        max_altitude = current_highest_altitude;
                    }

                    break;
                }

                if curr_vx == 0 && (x < input.x[0] || x > input.x[1]) {
                    break;
                }

                if curr_vx == 0 && y <= input.y[0] {
                    break;
                }
            }
        }
    }

    println!("Problem 1: {}", max_altitude);
    println!("Problem 2: {}", possible_velocities.len());
}
