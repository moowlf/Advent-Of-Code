use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

// ----------------------------------------------------------------------------
// Data Structures

#[derive(Debug, Clone)]
struct Scanner {
    id: u32,
    beacons: Vec<Vec<(i32, i32, i32)>>,
    position: (i32, i32, i32),
    chosen_directory: i32,
}

impl Scanner {
    fn new(id: u32) -> Scanner {
        Scanner {
            id,
            beacons: vec![vec![]; 24],
            position: (i32::MIN, i32::MIN, i32::MIN),
            chosen_directory: -1,
        }
    }

    fn add_beacon(self: &mut Self, pos: (i32, i32, i32)) {
        self.beacons.get_mut(0).unwrap().push(pos);
        self.beacons
            .get_mut(1)
            .unwrap()
            .push((pos.0, pos.2, -pos.1));
        self.beacons
            .get_mut(2)
            .unwrap()
            .push((pos.0, -pos.1, -pos.2));
        self.beacons
            .get_mut(3)
            .unwrap()
            .push((pos.0, -pos.2, pos.1));
        self.beacons
            .get_mut(4)
            .unwrap()
            .push((-pos.0, -pos.1, pos.2));
        self.beacons
            .get_mut(5)
            .unwrap()
            .push((-pos.0, pos.2, pos.1));
        self.beacons
            .get_mut(6)
            .unwrap()
            .push((-pos.0, pos.1, -pos.2));
        self.beacons
            .get_mut(7)
            .unwrap()
            .push((-pos.0, -pos.2, -pos.1));

        self.beacons.get_mut(8).unwrap().push((pos.1, pos.2, pos.0));
        self.beacons
            .get_mut(9)
            .unwrap()
            .push((pos.1, pos.0, -pos.2));
        self.beacons
            .get_mut(10)
            .unwrap()
            .push((pos.1, -pos.2, -pos.0));
        self.beacons
            .get_mut(11)
            .unwrap()
            .push((pos.1, -pos.0, pos.2));
        self.beacons
            .get_mut(12)
            .unwrap()
            .push((-pos.1, -pos.2, pos.0));
        self.beacons
            .get_mut(13)
            .unwrap()
            .push((-pos.1, pos.0, pos.2));
        self.beacons
            .get_mut(14)
            .unwrap()
            .push((-pos.1, pos.2, -pos.0));
        self.beacons
            .get_mut(15)
            .unwrap()
            .push((-pos.1, -pos.0, -pos.2));

        self.beacons
            .get_mut(16)
            .unwrap()
            .push((pos.2, pos.0, pos.1));
        self.beacons
            .get_mut(17)
            .unwrap()
            .push((pos.2, pos.1, -pos.0));
        self.beacons
            .get_mut(18)
            .unwrap()
            .push((pos.2, -pos.0, -pos.1));
        self.beacons
            .get_mut(19)
            .unwrap()
            .push((pos.2, -pos.1, pos.0));
        self.beacons
            .get_mut(20)
            .unwrap()
            .push((-pos.2, -pos.0, pos.1));
        self.beacons
            .get_mut(21)
            .unwrap()
            .push((-pos.2, pos.1, pos.0));
        self.beacons
            .get_mut(22)
            .unwrap()
            .push((-pos.2, pos.0, -pos.1));
        self.beacons
            .get_mut(23)
            .unwrap()
            .push((-pos.2, -pos.1, -pos.0));
    }

    fn get_beacon_position(self: &Self, direction: usize, id: usize) -> &(i32, i32, i32) {
        self.beacons.get(direction).unwrap().get(id).unwrap()
    }

    fn does_match(self: &Self, scanner: &mut Scanner, min_needed_to_match: u32) -> bool {
        for our_beacon in self.beacons.get(self.chosen_directory as usize).unwrap() {
            for direction in 0..24 {
                let amount_of_beacons = scanner.beacons.get(direction).unwrap().len();

                for match_symbol_id in 0..amount_of_beacons {
                    let match_symbol = scanner
                        .beacons
                        .get(direction)
                        .unwrap()
                        .get(match_symbol_id)
                        .unwrap();

                    let translation = Scanner::negate_directions(match_symbol);
                    let new_scanner = (
                        our_beacon.0 + translation.0,
                        our_beacon.1 + translation.1,
                        our_beacon.2 + translation.2,
                    );

                    // Check other beacons based on new_scanner
                    let mut matched = 1;

                    for other_beacons_id in 0..amount_of_beacons {
                        if other_beacons_id == match_symbol_id {
                            continue;
                        }

                        let tb_translated = scanner
                            .beacons
                            .get(direction)
                            .unwrap()
                            .get(other_beacons_id)
                            .unwrap();
                        let possible_new_beacon = (
                            tb_translated.0 + new_scanner.0,
                            tb_translated.1 + new_scanner.1,
                            tb_translated.2 + new_scanner.2,
                        );

                        if self.contains(&possible_new_beacon) {
                            matched += 1;
                        }
                    }

                    if matched >= min_needed_to_match {
                        scanner.position = new_scanner;
                        scanner.chosen_directory = direction as i32;
                        scanner.translate_beacon();
                        return true;
                    }
                }
            }
        }

        false
    }

    fn contains(self: &Self, beacon: &(i32, i32, i32)) -> bool {
        self.beacons
            .get(self.chosen_directory as usize)
            .unwrap()
            .contains(beacon)
    }

    fn negate_directions(dir: &(i32, i32, i32)) -> (i32, i32, i32) {
        (-dir.0, -dir.1, -dir.2)
    }

    fn translate_beacon(self: &mut Self) {
        for beacon in self
            .beacons
            .get_mut(self.chosen_directory as usize)
            .unwrap()
        {
            *beacon = (
                beacon.0 + self.position.0,
                beacon.1 + self.position.1,
                beacon.2 + self.position.2,
            );
        }
    }
}

// ----------------------------------------------------------------------------
// Problems

fn problem_1(data: &mut Vec<Scanner>, minimum_to_match: u32) -> Vec<Scanner> {
    // We assume first scanner as the main direction
    let mut correctly_placed_scanner = Vec::new();
    data.get_mut(0).unwrap().position = (0, 0, 0);
    data.get_mut(0).unwrap().chosen_directory = 0;
    correctly_placed_scanner.push(data.get_mut(0).unwrap().clone());

    let mut placed_scanner_ids = HashSet::<usize>::new();
    placed_scanner_ids.insert(0);

    let mut scanner_id = 1;

    loop {
        if placed_scanner_ids.len() == data.len() {
            break;
        }

        scanner_id %= data.len();
        if placed_scanner_ids.contains(&scanner_id) {
            scanner_id += 1;
            continue;
        }

        let scanner = data.get_mut(scanner_id).unwrap();
        for placed_scanner in &correctly_placed_scanner {
            if placed_scanner.does_match(scanner, minimum_to_match) {
                correctly_placed_scanner.push(scanner.clone());
                placed_scanner_ids.insert(scanner_id);
                break;
            }
        }

        scanner_id += 1;
    }

    correctly_placed_scanner
}

fn problem_2(data: &mut Vec<Scanner>) -> i32 {
    let points: Vec<_> = data.iter().map(|x| x.position).collect();

    let mut max_distance = i32::MIN;

    for index in 0..points.len() {
        for index_1 in index + 1..points.len() {
            let point = points.get(index).unwrap();
            let point_1 = points.get(index_1).unwrap();

            let tmp_distance = (point.0 - point_1.0).abs()
                + (point.1 - point_1.1).abs()
                + (point.2 - point_1.2).abs();

            dbg!(point, point_1, tmp_distance);
            if max_distance < tmp_distance {
                max_distance = tmp_distance;
            }
        }
    }

    max_distance
}

// ----------------------------------------------------------------------------
// Main Functions

fn parse_file(filename: &str) -> Vec<Scanner> {
    let file = File::open(filename).expect("Failed to open file!");
    let reader = BufReader::new(file);

    // Regex helper
    let re = Regex::new(r"(\d+)").unwrap();

    let mut scanners: Vec<Scanner> = Vec::new();

    for line in reader.lines() {
        let line_value = line.unwrap();
        if line_value.contains("--") {
            // --- scanner X ---
            let id_str = re.captures(&line_value).unwrap().get(1);
            let id: u32 = id_str.unwrap().as_str().parse().unwrap();

            scanners.push(Scanner::new(id));
        } else if line_value.len() != 0 {
            // \d+,\d+,\d+

            let coords_vec: Vec<i32> = line_value
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            let coords = (
                *coords_vec.get(0).unwrap_or(&0),
                *coords_vec.get(1).unwrap_or(&0),
                *coords_vec.get(2).unwrap_or(&0),
            );
            scanners.last_mut().unwrap().add_beacon(coords);
        }
    }

    scanners
}

fn main() {
    let mut input = parse_file("./inputs/input.txt");

    let solved = problem_1(&mut input, 12);

    let problem_1_solved = solved
        .iter()
        .flat_map(|x| x.beacons.get(x.chosen_directory as usize).unwrap())
        .collect::<HashSet<&(i32, i32, i32)>>()
        .len();

    println!("Problem #1: {}", problem_1_solved);
    println!("Problem #2: {}", problem_2(&mut input));
}
