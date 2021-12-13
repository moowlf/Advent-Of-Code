use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_file(filename: &str) -> Vec<Vec<u32>> {
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

fn transpose_bits(input: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut result = Vec::new();

    for current_byte in input {
        for (index, current_bit) in current_byte.iter().enumerate() {
            if result.len() <= index {
                result.push(Vec::new());
            }

            match current_bit {
                0 | 1 => result[index].push(*current_bit),
                _ => (),
            }
        }
    }

    result
}

fn convert_to_u32_from_u8(data: &Vec<u8>) -> u32 {
    data.iter().fold(0, |acc, &x| (acc << 1) + x as u32)
}

fn convert_to_u32_from_u32(data: &Vec<u32>) -> u32 {
    data.iter().fold(0, |acc, &x| (acc << 1) + x as u32)
}

fn retrieve_common_bits(data: &Vec<Vec<u32>>) -> Vec<u8> {
    let mut common_bits = vec![0; data.len()];

    for (index, v) in data.iter().enumerate() {
        let current_sum: u32 = v.iter().sum();
        common_bits[index] = u8::from(current_sum >= (v.len() / 2).try_into().unwrap());
    }

    common_bits
}

fn retrieve_common_bit(data: &Vec<Vec<u32>>, pos: u32) -> u32 {
    let mut occurrences = [0, 0];

    for bit in data[pos as usize].iter() {
        match bit {
            0 => occurrences[0] += 1,
            1 => occurrences[1] += 1,
            _ => (),
        }
    }

    (occurrences[1] >= occurrences[0]) as u32
}

fn retrieve_less_common_bit(data: &Vec<Vec<u32>>, pos: u32) -> u32 {
    let mut occurrences = [0, 0];

    for bit in data[pos as usize].iter() {
        match bit {
            0 => occurrences[0] += 1,
            1 => occurrences[1] += 1,
            _ => (),
        }
    }

    !(occurrences[0] <= occurrences[1]) as u32
}

fn problem_1(input: &Vec<Vec<u32>>) -> u32 {
    let transposed_bits = transpose_bits(&input);
    let gamma = retrieve_common_bits(&transposed_bits);
    let epsilon: Vec<u8> = gamma.iter().map(|x| if *x != 0 { 0 } else { 1 }).collect();

    convert_to_u32_from_u8(&gamma) * convert_to_u32_from_u8(&epsilon)
}

fn problem_2(input: &Vec<Vec<u32>>) -> u32 {
    let mut current_index = 0;
    let mut current_possiblities = input.clone();

    loop {
        let transposed_bits = transpose_bits(&current_possiblities);
        let common_bit = retrieve_common_bit(&transposed_bits, current_index);

        current_possiblities.retain(|x| x[current_index as usize] == common_bit as u32);
        current_index += 1;

        if current_possiblities.len() == 1 {
            break;
        }
    }

    let oxygen_gen_rating = current_possiblities[0].to_owned();

    current_possiblities = input.clone();
    current_index = 0;

    loop {
        let transposed_bits = transpose_bits(&current_possiblities);
        let common_bit = retrieve_less_common_bit(&transposed_bits, current_index);

        current_possiblities.retain(|x| x[current_index as usize] == common_bit as u32);
        current_index += 1;

        if current_possiblities.len() == 1 {
            break;
        }
    }

    let co2_gen_rating = current_possiblities[0].to_owned();

    convert_to_u32_from_u32(&oxygen_gen_rating) * convert_to_u32_from_u32(&co2_gen_rating)
}

fn main() {
    let input = parse_file("./inputs/input.txt");
    println!("Problem_1: {}", problem_1(&input));
    println!("Problem_2: {}", problem_2(&input));
}
