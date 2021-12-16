use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = parse_file("./inputs/input.txt");
    println!("Problem 1: {}", problem_1(&input));
    println!("Problem 2: {}", problem_2(&input));
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

fn problem_1(input: &Vec<u32>) -> u32 {
    let mut fuel_costs: Vec<u32> = Vec::new();
    let max_value = input.len() + 1;

    for position in 0..max_value {
        let mut sum: u32 = 0;
        for crab in input.iter() {
            let cost: i32 = position as i32 - *crab as i32;
            sum += cost.abs() as u32;
        }

        fuel_costs.push(sum);
    }

    fuel_costs.sort();
    fuel_costs[0]
}

fn problem_2(input: &Vec<u32>) -> u32 {
    let mut fuel_costs: Vec<u32> = Vec::new();
    let max_value = input.len() + 1;
    let mut dyn_prog = HashMap::<i32, u32>::new();

    for position in 0..max_value {
        let mut sum: u32 = 0;
        for crab in input.iter() {
            let move_len: i32 = (position as i32 - *crab as i32).abs();

            let cost = dyn_prog.get(&move_len);

            match cost {
                Some(x) => sum += x,
                None => {
                    let mut calculate_cost: u32 = 0;

                    for move_id in 1..move_len + 1 {
                        calculate_cost += move_id as u32;
                    }

                    dyn_prog.insert(move_len, calculate_cost);
                    sum += calculate_cost;
                }
            }
        }

        fuel_costs.push(sum);
    }

    fuel_costs.sort();
    fuel_costs[0]
}
