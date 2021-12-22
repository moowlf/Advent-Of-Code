use crossterm::{
    cursor::MoveTo, queue, style::Print, terminal::Clear, terminal::ClearType, ExecutableCommand,
};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{stdout, Write};
use std::io::{BufRead, BufReader};

enum FoldInstructions {
    X(u32),
    Y(u32),
}

struct Input {
    points: Vec<(u32, u32)>,
    instructions: Vec<FoldInstructions>,
}

fn main() {
    let mut input = parse_file("./inputs/input.txt");
    println!("Problem 1: {}", problem_1(&mut input));
    problem_2(&mut input);
}

fn parse_file(filename: &str) -> Input {
    let file = File::open(filename).expect("Failed to open file.");
    let reader = BufReader::new(file);

    let mut points = Vec::new();
    let mut instructions = Vec::new();

    let mut has_found_empty_line = false;
    for line in reader.lines() {
        let unwraped_line = line.unwrap();

        if unwraped_line.is_empty() {
            has_found_empty_line = true;
            continue;
        }

        if !has_found_empty_line {
            // * Is point coordinate
            let data: Vec<String> = unwraped_line.split(",").map(|c| c.to_string()).collect();
            points.push((data[0].parse().unwrap(), data[1].parse().unwrap()));
            continue;
        }

        // * is instruction
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\s(x|y)=(\d+)").unwrap();
        }

        let result = RE.captures(&unwraped_line).unwrap();
        let axis = result.get(1).map_or("", |m| m.as_str());
        let number = result.get(2).map_or("", |m| m.as_str());

        match axis {
            "x" => instructions.push(FoldInstructions::X(number.parse().unwrap())),
            "y" => instructions.push(FoldInstructions::Y(number.parse().unwrap())),
            _ => panic!("Failed to parse axis"),
        }
    }

    Input {
        points,
        instructions,
    }
}

fn problem_1(input: &mut Input) -> u32 {
    let fold = input.instructions.first().unwrap();
    match fold {
        FoldInstructions::X(i) => {
            for point in input.points.iter_mut() {
                if point.0 > *i {
                    point.0 = i - (point.0 - i);
                }
            }
        }
        FoldInstructions::Y(i) => {
            for point in input.points.iter_mut() {
                if point.1 > *i {
                    point.1 = i - (point.1 - i);
                }
            }
        }
    }

    let mut visible_points = HashSet::new();

    for point in input.points.iter() {
        visible_points.insert(point);
    }

    visible_points.len() as u32
}

fn problem_2(input: &mut Input) {
    for fold in input.instructions.iter() {
        match fold {
            FoldInstructions::X(i) => {
                for point in input.points.iter_mut() {
                    if point.0 > *i {
                        point.0 = i - (point.0 - i);
                    }
                }
            }
            FoldInstructions::Y(i) => {
                for point in input.points.iter_mut() {
                    if point.1 > *i {
                        point.1 = i - (point.1 - i);
                    }
                }
            }
        }
    }

    let mut stdout = stdout();
    stdout.execute(Clear(ClearType::All)).unwrap();

    for point in input.points.iter() {
        queue!(stdout, MoveTo(point.0 as u16, point.1 as u16));
        queue!(stdout, Print("#"));
    }
    stdout.flush().unwrap();
}
