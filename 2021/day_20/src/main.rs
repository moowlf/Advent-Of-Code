use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Input {
    algorithm: String,
    input_image: HashMap<(i32, i32), bool>,
    iteration: u32,
}

impl Input {
    fn convert_vec_to_hashset(vec: &Vec<String>) -> HashMap<(i32, i32), bool> {
        let mut hash_map = HashMap::new();

        for row in 0..vec.len() {
            let row_data = vec.get(row).unwrap();
            for column in 0..row_data.len() {
                hash_map.insert(
                    (column as i32, row as i32),
                    row_data.chars().nth(column).unwrap() == '#',
                );
            }
        }

        hash_map
    }

    fn get_neighbours(column: i32, row: i32) -> Vec<(i32, i32)> {
        let data = vec![
            (column - 1, row - 1),
            (column, row - 1),
            (column + 1, row - 1),
            (column - 1, row),
            (column, row),
            (column + 1, row),
            (column - 1, row + 1),
            (column, row + 1),
            (column + 1, row + 1),
        ];

        data
    }

    fn calculate_index(self: &Self, neighbours: &Vec<(i32, i32)>) -> usize {
        let mut bin: usize = 0;
        let current_state_of_outside = if self.algorithm.chars().nth(0).unwrap() == '#'
            && self.algorithm.chars().last().unwrap() == '.'
            && self.iteration % 2 != 0
        {
            1
        } else {
            0
        };

        let mut current_index = 8;
        for neighbour in neighbours {
            let neigh = self.input_image.get(neighbour);

            let number_to_shift = match neigh {
                None => current_state_of_outside,
                Some(x) => usize::from(*x),
            };

            bin |= number_to_shift << current_index;
            current_index -= 1;
        }

        bin
    }

    fn should_light_up(self: &Self, neighbours: &Vec<(i32, i32)>) -> bool {
        let bin = self.calculate_index(&neighbours);
        self.algorithm.chars().nth(bin).unwrap() == '#'
    }

    fn retrieve_next_gen(self: &Self) -> HashMap<(i32, i32), bool> {
        //* Get Active Cells + Possible affected cells
        let mut cells_to_watch = self
            .input_image
            .iter()
            .map(|x| *x.0)
            .collect::<HashSet<(i32, i32)>>();

        for neighbour in &self.input_image {
            let neighbours = Input::get_neighbours(neighbour.0 .0, neighbour.0 .1);
            for neigh in &neighbours {
                cells_to_watch.insert(*neigh);
            }
        }

        //* Try to come up with next iteration
        let mut next_iteration = HashMap::<(i32, i32), bool>::new();

        for cell_to_watch in &cells_to_watch {
            let neighbours = Input::get_neighbours(cell_to_watch.0, cell_to_watch.1);
            let should_light_up = self.should_light_up(&neighbours);
            next_iteration.insert((cell_to_watch.0, cell_to_watch.1), should_light_up);
        }

        next_iteration
    }
}

fn problem_1(data: &mut Input) -> usize {
    for _ in 0..2 {
        //* Update next iteration
        data.input_image = data.retrieve_next_gen();
        data.iteration += 1;
    }

    data.input_image
        .iter()
        .filter(|x| *x.1 == true)
        .collect::<HashMap<_, _>>()
        .len()
}

fn problem_2(data: &mut Input) -> usize {
    for _ in 0..50 {
        //* Update next iteration
        data.input_image = data.retrieve_next_gen();
        data.iteration += 1;
    }

    data.input_image
        .iter()
        .filter(|x| *x.1 == true)
        .collect::<HashMap<_, _>>()
        .len()
}

fn parse_file(filename: &str) -> Input {
    let file = File::open(filename).expect("Failed to open file.");
    let reader = BufReader::new(file);

    let mut parsed: Vec<_> = reader
        .lines()
        .map(|x| x.unwrap())
        .filter(|x| !x.is_empty())
        .collect();

    let algorithm = parsed.get(0).unwrap().clone();
    parsed.remove(0);
    let input_image = Input::convert_vec_to_hashset(&parsed);

    Input {
        algorithm,
        input_image,
        iteration: 0,
    }
}

fn main() {
    let input = parse_file("./inputs/input.txt");
    println!("Problem 1: {}", problem_1(&mut input.clone()));
    println!("Problem 2: {}", problem_2(&mut input.clone()));
}
