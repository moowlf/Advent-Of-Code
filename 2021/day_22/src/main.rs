use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Cube {
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}

#[derive(Debug, Clone)]
struct Rule {
    status: bool,
    cube: Cube,
}

impl Cube {
    fn intersects(self: &Self, rhs: &Cube) -> bool {
        return Cube::intersect_coordinates(&self.x, &rhs.x)
            && Cube::intersect_coordinates(&self.y, &rhs.y)
            && Cube::intersect_coordinates(&self.z, &rhs.z);
    }

    fn intersect_coordinates(a: &(i32, i32), b: &(i32, i32)) -> bool {
        if b.1 < a.0 || b.0 > a.1 {
            return false;
        }

        true
    }
}

impl Rule {
    fn create_rule_from_intersect(to_insert: &Rule, inserted: &Rule) -> Rule {
        Rule {
            status: !inserted.status,
            cube: Cube {
                x: (
                    std::cmp::max(to_insert.cube.x.0, inserted.cube.x.0),
                    std::cmp::min(to_insert.cube.x.1, inserted.cube.x.1),
                ),
                y: (
                    std::cmp::max(to_insert.cube.y.0, inserted.cube.y.0),
                    std::cmp::min(to_insert.cube.y.1, inserted.cube.y.1),
                ),
                z: (
                    std::cmp::max(to_insert.cube.z.0, inserted.cube.z.0),
                    std::cmp::min(to_insert.cube.z.1, inserted.cube.z.1),
                ),
            },
        }
    }
}

fn parse_file(filename: &str) -> Vec<Rule> {
    let file = File::open(filename).expect("Failed to open file.");
    let reader = BufReader::new(file);

    let re =
        Regex::new(r"(on|off)\s(x=(-?\d+)..(-?\d+)),(y=(-?\d+)..(-?\d+)),(z=(-?\d+)..(-?\d+))")
            .unwrap();
    let parsed: Vec<_> = reader
        .lines()
        .map(|x| {
            let line = x.unwrap();
            let d = re.captures(&line).unwrap();
            Rule {
                status: &d[1] == "on",
                cube: Cube {
                    x: (d[3].parse().unwrap(), d[4].parse().unwrap()),
                    y: (d[6].parse().unwrap(), d[7].parse().unwrap()),
                    z: (d[9].parse().unwrap(), d[10].parse().unwrap()),
                },
            }
        })
        .collect();

    parsed
}

fn solve(data: &Vec<Rule>) -> Vec<Rule> {
    let mut rules: Vec<Rule> = vec![data.get(0).unwrap().clone()];

    for rule in data.iter().skip(1) {
        // Check for overlapping areas
        for rule_id in 0..rules.len() {
            let rule_already_inserted = rules.get(rule_id).unwrap();

            if !rule.cube.intersects(&rule_already_inserted.cube) {
                continue;
            }

            let intersection = Rule::create_rule_from_intersect(&rule, rule_already_inserted);

            // Deal with intersection
            rules.push(intersection);
        }

        if rule.status {
            rules.push(rule.clone());
        }
    }

    rules
}

fn supress_coordinates(coordinates: &(i32, i32)) -> (i32, i32) {
    let valid_coordinates = (-50, 50);

    if Cube::intersect_coordinates(coordinates, &valid_coordinates) {
        return (
            std::cmp::max(valid_coordinates.0, coordinates.0),
            std::cmp::min(valid_coordinates.1, coordinates.1),
        );
    }

    (0, -1) // to give 0 as distance [ max - min + 1]
}

fn problem_1(data: &Vec<Rule>) -> i64 {
    let rules = solve(data);
    let mut count: i64 = 0;

    for rule in &rules {
        let x = supress_coordinates(&rule.cube.x);
        let y = supress_coordinates(&rule.cube.y);
        let z = supress_coordinates(&rule.cube.z);

        let x_dist = (x.1 - x.0 + 1).abs() as i64;
        let y_dist = (y.1 - y.0 + 1).abs() as i64;
        let z_dist = (z.1 - z.0 + 1).abs() as i64;

        let vol = x_dist * y_dist * z_dist;
        if !rule.status {
            count -= vol;
        } else {
            count += vol;
        }
    }

    count
}

fn problem_2(data: &Vec<Rule>) -> i64 {
    let rules = solve(data);
    let mut count: i64 = 0;

    for rule in &rules {
        let x_dist = (rule.cube.x.1 - rule.cube.x.0 + 1).abs() as i64;
        let y_dist = (rule.cube.y.1 - rule.cube.y.0 + 1).abs() as i64;
        let z_dist = (rule.cube.z.1 - rule.cube.z.0 + 1).abs() as i64;

        let vol = x_dist * y_dist * z_dist;
        if !rule.status {
            count -= vol;
        } else {
            count += vol;
        }
    }

    count
}

fn main() {
    let input_data = parse_file("./inputs/input.txt");
    println!("Problem 1: {}", problem_1(&input_data));
    println!("Problem 2: {}", problem_2(&input_data));
}
