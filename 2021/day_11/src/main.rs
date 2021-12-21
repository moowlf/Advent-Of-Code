use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate termion;
use termion::color;

fn print(vec: &Vec<Vec<u32>>) {
    print!("{}", termion::clear::All);
    for y in vec {
        for x in y {
            if *x == 0 {
                print!("{}0 ", color::Fg(color::LightYellow));
            } else {
                print!("{}{} ", color::Fg(color::White), x);
            }
        }
        println!();
    }
    println!();
}

fn main() {
    let input = parse_file("./inputs/input.txt");
    println!("Problem 1: {}", problem_1(&mut input.clone()));
    println!("Problem 2: {}", problem_2(&mut input.clone()));
}

fn parse_file(filename: &str) -> Vec<Vec<u32>> {
    let file = File::open(filename).expect("Failed to open file.");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split("")
                .filter(|&x| !x.is_empty())
                .map(|c| c.parse().unwrap())
                .collect()
        })
        .collect()
}

fn get_adjacent_location(input: &Vec<Vec<u32>>, x_pos: usize, y_pos: usize) -> Vec<(usize, usize)> {
    let mut vec = Vec::new();

    let max_width: usize = input[0].len() - 1;
    let max_height: usize = input.len() - 1;

    if x_pos != 0 {
        vec.push((x_pos - 1, y_pos));
    }

    if x_pos != max_width {
        vec.push((x_pos + 1, y_pos));
    }

    if y_pos != 0 {
        vec.push((x_pos, y_pos - 1));
    }

    if y_pos != max_height {
        vec.push((x_pos, y_pos + 1));
    }

    if x_pos != 0 && y_pos != 0 {
        vec.push((x_pos - 1, y_pos - 1));
    }

    if x_pos != 0 && y_pos != max_height {
        vec.push((x_pos - 1, y_pos + 1));
    }

    if x_pos != max_width && y_pos != 0 {
        vec.push((x_pos + 1, y_pos - 1));
    }

    if x_pos != max_width && y_pos != max_height {
        vec.push((x_pos + 1, y_pos + 1));
    }

    vec
}

fn problem_1(input: &mut Vec<Vec<u32>>) -> u64 {
    let x_size = input[0].len();
    let y_size = input.len();
    let mut num_flashes = 0;

    for _ in 0..100 {
        let mut flashes = Vec::new();

        for y in 0..y_size {
            for x in 0..x_size {
                if input[y][x] == 9 {
                    input[y][x] = 0;
                    flashes.push((x, y));
                    continue;
                }

                input[y][x] += 1;
            }
        }

        loop {
            if flashes.len() == 0 {
                break;
            }
            num_flashes += 1;

            let (curr_x, curr_y) = flashes.pop().unwrap();

            let neighbors = get_adjacent_location(input, curr_x, curr_y);

            for neighbour in neighbors {
                if input[neighbour.1][neighbour.0] == 9 {
                    flashes.push((neighbour.0, neighbour.1));
                    input[neighbour.1][neighbour.0] = 0;
                } else {
                    if input[neighbour.1][neighbour.0] != 0 {
                        input[neighbour.1][neighbour.0] += 1;
                    }
                }
            }
        }

        //print(input);
    }

    num_flashes
}

fn problem_2(input: &mut Vec<Vec<u32>>) -> u64 {
    let x_size = input[0].len();
    let y_size = input.len();
    let mut current_index = 0;

    'main: loop {
        let mut flashes = Vec::new();

        for y in 0..y_size {
            for x in 0..x_size {
                if input[y][x] == 9 {
                    input[y][x] = 0;
                    flashes.push((x, y));
                    continue;
                }

                input[y][x] += 1;
            }
        }

        let mut num_flashes = 0;
        loop {
            if flashes.len() == 0 {
                break;
            }
            num_flashes += 1;

            let (curr_x, curr_y) = flashes.pop().unwrap();

            let neighbors = get_adjacent_location(input, curr_x, curr_y);

            for neighbour in neighbors {
                if input[neighbour.1][neighbour.0] == 9 {
                    flashes.push((neighbour.0, neighbour.1));
                    input[neighbour.1][neighbour.0] = 0;
                } else {
                    if input[neighbour.1][neighbour.0] != 0 {
                        input[neighbour.1][neighbour.0] += 1;
                    }
                }
            }
        }

        current_index += 1;
        //print(input);

        if num_flashes == y_size * x_size {
            break 'main;
        }
    }

    current_index
}
