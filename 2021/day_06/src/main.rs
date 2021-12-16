use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut input = parse_file("./inputs/input.txt");
    println!("Problem 1: {}", problem_1(&mut input.clone()));
    println!("Problem 2: {}", problem_2(&mut input.clone()));
}

fn parse_file(filepath: &str) -> Vec<u32> {
    let file = File::open(filepath).expect("Failed to open file!");
    let reader = BufReader::new(file);
    let mut v = Vec::new();

    for line in reader.lines() {
        let current_line = line.unwrap();
        let numbers: Vec<&str> = current_line.split(",").collect();

        for number in numbers.iter() {
            v.push(number.parse().unwrap());
        }
    }

    v
}

fn problem_1(input: &mut Vec<u32>) -> usize {
    for _ in 0..80 {
        let mut to_add: u32 = 0;
        input.iter_mut().for_each(|x| {
            if *x == 0 {
                to_add += 1;
                *x = 6;
                return;
            }

            *x -= 1;
        });

        for _ in 0..to_add {
            input.push(8);
        }
    }

    input.len()
}

fn problem_2(input: &mut Vec<u32>) -> u64 {
    let mut arr: [u64; 256 + 9] = [0; 256 + 9];
    arr[0] = input.len() as u64;

    input.iter().for_each(|x| arr[*x as usize] += 1);

    for day in 1..256 {
        arr[day + 7] += arr[day];
        arr[day + 9] += arr[day];
        arr[day] += arr[day - 1];
    }

    arr[255]
}
