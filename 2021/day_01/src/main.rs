use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

fn parse_file(path: &str) -> Result<Vec<u32>, Error> {
    let file = File::open(path).expect("Failed to open file!");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            line.unwrap()
                .parse::<u32>()
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))
        })
        .collect()
}

fn problem_1(input: &Vec<u32>) -> u32 {
    let mut count = 0;
    for elems in input.windows(2) {
        if elems[1] > elems[0] {
            count += 1;
        }
    }

    count
}

fn problem_2(input: &Vec<u32>) -> u32 {
    let mut count = 0;
    let mut last_sum = input.iter().take(3).sum();

    for elems in input.windows(3).skip(1) {
        let current_sum: u32 = elems.iter().sum();

        if current_sum > last_sum {
            count += 1;
        }

        last_sum = current_sum;
    }

    count
}

fn main() {
    let input = parse_file("./inputs/input.txt").expect("Found error");

    println!("Problem #1: {}", problem_1(&input));
    println!("Problem #1: {}", problem_2(&input));
}
