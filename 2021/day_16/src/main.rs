use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = parse_file("./inputs/input.txt");

    println!("Problem 1: {}", problem_1(&input));
    println!("Problem 2: {}", problem_2(&input));
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => panic!("Char found should not exist."),
    }
}

fn parse_file(filename: &str) -> String {
    let file = File::open(filename).expect("Failed to open file.");
    let mut reader = BufReader::new(file);

    let mut data: String = String::new();
    reader.read_line(&mut data).unwrap();
    data.truncate(data.trim_right().len());

    let mut result = String::new();
    result.reserve(data.len() * 4);

    for chr in data.chars() {
        let bits = to_binary(chr);

        for bit in bits.chars() {
            result.push(bit);
        }
    }

    result
}

#[derive(Debug)]
struct Packet {
    sum_version: u64,
    version: u8,
    type_id: u8,
    value: u64,
    subpackets: Vec<Packet>,
}

impl Packet {
    fn new() -> Packet {
        let packet = Packet {
            sum_version: 0,
            version: 0,
            type_id: 0,
            value: 0,
            subpackets: vec![],
        };

        packet
    }

    fn parse(self: &mut Self, input: &String, index: usize) -> usize {
        let mut current_index = index;

        self.version = u8::from_str_radix(&input[index..index + 3], 2).unwrap();
        self.type_id = u8::from_str_radix(&input[index + 3..index + 6], 2).unwrap();

        self.sum_version += self.version as u64;

        let read_bits = match self.type_id {
            4 => self.parse_type_4(input, current_index + 6),
            _ => self.parse_type_6(input, current_index + 6),
        };

        current_index += read_bits;
        current_index - index
    }

    fn parse_type_4(self: &mut Self, input: &String, index: usize) -> usize {
        let mut current_string = String::new();
        let mut current_index = index;
        let mut block_bits_read = 0;

        loop {
            let last_group = input.as_bytes()[current_index] as char == '0';
            current_string.push_str(&input[current_index + 1..current_index + 5]);
            current_index = current_index + 5;
            block_bits_read += 1;

            if last_group {
                break;
            }
        }

        self.value = u64::from_str_radix(&current_string, 2).unwrap();
        let read_bits = block_bits_read * 5 + 6;
        read_bits
    }

    fn parse_type_6(self: &mut Self, input: &String, index: usize) -> usize {
        let length_type_id = input.as_bytes()[index] as char;

        let bits_to_read = match length_type_id {
            '0' => 15,
            '1' => 11,
            _ => panic!("Failed to match"),
        };

        let mut subpacket_length =
            usize::from_str_radix(&input[index + 1..index + 1 + bits_to_read], 2).unwrap();
        let mut bits_read = 0;

        loop {
            if length_type_id == '0' && bits_read >= subpacket_length {
                break;
            }

            if length_type_id == '1' && subpacket_length == 0 {
                break;
            }

            let mut new_packet = Packet::new();
            bits_read += new_packet.parse(input, index + 1 + bits_to_read + bits_read);
            self.sum_version += new_packet.sum_version;
            self.subpackets.push(new_packet);

            if length_type_id == '1' {
                subpacket_length -= 1;
            }
        }

        bits_read + bits_to_read + 1 + 6
    }
}

fn problem_1(input: &String) -> u64 {
    let mut packet = Packet::new();
    packet.parse(input, 0);

    packet.sum_version
}

fn calculate_result(packet: &Packet) -> i64 {
    let mut value = 0;

    match packet.type_id {
        0 => {
            for pck in packet.subpackets.iter() {
                value += calculate_result(pck);
            }
        }
        1 => {
            value = 1;
            for pck in packet.subpackets.iter() {
                value *= calculate_result(pck);
            }
        }
        2 => {
            value = i64::MAX;
            for pck in packet.subpackets.iter() {
                let tmp = calculate_result(pck);

                if tmp < value {
                    value = tmp;
                }
            }
        }
        3 => {
            value = i64::MIN;
            for pck in packet.subpackets.iter() {
                let tmp = calculate_result(pck);

                if tmp > value {
                    value = tmp;
                }
            }
        }
        4 => {
            value = packet.value as i64;
        }
        5 => {
            value = if calculate_result(&packet.subpackets[0])
                > calculate_result(&packet.subpackets[1])
            {
                1
            } else {
                0
            };
        }
        6 => {
            value = if calculate_result(&packet.subpackets[0])
                < calculate_result(&packet.subpackets[1])
            {
                1
            } else {
                0
            };
        }
        7 => {
            value = if calculate_result(&packet.subpackets[0])
                == calculate_result(&packet.subpackets[1])
            {
                1
            } else {
                0
            };
        }
        _ => panic!("Failed to parse type."),
    }

    value
}

fn problem_2(input: &String) -> i64 {
    let mut packet = Packet::new();
    packet.parse(input, 0);

    calculate_result(&packet)
}
