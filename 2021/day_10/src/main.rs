use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = parse_file("./inputs/input.txt");

    let (problem_1_solution, filtered_input) = problem_1(&input);
    println!("Problem 1: {}", problem_1_solution);
    println!("Problem 2: {}", problem_2(&filtered_input));
}

fn parse_file(filepath: &str) -> Vec<Vec<char>> {
    let file = File::open(filepath).expect("Failed to open file!");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

fn problem_1(input: &Vec<Vec<char>>) -> (u32, Vec<Vec<char>>) {
    let mut score: u32 = 0;
    let mut filtered: Vec<Vec<char>> = Vec::new();

    for line in input.iter() {
        let mut stack: Vec<char> = Vec::new();
        let mut successful: bool = true;

        for ch in line.iter() {
            match ch {
                // Open ones
                '(' => stack.push('('),
                '[' => stack.push('['),
                '{' => stack.push('{'),
                '<' => stack.push('<'),

                // Close ones
                _ => {
                    let last_char = stack.pop().unwrap();

                    match ch {
                        ')' => {
                            if last_char != '(' {
                                score += 3;
                                successful = false;
                                break;
                            }
                        }
                        ']' => {
                            if last_char != '[' {
                                score += 57;
                                successful = false;
                                break;
                            }
                        }
                        '}' => {
                            if last_char != '{' {
                                score += 1197;
                                successful = false;
                                break;
                            }
                        }
                        '>' => {
                            if last_char != '<' {
                                score += 25137;
                                successful = false;
                                break;
                            }
                        }
                        _ => panic!("Failed to match ch"),
                    }
                }
            }
        }

        if successful && stack.len() != 0 {
            filtered.push(stack.clone());
        }
    }

    (score, filtered)
}

fn problem_2(input: &Vec<Vec<char>>) -> u64 {
    let mut scores: Vec<u64> = Vec::new();
    for stack in input.iter() {
        let mut current_score: u64 = 0;
        for elem in stack.iter().rev() {
            let mut reward = 0;
            match elem {
                '(' => reward = 1,
                '[' => reward = 2,
                '{' => reward = 3,
                '<' => reward = 4,
                _ => panic!("Failed to match elem."),
            }

            current_score = current_score * 5 + reward;
        }
        scores.push(current_score);
    }

    scores.sort();
    scores[scores.len() / 2]
}
