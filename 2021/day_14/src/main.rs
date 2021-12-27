use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Input {
    initial_polymer: String,
    rules: HashMap<String, String>,
}

fn main() {
    let input = parse_file("./inputs/input.txt");
    println!("Problem 1: {}", problem(&input, 10));
    println!("Problem 2: {}", problem(&input, 40));
}

fn parse_file(filename: &str) -> Input {
    let file = File::open(filename).expect("Failed to open file.");
    let mut reader = BufReader::new(file);

    let mut initial_polymer: String = String::new();
    reader.read_line(&mut initial_polymer).unwrap();
    initial_polymer.truncate(initial_polymer.trim_right().len());

    let mut rules = HashMap::new();

    for line in reader.lines().skip(1) {
        let unwraped_line = line.unwrap();

        let splitted_line: Vec<&str> = unwraped_line.split(" -> ").collect();

        rules.insert(splitted_line[0].to_string(), splitted_line[1].to_string());
    }

    Input {
        initial_polymer,
        rules,
    }
}

fn problem(input: &Input, depth: u32) -> i64 {
    // * Initial couting
    let mut letter_counting = HashMap::new();

    for index in 0..input.initial_polymer.len() - 1 {
        let pair = input.initial_polymer[index..index + 2].to_string();
        *letter_counting.entry(pair).or_insert(0) += 1;
    }

    // * Do this depth times
    let mut char_counter = HashMap::new();
    for index in 0..depth {
        let mut new_counter: HashMap<String, u64> = HashMap::new();
        let last_cycle = index == depth - 1;

        for (p_key, p_value) in letter_counting.iter() {
            let conversion_char = input.rules.get(p_key).unwrap();

            let mut left_pair = String::new();
            left_pair.push(p_key.as_bytes()[0] as char);
            left_pair.push(conversion_char.as_bytes()[0] as char);
            *new_counter.entry(left_pair).or_insert(0) += p_value;

            let mut right_pair = String::new();
            right_pair.push(conversion_char.as_bytes()[0] as char);
            right_pair.push(p_key.as_bytes()[1] as char);
            *new_counter.entry(right_pair).or_insert(0) += p_value;

            if last_cycle {
                *char_counter
                    .entry(conversion_char.as_bytes()[0] as char)
                    .or_insert(0) += p_value;
                *char_counter.entry(p_key.as_bytes()[0] as char).or_insert(0) += p_value;
            }
        }

        letter_counting = new_counter;
    }

    *char_counter
        .entry(input.initial_polymer.chars().last().unwrap())
        .or_insert(0) += 1;

    let max = char_counter.iter().max_by_key(|entry| entry.1).unwrap().1;
    let min = char_counter.iter().min_by_key(|entry| entry.1).unwrap().1;
    (max - min).try_into().unwrap()
}
