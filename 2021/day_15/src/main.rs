use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = parse_file("./inputs/input.txt");
    println!("Problem 1: {}", problem_1(&mut input.clone()));
    println!("Problem 2: {}", problem_2(&mut input.clone()));
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

fn get_adjacent_location(input: &Vec<Vec<u32>>, x_pos: usize, y_pos: usize) -> Vec<(usize, usize)> {
    let mut vec = Vec::new();

    let max_width: usize = input[0].len() - 1;
    let max_height: usize = input.len() - 1;

    if x_pos != 0 {
        vec.push((x_pos - 1, y_pos));
    }

    if x_pos != max_width {
        vec.push((x_pos + 1, y_pos));
    }

    if y_pos != 0 {
        vec.push((x_pos, y_pos - 1));
    }

    if y_pos != max_height {
        vec.push((x_pos, y_pos + 1));
    }

    vec
}

struct HeapNode {
    distance: u64,
    current_loc: (usize, usize),
}

impl PartialEq for HeapNode {
    fn eq(&self, other: &Self) -> bool {
        if self.distance == other.distance {
            true
        } else {
            false
        }
    }
}

impl Eq for HeapNode {}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        let me = &(self.distance);
        let them = &(other.distance);

        if me > them {
            Ordering::Less
        } else if me < them {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl HeapNode {
    fn new(distance: u64, current_loc: (usize, usize)) -> HeapNode {
        Self {
            distance,
            current_loc,
        }
    }
}

fn problem_1(input: &mut Vec<Vec<u32>>) -> u64 {
    let matrix_width = input[0].len();
    let matrix_height = input.len();

    let mut heap: BinaryHeap<HeapNode> = BinaryHeap::new();
    let mut vertex_distances = vec![u64::MAX; matrix_height * matrix_width];

    heap.push(HeapNode::new(0, (0, 0)));
    vertex_distances[0] = input[0][0] as u64;

    loop {
        if heap.is_empty() {
            break;
        }

        let node = heap.pop().unwrap();
        let coord_id = matrix_width * node.current_loc.1 + node.current_loc.0;

        if coord_id == matrix_height * matrix_width - 1 {
            break;
        }

        let adjacent_locations =
            get_adjacent_location(input, node.current_loc.0, node.current_loc.1);

        for loc in adjacent_locations.iter() {
            let loc_weig = input[loc.1][loc.0] as u64;
            let loc_id = matrix_width * loc.1 + loc.0;

            if vertex_distances[loc_id] > node.distance + loc_weig {
                vertex_distances[loc_id] = node.distance + loc_weig;
                heap.push(HeapNode::new(vertex_distances[loc_id], *loc))
            }
        }
    }

    *vertex_distances.last().unwrap()
}

fn clamp(value: u32, min: u32, max: u32) -> u32 {
    if value < min {
        min
    } else if value > max {
        min
    } else {
        value
    }
}

fn problem_2(input: &mut Vec<Vec<u32>>) -> u64 {
    let witdh = input[0].len();
    let height = input.len();

    for index in 0..height {
        input[index].resize(witdh * 5, 0 as u32);

        for w_index in 0..witdh {
            input[index][w_index + 1 * witdh] = clamp(input[index][w_index + 0 * witdh] + 1, 1, 9);
            input[index][w_index + 2 * witdh] = clamp(input[index][w_index + 1 * witdh] + 1, 1, 9);
            input[index][w_index + 3 * witdh] = clamp(input[index][w_index + 2 * witdh] + 1, 1, 9);
            input[index][w_index + 4 * witdh] = clamp(input[index][w_index + 3 * witdh] + 1, 1, 9);
        }
    }

    input.resize(height * 5, vec![]);

    for index in height..input.len() {
        input[index].resize(witdh * 5, 0 as u32);

        for w_index in 0..input[index].len() {
            input[index][w_index] = clamp(input[index - height][w_index] + 1, 1, 9);
        }
    }

    problem_1(input)
}
