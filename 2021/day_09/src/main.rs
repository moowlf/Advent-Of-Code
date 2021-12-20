use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = parse_file("./inputs/input.txt");

    let (solution_1, low_points) = problem_1(&input);
    println!("Problem 1: {}", solution_1);
    println!("Problem 2: {}", problem_2(&input, &low_points));
}

fn parse_file(filename: &str) -> Vec<Vec<u32>> {
    let file = File::open(filename).expect("Failed to open file.");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split("")
                .filter(|&x| !x.is_empty())
                .map(|c| c.parse().unwrap())
                .collect()
        })
        .collect()
}

fn get_adjacent_location(matrix: &Vec<Vec<u32>>, curr_x: u32, curr_y: u32) -> Vec<(u32, u32)> {
    let mut vec = Vec::new();

    let max_width: u32 = matrix[0].len() as u32 - 1;
    let max_height: u32 = matrix.len() as u32 - 1;

    if curr_x != 0 {
        vec.push((curr_x - 1, curr_y));
    }

    if curr_x != max_width {
        vec.push((curr_x + 1, curr_y));
    }

    if curr_y != 0 {
        vec.push((curr_x, curr_y - 1));
    }

    if curr_y != max_height {
        vec.push((curr_x, curr_y + 1));
    }

    vec
}

fn problem_1(input: &Vec<Vec<u32>>) -> (u32, Vec<(u32, u32)>) {
    let mut risk_level: u32 = 0;
    let mut low_points = Vec::new();

    for y_index in 0..input.len() {
        for x_index in 0..input[y_index].len() {
            let adj_locations = get_adjacent_location(input, x_index as u32, y_index as u32);

            if adj_locations
                .iter()
                .all(|c| input[c.1 as usize][c.0 as usize] > input[y_index][x_index])
            {
                risk_level += input[y_index][x_index] + 1;
                low_points.push((x_index as u32, y_index as u32));
            }
        }
    }

    (risk_level, low_points)
}

fn problem_2(input: &Vec<Vec<u32>>, low_points: &Vec<(u32, u32)>) -> u32 {
    let mut result: Vec<u32> = Vec::new();
    for low_point in low_points.iter() {
        // Coords used
        let mut basin_set: HashSet<(u32, u32)> = HashSet::new();
        basin_set.insert(*low_point);

        // Current stack
        let mut basin_cur_stack: VecDeque<(u32, u32)> = VecDeque::new();
        basin_cur_stack.push_front(*low_point);

        // Cycle
        loop {
            // Get current location
            let current_location = basin_cur_stack.pop_front().unwrap();
            basin_set.insert(current_location);

            let adj_locations =
                get_adjacent_location(input, current_location.0, current_location.1);

            for next_location in adj_locations.iter() {
                if input[next_location.1 as usize][next_location.0 as usize] == 9 {
                    continue;
                }

                let has_continuity = input[next_location.1 as usize][next_location.0 as usize]
                    as i32
                    > input[current_location.1 as usize][current_location.0 as usize] as i32;

                if has_continuity {
                    if !basin_set.contains(&next_location) {
                        basin_cur_stack.push_front(*next_location);
                    }
                }
            }

            if basin_cur_stack.len() == 0 {
                result.push(basin_set.len() as u32);
                break;
            }
        }
    }

    result.sort();

    result[result.len() - 3] * result[result.len() - 2] * result[result.len() - 1]
}
