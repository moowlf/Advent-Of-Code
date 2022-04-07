use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_file(filename: &str) -> Vec<u32> {
    let file = File::open(filename).expect("Failed to open file.");
    let reader = BufReader::new(file);

    lazy_static! {
        static ref RE: Regex = Regex::new(r".*:\s(\d+)").unwrap();
    }

    let parsed: Vec<u32> = reader
        .lines()
        .map(|x| {
            RE.captures(&x.unwrap())
                .unwrap()
                .get(1)
                .map(|x| x.as_str())
                .unwrap()
                .parse::<u32>()
                .unwrap()
        })
        .collect();

    parsed
}

fn problem_1(data: &Vec<u32>) -> u32 {
    let mut current_score_player_0 = 0;
    let mut current_score_player_1 = 0;

    let mut player_0_position = data.get(0).unwrap().to_owned();
    let mut player_1_position = data.get(1).unwrap().to_owned();

    let mut current_index = 1;
    let mut roll_id = 0;

    loop {
        let first_roll = (current_index + 1) * 3;
        player_0_position = (player_0_position - 1 + first_roll) % 10 + 1;
        current_score_player_0 += player_0_position;
        roll_id += 3;

        if current_score_player_0 >= 1000 {
            return current_score_player_1 * roll_id;
        }

        let second_roll = (current_index + 4) * 3;
        player_1_position = (player_1_position - 1 + second_roll) % 10 + 1;
        current_score_player_1 += player_1_position;
        roll_id += 3;

        if current_score_player_1 >= 1000 {
            return current_score_player_0 * roll_id;
        }

        current_index += 6;
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct State {
    player_1_position: u8,
    player_1_points: u8,
    player_2_position: u8,
    player_2_points: u8,
    player_1_turn: bool,
}

impl State {
    fn new(p1_pos: u8, p1_points: u8, p2_pos: u8, p2_points: u8, turn: bool) -> State {
        Self {
            player_1_position: p1_pos,
            player_1_points: p1_points,
            player_2_position: p2_pos,
            player_2_points: p2_points,
            player_1_turn: turn,
        }
    }
}

fn problem_2_dp(
    current_state: &State,
    cache: &mut HashMap<State, (u64, u64)>,
    sums: &Vec<(u8, u8)>,
) -> (u64, u64) {
    let mut wins = cache.get(&current_state).unwrap().clone();

    for (dice_sum, times) in sums {
        let mut new_state = current_state.clone();
        if current_state.player_1_turn {
            new_state.player_1_position = (new_state.player_1_position - 1 + dice_sum) % 10 + 1;
            new_state.player_1_points =
                std::cmp::min(new_state.player_1_points + new_state.player_1_position, 21);
        } else {
            new_state.player_2_position = (new_state.player_2_position - 1 + dice_sum) % 10 + 1;
            new_state.player_2_points =
                std::cmp::min(new_state.player_2_points + new_state.player_2_position, 21);
        }

        if current_state.player_1_turn && new_state.player_1_points >= 21 {
            wins.0 += *times as u64;
            continue;
        } else if !current_state.player_1_turn && new_state.player_2_points >= 21 {
            wins.1 += *times as u64;
            continue;
        }

        new_state.player_1_turn = !new_state.player_1_turn;

        if cache.contains_key(&new_state) {
            let cache_state = cache.get(&new_state).unwrap();
            wins.0 += cache_state.0 * (*times) as u64;
            wins.1 += cache_state.1 * (*times) as u64;
        } else {
            cache.insert(new_state.clone(), (0, 0));
            let result = problem_2_dp(&new_state, cache, &sums);
            wins.0 += result.0 * (*times) as u64;
            wins.1 += result.1 * (*times) as u64;
        }
    }

    *cache.get_mut(&current_state).unwrap() = wins;
    wins
}

fn problem_2(data: &Vec<u32>) -> u64 {
    let initial_player_1_position = data.get(0).unwrap().to_owned() as u8;
    let initial_player_2_position = data.get(1).unwrap().to_owned() as u8;

    let possible_adds = vec![
        /*
        (2, 1),
        (3, 2),
        (4, 1),*/
        (3, 1), // 1,1,1
        (4, 3), // 1,1,2
        (5, 6), // 1,2,2 or 1,1,3
        (6, 7), // 2,2,2 or 1,2,3
        (7, 6), // 2,2,3 or 1,3,3
        (8, 3), // 2,3,3
        (9, 1), // 3,3,3
    ];

    let initial_state = State::new(
        initial_player_1_position,
        0,
        initial_player_2_position,
        0,
        true,
    );
    let mut cache = HashMap::<State, (u64, u64)>::new();
    cache.insert(initial_state.clone(), (0, 0));

    let result = problem_2_dp(&initial_state, &mut cache, &possible_adds);
    std::cmp::max(result.0, result.1)
}

fn main() {
    let input = parse_file("./inputs/input.txt");
    println!("Problem 1: {}", problem_1(&mut input.clone()));
    println!("Problem 2: {}", problem_2(&input));
}
