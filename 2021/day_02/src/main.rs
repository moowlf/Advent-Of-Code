use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
enum Directions {
    Forward(u32),
    Up(u32),
    Down(u32)
}

fn parse_file(filename: &str) -> Vec<Directions> {

    let file = File::open(filename).expect("Failed to open file!");
    let reader = BufReader::new(file);

    reader.lines().map(|line| {

        let line : String = line.unwrap();
        let mut data = line.split_whitespace();

        let dir_type = data.next().unwrap();
        let dir_value = data.next().unwrap().parse::<u32>().unwrap();

        match dir_type {
            "forward" => Directions::Forward(dir_value),
            "up" => Directions::Up(dir_value),
            "down" => Directions::Down(dir_value),
            _ => panic!("Failed to read directions")
        }
    }).collect()
}

fn problem_1(input: &Vec<Directions>) -> u32 {

    let mut coords = (0, 0);

    for direction  in input {

        match direction {
            Directions::Up(x) => coords.1 -= x,
            Directions::Down(x) => coords.1 += x,
            Directions::Forward(x) => coords.0 += x,
        }
    }

    coords.0 * coords.1
}

fn problem_2(input: &Vec<Directions>) -> u32 {

    let mut coords = (0, 0, 0);

    for direction  in input {

        match direction {
            Directions::Up(x) => coords.2 -= x,
            Directions::Down(x) => coords.2 += x,
            Directions::Forward(x) => {coords.0 += x; coords.1 += coords.2 * x },
        }
    }

    coords.0 * coords.1
}


fn main() {
    let input = parse_file("./inputs/input.txt");

    println!("{}", problem_1(&input));
    println!("{}", problem_2(&input));
}
