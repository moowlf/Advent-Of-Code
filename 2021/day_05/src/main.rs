use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Wind {
    initial_position: (i32, i32),
    final_position: (i32, i32),
    current_position: (i32, i32),
    direction: (i32, i32),
}

impl Wind {
    fn next(&mut self) -> Option<(i32, i32)> {
        if self.current_position == self.final_position {
            return None;
        }

        let next = (
            self.current_position.0 + self.direction.0,
            self.current_position.1 + self.direction.1,
        );

        Some(next)
    }

    fn calculate_direction(initial: &(i32, i32), end: &(i32, i32)) -> (i32, i32) {
        let mut direction = (end.0 - initial.0, end.1 - initial.1);

        if direction.0 != 0 {
            direction.0 /= direction.0.abs();
        }

        if direction.1 != 0 {
            direction.1 /= direction.1.abs();
        }

        direction
    }
}

fn main() {
    let input = parse_file("./inputs/input.txt");
    println!("Problem 1: {}", problem_1(&mut input.clone()));
    println!("Problem 2: {}", problem_2(&mut input.clone()));
}

fn parse_file(filename: &str) -> Vec<Wind> {
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let l = line.unwrap();
            let coordinates: Vec<&str> = l.split(" -> ").collect();

            let initial_coords: Vec<i32> = coordinates[0]
                .split(",")
                .map(|x| x.parse().unwrap())
                .collect();

            let final_coords: Vec<i32> = coordinates[1]
                .split(",")
                .map(|x| x.parse().unwrap())
                .collect();

            let initial_coords = (initial_coords[0], initial_coords[1]);
            let final_coords = (final_coords[0], final_coords[1]);

            Wind {
                initial_position: initial_coords,
                final_position: final_coords,
                current_position: initial_coords,
                direction: Wind::calculate_direction(&initial_coords, &final_coords),
            }
        })
        .collect()
}

fn problem_1(input: &mut Vec<Wind>) -> u32 {
    let mut wind_points: HashMap<(i32, i32), u32> = HashMap::new();

    input
        .iter_mut()
        .filter(|x| !(x.direction.0 != 0 && x.direction.1 != 0))
        .for_each(|x| loop {
            let has_point = wind_points.get_mut(&x.current_position);

            match has_point {
                Some(num_times) => *num_times += 1,
                None => {
                    wind_points.insert(x.current_position, 1);
                }
            }

            let next = x.next();

            if next.is_none() {
                break;
            }

            x.current_position = next.unwrap();
        });

    wind_points.iter().filter(|x| *x.1 > 1).count() as u32
}

fn problem_2(input: &mut Vec<Wind>) -> u32 {
    let mut wind_points: HashMap<(i32, i32), u32> = HashMap::new();

    input.iter_mut().for_each(|x| loop {
        let has_point = wind_points.get_mut(&x.current_position);

        match has_point {
            Some(num_times) => *num_times += 1,
            None => {
                wind_points.insert(x.current_position, 1);
            }
        }

        let next = x.next();

        if next.is_none() {
            break;
        }

        x.current_position = next.unwrap();
    });

    wind_points.iter().filter(|x| *x.1 > 1).count() as u32
}
