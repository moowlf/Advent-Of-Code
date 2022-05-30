use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq)]
enum Space {
    EastCucumber,
    SouthCucumber,
    EmptySpace,
}

impl Space {
    fn from(ch: char) -> Space {
        match ch {
            '>' => Space::EastCucumber,
            'v' => Space::SouthCucumber,
            '.' => Space::EmptySpace,
            _ => panic!("Impossible"),
        }
    }
}

fn move_east(input: &mut Vec<Vec<Space>>) -> u64 {
    let mut number_changes = 0;

    let (x_size, y_size) = (input[0].len(), input.len());

    for row in 0..y_size {
        let can_last_wrap = input[row][0] == Space::EmptySpace;
        let mut column = 0;
        while column < x_size {
            if input[row][column] != Space::EastCucumber {
                column += 1;
                continue;
            }

            let next_column_id = (column + 1) % x_size;
            if next_column_id == 0 && !can_last_wrap {
                column += 1;
                continue;
            }

            if input[row][next_column_id] == Space::EmptySpace {
                input[row][column] = Space::EmptySpace;
                input[row][next_column_id] = Space::EastCucumber;
                column += 2;
                number_changes += 1;
                continue;
            }

            column += 1;
        }
    }

    number_changes
}

fn move_south(input: &mut Vec<Vec<Space>>) -> u64 {
    let mut number_changes = 0;
    let (x_size, y_size) = (input[0].len(), input.len());

    for column in 0..x_size {
        let can_last_wrap = input[0][column] == Space::EmptySpace;
        let mut row = 0;
        while row < y_size {
            if input[row][column] != Space::SouthCucumber {
                row += 1;
                continue;
            }

            let next_row_id = (row + 1) % y_size;
            if next_row_id == 0 && !can_last_wrap {
                row += 1;
                continue;
            }

            if input[next_row_id][column] == Space::EmptySpace {
                input[row][column] = Space::EmptySpace;
                input[next_row_id][column] = Space::SouthCucumber;
                number_changes += 1;
                row += 2;
                continue;
            }

            row += 1;
        }
    }

    number_changes
}

fn solve_1(mut input: Vec<Vec<Space>>) -> u64 {
    let mut iteration = 0;

    loop {
        iteration += 1;
        let east_moved = move_east(&mut input);
        let south_moved = move_south(&mut input);

        if east_moved + south_moved == 0 {
            break;
        }
    }

    iteration
}

fn parse_file(filename: &str) -> Vec<Vec<Space>> {
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().chars().map(|c| Space::from(c)).collect())
        .collect()
}

fn main() {
    let input_data = parse_file("./inputs/input.txt");
    println!("Problem #1: {}", solve_1(input_data));
}
