use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct BingoCard {
    numbers: Vec<u32>,
    hits: u32,
    winning_pos: Option<u32>,
    winning_score: u64,
}

impl From<&[String]> for BingoCard {
    fn from(data: &[String]) -> BingoCard {
        let mut numbers = Vec::new();

        for line in data {
            let c: Vec<u32> = line
                .split(" ")
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            numbers.push(c);
        }

        let hits = 0;
        let numbers = numbers.concat();
        BingoCard {
            numbers,
            hits,
            winning_pos: None,
            winning_score: 0,
        }
    }
}

impl BingoCard {
    fn mark_number(&mut self, num: u32) {
        let position = self.numbers.iter().position(|&x| x == num);

        match position {
            Some(pos) => {
                self.hits |= 1 << pos;
            }
            None => (),
        };
    }

    fn calculate_score(&self) -> u64 {
        let mut result: u64 = 0;
        for (idx, card_number) in self.numbers.iter().enumerate() {
            if self.hits & (1 << idx) == 0 {
                result += *card_number as u64
            }
        }

        result
    }

    fn set_winning_position(&mut self, num: u32) {
        self.winning_pos = Some(num);
        self.winning_score = self.calculate_score();
    }

    fn already_won(&self) -> bool {
        self.winning_pos.is_some()
    }
}

#[derive(Debug)]
struct Bingo {
    cards: Vec<BingoCard>,
    balls: Vec<u32>,
    prizes: HashMap<u32, [u32; 5]>,
    last_played_ball: Option<u32>,
}

impl Bingo {
    pub fn new(balls: Vec<u32>, cards: Vec<BingoCard>) -> Bingo {
        let prizes = HashMap::from([
            (31, [0, 1, 2, 3, 4]),            // 1st line
            (992, [5, 6, 7, 8, 9]),           // 2nd line
            (31744, [10, 11, 12, 13, 14]),    // 3rd line
            (1015808, [15, 16, 17, 18, 19]),  // 4th line
            (32505856, [20, 21, 22, 23, 24]), // 5th line
            (1082401, [0, 5, 10, 15, 20]),    // 1st column
            (2164802, [1, 6, 11, 16, 21]),    // 2nd column
            (4329604, [2, 7, 12, 17, 22]),    // 3rd column
            (8659208, [3, 8, 13, 18, 23]),    // 4th column
            (17318416, [4, 9, 14, 19, 24]),   // 5th column
        ]);

        Bingo {
            cards,
            balls,
            prizes,
            last_played_ball: None,
        }
    }

    fn play(&mut self) {
        let balls = &self.balls;
        let cards = &mut self.cards;

        for (ball_idx, ball) in balls.iter().enumerate() {
            for card in cards.iter_mut() {
                card.mark_number(*ball);

                self.last_played_ball = Some(*ball);

                let a = card.already_won();
                if card.already_won() {
                    continue;
                }

                match Bingo::has_prize(&self.prizes, card) {
                    Some(_) => card.set_winning_position(ball_idx as u32),
                    None => (),
                };
            }
        }
    }

    fn has_prize(prizes: &HashMap<u32, [u32; 5]>, card: &BingoCard) -> Option<u32> {
        for prize in prizes.iter() {
            if prize.0 & card.hits == *prize.0 {
                return Some(*prize.0);
            }
        }

        None
    }
}

fn parse_file(filename: &str) -> Bingo {
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

    // * Balls
    let balls: Vec<u32> = lines[0]
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    // * Cards
    let cards: Vec<BingoCard> = (2..lines.len())
        .step_by(6)
        .map(|x| BingoCard::from(&lines[x..x + 5]))
        .collect();

    Bingo::new(balls, cards)
}

fn solve(winning_card: &BingoCard, balls: &Vec<u32>) -> u64 {
    let last_ball = balls[winning_card.winning_pos.unwrap() as usize];
    winning_card.winning_score * last_ball as u64
}

fn main() {
    let mut input = parse_file("./inputs/input.txt");
    input.play();
    input.cards.sort_by_key(|a| a.winning_pos);

    println!("Problem 1: {}", solve(&input.cards[0], &input.balls));
    println!(
        "Problem 2: {}",
        solve(&input.cards.last().unwrap(), &input.balls)
    );
}
