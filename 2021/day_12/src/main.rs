use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Graph {
    nodes: Vec<String>,
    connections: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            connections: HashMap::new(),
        }
    }

    fn add_node(&mut self, data: &String) {
        if !self.nodes.contains(data) {
            self.nodes.push(data.to_string());
        }
    }

    fn add_connection(&mut self, node_name: &String, node_to_name: &String) {
        let connections = self.connections.get_mut(node_name);

        match connections {
            Some(a) => {
                a.push(node_to_name.to_string());
            }
            None => {
                self.connections
                    .insert(node_name.to_string(), vec![node_to_name.to_string()]);
            }
        }

        // Add reverse connection

        let inverse_connection = self.connections.get_mut(node_to_name);

        match inverse_connection {
            Some(a) => {
                a.push(node_name.to_string());
            }
            None => {
                self.connections
                    .insert(node_to_name.to_string(), vec![node_name.to_string()]);
            }
        }
    }
}

fn main() {
    let input = parse_file("./inputs/input.txt");
    println!("Problem 1: {}", problem_1(&input));
    println!("Problem 2: {}", problem_2(&input));
}

fn parse_file(filename: &str) -> Graph {
    let file = File::open(filename).expect("Failed to open file.");
    let reader = BufReader::new(file);

    let mut graph = Graph::new();

    reader.lines().for_each(|line| {
        let data: Vec<String> = line.unwrap().split("-").map(|c| c.to_string()).collect();

        let origin = &data[0];
        let destination = &data[1];

        graph.add_node(&origin);
        graph.add_connection(origin, destination);
    });

    graph
}

fn problem_2(input: &Graph) -> u64 {
    let mut possible_paths = VecDeque::<VecDeque<&str>>::new();
    possible_paths.push_back(VecDeque::from(["start"]));
    let mut number_distincts_paths = 0;

    loop {
        // * End prematurely if there is no more paths
        if possible_paths.len() == 0 {
            break;
        }

        // * get current path
        let current_path = possible_paths.pop_front().unwrap();
        let last_cave = *current_path.back().unwrap();

        if last_cave == "end" {
            number_distincts_paths += 1;
            println!("{:?}", current_path);
            continue;
        }

        // * Get childs
        let current_path_childs = input.connections.get(last_cave).unwrap();

        // * Create new paths
        for child in current_path_childs.iter() {
            if child == "start" {
                continue;
            }

            let is_all_uppercase = child.chars().all(|x| x.is_uppercase());

            if !is_all_uppercase {
                // * Since it's not an all uppercase cave, let's check if we
                // * already been in there

                let mut elem_count = HashMap::<&str, u32>::new();

                for elem in current_path.iter() {
                    *elem_count.entry(elem).or_insert(0) += 1;
                }

                if elem_count.contains_key(&child[..]) {
                    let has_double_cave = elem_count.iter().any(|(key, value)| {
                        if key.chars().all(|c| c.is_lowercase()) {
                            return *value >= 2;
                        }

                        false
                    });

                    if has_double_cave {
                        continue;
                    }
                }
            }

            let mut new_path = current_path.clone();
            new_path.push_back(child);

            possible_paths.push_back(new_path);
        }
    }

    number_distincts_paths
}

fn problem_1(input: &Graph) -> u64 {
    let mut possible_paths = VecDeque::<VecDeque<&str>>::new();
    possible_paths.push_back(VecDeque::from(["start"]));
    let mut number_distincts_paths = 0;

    loop {
        // * End prematurely if there is no more paths
        if possible_paths.len() == 0 {
            break;
        }

        // * get current path
        let current_path = possible_paths.pop_front().unwrap();
        let last_cave = *current_path.back().unwrap();

        if last_cave == "end" {
            number_distincts_paths += 1;
            continue;
        }

        // * Get childs
        let current_path_childs = input.connections.get(last_cave).unwrap();

        // * Create new paths
        for child in current_path_childs.iter() {
            let is_all_uppercase = child.chars().all(|x| x.is_uppercase());

            if !is_all_uppercase {
                // * Since it's not an all uppercase cave, let's check if we
                // * already been in there

                if current_path.contains(&&child[0..]) {
                    // * we already visited it. Not a valid route
                    continue;
                }
            }

            let mut new_path = current_path.clone();
            new_path.push_back(child);

            possible_paths.push_back(new_path);
        }
    }

    number_distincts_paths
}
