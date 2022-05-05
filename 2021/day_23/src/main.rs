use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod types;
use types::{Amphipod, AmphipodType, HallWayPositions, Map, RoomPosition};

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
struct State {
    map: Map,
    total_cost: u32,
}

impl State {
    fn successors(&self) -> Vec<(State, u32)> {
        let mut possible_successors = Vec::new();

        // Collect all hallway positions
        let hallway_positions: HashSet<(usize, usize)> = self
            .map
            .hallway_positions
            .iter()
            .map(|x| x.position)
            .collect();

        // Amphipods's position
        let occupied_positions: HashSet<(usize, usize)> =
            self.map.amphipods.iter().map(|x| x.position).collect();

        // Get doors
        let room_denied_doors: HashSet<(usize, usize)> = self
            .map
            .rooms
            .iter()
            .map(|x| (x.position.unwrap().0, x.position.unwrap().1 - 1))
            .collect();

        for amphipod in &self.map.amphipods {
            let desired_room = self
                .map
                .rooms
                .iter()
                .filter(|x| x.amph_type == amphipod.own_type)
                .next()
                .unwrap();

            // if it's already in the desired room AND the state of the room is valid, do not touch it
            if desired_room.position.unwrap().0 == amphipod.position.0
                && desired_room.is_in_valid_state()
            {
                continue;
            }

            // Get possible locations
            let all_possible_positions = self.get_possible_locations(
                amphipod.position,
                &hallway_positions,
                &occupied_positions,
                &room_denied_doors,
                &desired_room,
            );

            let in_hallway = hallway_positions.contains(&amphipod.position);
            for new_position in &all_possible_positions {
                let mut cloned_state = self.clone();
                let current_position = amphipod.position;
                let is_desired_location_in_hallway = hallway_positions.contains(&new_position);

                cloned_state.map.update_positions(
                    in_hallway,
                    is_desired_location_in_hallway,
                    amphipod.own_type,
                    *new_position,
                    &amphipod,
                );

                let cost = Map::calculate_steps(&current_position, &new_position)
                    * AmphipodType::cost_per_move(&amphipod.own_type);
                cloned_state.total_cost += cost;
                possible_successors.push((cloned_state, cost));
            }
        }

        possible_successors
    }

    fn predict_cost(&self) -> u32 {
        let mut cost = 0;
        for amphipod in &self.map.amphipods {
            let desired_room = self
                .map
                .rooms
                .iter()
                .filter(|x| x.amph_type == amphipod.own_type)
                .next()
                .unwrap();

            if desired_room.position.unwrap().0 == amphipod.position.0
                && desired_room.is_in_valid_state()
            {
                continue;
            } else {
                cost += Map::calculate_steps(&amphipod.position, &desired_room.position.unwrap())
                    * AmphipodType::cost_per_move(&amphipod.own_type);
            }
        }
        cost
    }

    fn is_finished(&self) -> bool {
        let b = self.map.rooms.iter().all(|x| x.complete());
        b
    }

    fn get_possible_locations(
        &self,
        mut current_pos: (usize, usize),
        hallway_positions: &HashSet<(usize, usize)>,
        occupied_positions: &HashSet<(usize, usize)>,
        room_denied_doors: &HashSet<(usize, usize)>,
        desired_room: &RoomPosition,
    ) -> Vec<(usize, usize)> {
        let mut possible_locations = Vec::new();

        let min_col = hallway_positions.iter().min_by_key(|x| x.0).unwrap();
        let max_col = hallway_positions.iter().max_by_key(|x| x.0).unwrap();

        let is_in_hallway = hallway_positions.contains(&current_pos);
        if !is_in_hallway {
            // Check if can move out of the room
            let mut reached_hallway = false;
            loop {
                current_pos = (current_pos.0, current_pos.1 - 1);

                if hallway_positions.contains(&current_pos) {
                    reached_hallway = true;
                    break;
                }

                if occupied_positions.contains(&current_pos) {
                    break;
                }
            }

            if !reached_hallway {
                return possible_locations;
            }
        }

        // can reach or it is in hallway
        if is_in_hallway {
            if !desired_room.is_in_valid_state() {
                return possible_locations;
            }

            if self.valid_col_path_to(
                current_pos,
                &occupied_positions,
                desired_room.position.unwrap().0,
            ) {
                possible_locations.push(desired_room.get_rest_position());
                return possible_locations;
            } else {
                return possible_locations;
            }
        } else {
            // Check left moves
            let mut left = self.valid_paths(
                current_pos,
                min_col,
                &room_denied_doors,
                &occupied_positions,
            );
            // Check right moves
            let right = self.valid_paths(
                current_pos,
                max_col,
                &room_denied_doors,
                &occupied_positions,
            );
            // mutating
            left.extend(right);

            return left.into_iter().collect();
        }
    }

    fn valid_col_path_to(
        &self,
        mut cur_loc: (usize, usize),
        occ_pos: &HashSet<(usize, usize)>,
        desired_col: usize,
    ) -> bool {
        let incr_pos = cur_loc.0 < desired_col;

        loop {
            if cur_loc.0 == desired_col {
                break;
            }

            if incr_pos {
                cur_loc.0 += 1
            } else {
                cur_loc.0 -= 1;
            }

            if occ_pos.contains(&cur_loc) {
                return false;
            }
        }

        true
    }

    fn valid_paths(
        &self,
        mut cur_pos: (usize, usize),
        desired_pos: &(usize, usize),
        rooms_doors: &HashSet<(usize, usize)>,
        occ_pos: &HashSet<(usize, usize)>,
    ) -> HashSet<(usize, usize)> {
        let mut positions = HashSet::<(usize, usize)>::new();
        let pos_incr = cur_pos.0 < desired_pos.0;

        loop {
            let should_break = cur_pos.0 == desired_pos.0;
            if should_break {
                break;
            }

            if pos_incr {
                cur_pos.0 += 1;
            } else {
                cur_pos.0 -= 1;
            }

            if occ_pos.contains(&cur_pos) {
                break;
            }

            if !rooms_doors.contains(&cur_pos) {
                positions.insert(cur_pos);
            }
        }

        positions
    }
}

fn parse_file(filename: &str) -> State {
    let file = File::open(filename).expect("Failed to open file.");
    let reader = BufReader::new(file);

    let parsed: Vec<_> = reader.lines().map(|x| x.unwrap()).collect();

    let mut hallway_positions = Vec::<HallWayPositions>::new();
    let mut rooms = Vec::<RoomPosition>::new();
    let mut amphipods = Vec::<Amphipod>::new();

    rooms.push(RoomPosition::new(AmphipodType::Amber));
    rooms.push(RoomPosition::new(AmphipodType::Bronze));
    rooms.push(RoomPosition::new(AmphipodType::Copper));
    rooms.push(RoomPosition::new(AmphipodType::Desert));

    for (row_index, row) in parsed.iter().enumerate() {
        for (column_index, ch) in row.chars().enumerate() {
            if ch == '#' || ch == ' ' {
                continue;
            }

            if ch == '.' {
                hallway_positions.push(HallWayPositions {
                    position: (column_index, row_index),
                });
            } else {
                let room_index = amphipods.len() % 4;
                amphipods.push(Amphipod::new(
                    amphipods.len() as i32,
                    ch,
                    (column_index, row_index),
                ));

                let inserted_amph_type = amphipods.last().unwrap().own_type;
                rooms
                    .get_mut(room_index)
                    .unwrap()
                    .initial_add(inserted_amph_type, (column_index, row_index));
            }
        }
    }
    State {
        map: Map {
            hallway_positions,
            rooms,
            amphipods,
        },
        total_cost: 0,
    }
}

fn solve_minimum_path(filename: &str) -> u32 {
    // lib namespace
    use pathfinding::prelude::astar;
    let initial_state = parse_file(filename);

    let result = astar(
        &initial_state,
        |state| state.successors(),
        |state| state.predict_cost(),
        |state| state.is_finished(),
    );

    result.unwrap().1
}

fn main() {
    println!("Problem #1: {}", solve_minimum_path("./inputs/input.txt"));
    println!("Problem #2: {}", solve_minimum_path("./inputs/input_2.txt"));
}
