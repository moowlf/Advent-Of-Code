use std::collections::VecDeque;

#[derive(Hash, Copy, Clone, Debug, PartialEq, Eq)]
pub enum AmphipodType {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl AmphipodType {
    pub fn from_char(ch: char) -> AmphipodType {
        match ch {
            'A' => AmphipodType::Amber,
            'B' => AmphipodType::Bronze,
            'C' => AmphipodType::Copper,
            'D' => AmphipodType::Desert,
            _ => panic!(""),
        }
    }

    pub fn cost_per_move(amph_type: &AmphipodType) -> u32 {
        match amph_type {
            AmphipodType::Amber => 1,
            AmphipodType::Bronze => 10,
            AmphipodType::Copper => 100,
            AmphipodType::Desert => 1000,
        }
    }
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct Amphipod {
    pub id: i32,
    pub own_type: AmphipodType,
    pub position: (usize, usize),
}

impl Amphipod {
    pub fn new(id: i32, ch: char, position: (usize, usize)) -> Amphipod {
        let own_type = AmphipodType::from_char(ch);
        Amphipod {
            id,
            own_type,
            position,
        }
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct HallWayPositions {
    pub position: (usize, usize),
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct RoomPosition {
    pub amph_type: AmphipodType,
    pub position: Option<(usize, usize)>,
    pub content: VecDeque<AmphipodType>,
    pub max_size: u32,
}

impl RoomPosition {
    pub fn new(amph_type: AmphipodType) -> RoomPosition {
        RoomPosition {
            amph_type,
            position: None,
            content: VecDeque::<AmphipodType>::new(),
            max_size: 0,
        }
    }

    pub fn initial_add(&mut self, tp: AmphipodType, position: (usize, usize)) {
        if self.position.is_none() {
            self.position = Some(position)
        }

        self.content.push_back(tp);
        self.max_size += 1;
    }

    pub fn add(&mut self, tp: AmphipodType) {
        self.content.push_front(tp);
    }

    pub fn is_in_valid_state(&self) -> bool {
        self.content.iter().all(|x| *x == self.amph_type)
    }

    pub fn complete(&self) -> bool {
        self.is_in_valid_state() && self.max_size as usize == self.content.len()
    }

    pub fn get_rest_position(&self) -> (usize, usize) {
        (
            self.position.unwrap().0,
            self.position.unwrap().1 + self.max_size as usize - self.content.len() - 1,
        )
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Map {
    pub hallway_positions: Vec<HallWayPositions>,
    pub rooms: Vec<RoomPosition>,
    pub amphipods: Vec<Amphipod>,
}

impl Map {
    fn get_room_by_column(&mut self, column: usize) -> &mut RoomPosition {
        self.rooms
            .iter_mut()
            .find(|x| x.position.unwrap().0 == column)
            .unwrap()
    }

    pub fn calculate_steps(from: &(usize, usize), to: &(usize, usize)) -> u32 {
        ((to.0 as i32 - from.0 as i32).abs() + (to.1 as i32 - from.1 as i32).abs()) as u32
    }

    pub fn update_positions(
        &mut self,
        in_hallway: bool,
        to_hallway: bool,
        amph_type: AmphipodType,
        pos: (usize, usize),
        ig_amph: &Amphipod,
    ) {
        if !to_hallway {
            self.get_room_by_column(pos.0).add(amph_type);
        }

        if !in_hallway {
            self.get_room_by_column(ig_amph.position.0)
                .content
                .pop_front();
        }

        self.amphipods
            .iter_mut()
            .find(|x| *x == ig_amph)
            .unwrap()
            .position = pos;
    }
}
