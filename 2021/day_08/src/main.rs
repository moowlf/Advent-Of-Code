use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = parse_file("./inputs/input.txt");

    let elements_by_size = HashMap::from([
        (0, vec![]),
        (1, vec![]),
        (2, vec![1]),
        (3, vec![7]),
        (4, vec![4]),
        (5, vec![2, 3, 5]),
        (6, vec![0, 6, 9]),
        (7, vec![8]),
    ]);

    println!("Problem 1: {}", problem_1(&input, &elements_by_size));
    println!("Problem 2: {}", problem_2(&input));
}

#[derive(Debug)]
struct Entry {
    unique_signal_patterns: [String; 10],
    digital_output: [String; 4],
}

fn parse_file(filepath: &str) -> Vec<Entry> {
    let file = File::open(filepath).expect("Failed to open file.");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|r_line| {
            let line = r_line.unwrap();
            let separated_line: Vec<&str> = line.split(" | ").collect();

            let unique_signal_patterns: [String; 10] = separated_line[0]
                .split(" ")
                .map(|value| value.to_string())
                .collect::<Vec<String>>()
                .try_into()
                .unwrap();

            let digital_output = separated_line[1]
                .split(" ")
                .map(|value| value.to_string())
                .collect::<Vec<String>>()
                .try_into()
                .unwrap();

            Entry {
                unique_signal_patterns,
                digital_output,
            }
        })
        .collect()
}

fn problem_1(input: &Vec<Entry>, elements_by_size: &HashMap<u32, Vec<u32>>) -> u32 {
    let mut sum: u32 = 0;

    for entry in input {
        for single_digital_output in &entry.digital_output {
            let possibilities = elements_by_size
                .get(&(single_digital_output.len() as u32))
                .unwrap();
            if possibilities.len() == 1 {
                sum += 1;
            }
        }
    }

    sum
}

fn in_string(a: &str, b: &str) -> bool {
    b.chars().all(|ch| a.contains(ch))
}

fn substract_strings(a: &str, b: &str) -> i32 {
    let mut result: i32 = 0;

    b.chars().for_each(|c| {
        if a.contains(c) {
            result += 1
        }
    });

    a.len() as i32 - result
}

fn problem_2(input: &Vec<Entry>) -> u64 {
    let mut result = 0 as u64;
    for entry in input {
        let mut patterns = HashMap::<u32, Vec<String>>::new();
        for signal in entry.unique_signal_patterns.iter() {
            let size = signal.len() as u32;

            let mut signal_char: Vec<char> = signal.chars().collect();
            signal_char.sort_by(|a, b| b.cmp(a));

            if patterns.contains_key(&size) {
                patterns
                    .get_mut(&size)
                    .unwrap()
                    .push(String::from_iter(signal_char));
            } else {
                patterns.insert(size, vec![String::from_iter(signal_char)]);
            }
        }

        let temporary_string = String::from("");
        let mut found_elements = HashMap::from([
            (0, &temporary_string),
            (1, &patterns.get(&2).unwrap()[0]),
            (2, &temporary_string),
            (3, &temporary_string),
            (4, &patterns.get(&4).unwrap()[0]),
            (5, &temporary_string),
            (6, &temporary_string),
            (7, &patterns.get(&3).unwrap()[0]),
            (8, &patterns.get(&7).unwrap()[0]),
            (9, &temporary_string),
        ]);

        // 6 elements solve
        for entry in patterns.get(&6).unwrap().iter() {
            if in_string(entry, found_elements.get(&4).unwrap()) {
                *found_elements.get_mut(&9).unwrap() = entry;
            } else if !in_string(entry, found_elements.get(&7).unwrap()) {
                *found_elements.get_mut(&6).unwrap() = entry;
            } else {
                *found_elements.get_mut(&0).unwrap() = entry;
            }
        }

        // 5 elements solve
        for entry in patterns.get(&5).unwrap().iter() {
            if substract_strings(found_elements.get(&6).unwrap(), entry) == 1 {
                *found_elements.get_mut(&5).unwrap() = entry;
            } else if in_string(entry, found_elements.get(&7).unwrap()) {
                *found_elements.get_mut(&3).unwrap() = entry;
            } else {
                *found_elements.get_mut(&2).unwrap() = entry;
            }
        }

        let mut number = 0 as u64;
        for (idx, output) in entry.digital_output.iter().enumerate() {
            let mut sorted_output = output.chars().collect::<Vec<char>>();
            sorted_output.sort_by(|a, b| b.cmp(a));
            let sorted_output = String::from_iter(sorted_output);

            for data in found_elements.iter() {
                if sorted_output.eq(*data.1) {
                    number += 10_u64.pow(3 - idx as u32) * data.0;
                    break;
                }
            }
        }

        result += number;
    }
    result
}
