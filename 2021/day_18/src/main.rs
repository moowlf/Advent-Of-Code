use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;

fn main() {
    let data = parse_file("./inputs/input.txt");
    println!("Problem 1: {}", problem_1(data.clone()));
    println!("Problem 2 {}", problem_2(&data));
}

fn problem_1(mut data: VecDeque<BinTree>) -> u64 {
    loop {
        if data.len() <= 1 {
            break;
        }

        let bin1 = data.pop_front().unwrap();
        let bin2 = data.pop_front().unwrap();

        let mut new_bintree = bin1 + bin2;
        new_bintree.problem_1();

        data.push_front(new_bintree)
    }

    let top_root_id = data.front().unwrap().root;

    data.front().unwrap().calculate_magnitude(top_root_id) as u64
}

fn problem_2(data: &VecDeque<BinTree>) -> u64 {
    let mut max_magnitude: u64 = 0;
    for bin_tree_1 in 0..data.len() {
        for bin_tree_2 in bin_tree_1..data.len() {
            let mut bin_12 =
                data.get(bin_tree_1).unwrap().clone() + data.get(bin_tree_2).unwrap().clone();
            let mut bin_21 =
                data.get(bin_tree_2).unwrap().clone() + data.get(bin_tree_1).unwrap().clone();

            bin_12.problem_1();
            bin_21.problem_1();

            let res_12 = bin_12.calculate_magnitude(bin_12.root) as u64;
            let res_21 = bin_21.calculate_magnitude(bin_21.root) as u64;

            max_magnitude = std::cmp::max(max_magnitude, std::cmp::max(res_12, res_21));
        }
    }

    max_magnitude
}

fn parse_file(filename: &str) -> VecDeque<BinTree> {
    let mut data = VecDeque::new();

    // Open file
    let file = File::open(filename).expect("File not found!");
    let reader = BufReader::new(file);

    // read line
    for line in reader.lines() {
        data.push_back(BinTree::new(&line.unwrap()));
    }

    data
}

#[derive(Debug, Copy, Clone)]
struct SnailFish {
    // Binary Tree Data
    left: Option<usize>,
    right: Option<usize>,
    parent: Option<usize>,

    // Node values
    current_value: Option<usize>,
    current_depth: usize,
}

impl SnailFish {
    fn new(parent: Option<usize>, level: usize) -> SnailFish {
        Self { left: None, right: None, parent, current_value: None, current_depth: level }
    }

    fn new_v(parent: Option<usize>, val: usize, level: usize) -> SnailFish {
        Self { right: None, left: None, parent, current_value: Some(val), current_depth: level }
    }

    fn add(self: &mut Self, id: usize) {
        if self.left == None {
            self.left = Some(id);
        } else {
            self.right = Some(id);
        }
    }
}

#[derive(Debug, Clone)]
struct BinTree {
    root: usize,
    nodes: HashMap<usize, SnailFish>,
    max_node_id: usize,
}

impl BinTree {
    fn new(data: &String) -> BinTree {
        let mut snails: HashMap<usize, SnailFish> = HashMap::new();
        let snails_size = snails.len();
        snails.insert(0, SnailFish::new(None, 0));

        let mut current_parent_id = 0;
        let mut current_id = 1;

        loop {
            if current_id == data.len() - 1 {
                break;
            }

            let current_char = data.chars().nth(current_id).unwrap();
            let snail_size = snails.len();

            match current_char {
                '[' => {
                    snails.insert(
                        snail_size,
                        SnailFish::new(
                            Some(current_parent_id),
                            snails[&current_parent_id].current_depth + 1,
                        ),
                    );

                    snails.get_mut(&current_parent_id).unwrap().add(snail_size);
                    current_parent_id = snail_size;
                },
                ']' => {
                    current_parent_id = snails[&current_parent_id].parent.unwrap();
                },
                ',' => {},
                _ => {
                    snails.insert(
                        snail_size,
                        SnailFish::new_v(
                            Some(current_parent_id),
                            current_char as usize - '0' as usize,
                            snails[&current_parent_id].current_depth + 1,
                        ),
                    );
                    snails.get_mut(&current_parent_id).unwrap().add(snail_size);
                },
            }

            current_id += 1;
        }

        BinTree { root: 0, nodes: snails, max_node_id: snails_size }
    }

    fn problem_1(self: &mut Self) {
        loop {
            let indexs = self.dfs();
            let exploding = indexs.iter().find(|x| {
                self.nodes.get(x).unwrap().current_depth >= 4
                    && self.nodes.get(x).unwrap().current_value.is_none()
            });

            match exploding {
                Some(x) => {
                    let id = *x;
                    self.explode(id);
                    //println!("After explode:  {}", self);
                    continue;
                },
                None => {},
            };

            let split = indexs.iter().find(|x| {
                self.nodes.get(x).unwrap().current_value.is_some()
                    && self.nodes.get(x).unwrap().current_value.unwrap() > 9
            });

            match split {
                Some(x) => {
                    let id = *x;
                    self.split(id);
                    //println!("After split:    {}", self);
                },
                None => break,
            }
        }
    }

    fn explode(self: &mut Self, snail_id: usize) {
        // let snail
        let snail = &self.nodes[&snail_id];
        let snail_left_id = snail.left.unwrap();
        let snail_right_id = snail.right.unwrap();

        // Find the next left element
        let left = self.find_left_branch_not_equal(snail_left_id);

        // Find the next right element
        let right = self.find_right_branch_not_equal(snail_right_id);

        // Get the leftest leaf value
        let rightest_on_left = match left {
            None => None,
            Some(x) => Some(self.find_right_value_in_branch(x)),
        };

        // Get the rightest leaf value
        let leftest_on_right = match right {
            None => None,
            Some(x) => Some(self.find_left_value_in_branch(x)),
        };

        // Add values
        match rightest_on_left {
            Some(x) => {
                let current = *(&self.nodes[&x].current_value.unwrap());
                let left_value = *(&self.nodes[&snail_left_id].current_value.unwrap());
                self.update_snail_current_value(x, current + left_value);
            },
            None => {},
        };

        match leftest_on_right {
            Some(x) => {
                let current = *(&self.nodes[&x].current_value.unwrap());
                let right_value = *(&self.nodes[&snail_right_id].current_value.unwrap());
                self.update_snail_current_value(x, current + right_value);
            },
            None => {},
        }

        // Modify parent by transforming it to a leaf node
        self.update_snail_current_value(snail_id, 0);

        // need to remove both left and right from node and from vector
        // self.nodes[*(&snail.unique_id)].right = None;
        self.nodes.get_mut(&snail_id).unwrap().left = None;
        self.nodes.get_mut(&snail_id).unwrap().right = None;

        return;
    }

    fn split(self: &mut Self, snail_id: usize) {
        // 15 -> [7, 8]
        let unique_id = *self.dfs().iter().max().unwrap() + 1;
        let current_snail_depth = self.nodes[&snail_id].current_depth;

        self.nodes.insert(unique_id, SnailFish::new(Some(snail_id), current_snail_depth + 1));
        self.nodes.insert(unique_id + 1, SnailFish::new(Some(snail_id), current_snail_depth + 1));

        let current_snail_value = self.nodes[&snail_id].current_value.unwrap() as f64;
        let current_left_value = (current_snail_value / 2_f64).floor() as usize;
        let current_right_value = (current_snail_value / 2_f64).ceil() as usize;

        self.update_snail_current_value(unique_id, current_left_value);
        self.update_snail_current_value(unique_id + 1, current_right_value);

        self.nodes.get_mut(&snail_id).unwrap().current_value = None;
        self.nodes.get_mut(&snail_id).unwrap().left = Some(unique_id);
        self.nodes.get_mut(&snail_id).unwrap().right = Some(unique_id + 1);
    }

    fn calculate_magnitude(self: &Self, root: usize) -> usize {
        if self.nodes[&root].current_value.is_some() {
            return self.nodes[&root].current_value.unwrap();
        }

        let left_side = match self.nodes[&root].left {
            Some(x) => 3 * self.calculate_magnitude(x),
            None => 0,
        };

        let right_side = match self.nodes[&root].right {
            Some(x) => 2 * self.calculate_magnitude(x),
            None => 0,
        };

        left_side + right_side
    }

    fn find_left_branch_not_equal(self: &Self, id: usize) -> Option<usize> {
        let mut current_id = id;

        loop {
            let current_parent_id = match self.nodes[&current_id].parent {
                Some(x) => x,
                None => return None,
            };

            let parent_left_branch = match self.nodes[&current_parent_id].left {
                Some(x) => x,
                None => return None,
            };

            if parent_left_branch != current_id {
                return Some(parent_left_branch);
            }

            current_id = current_parent_id;
        }
    }

    fn find_right_branch_not_equal(self: &Self, id: usize) -> Option<usize> {
        let mut current_id = id;

        loop {
            let current_parent_id = match self.nodes[&current_id].parent {
                Some(x) => x,
                None => return None,
            };

            let parent_right_branch = match self.nodes[&current_parent_id].right {
                Some(x) => x,
                None => return None,
            };

            if parent_right_branch != current_id {
                return Some(parent_right_branch);
            }

            current_id = current_parent_id;
        }
    }

    fn find_left_value_in_branch(self: &Self, id: usize) -> usize {
        let mut target_id = id;

        loop {
            match self.nodes[&target_id].current_value {
                Some(_) => return target_id,
                None => {},
            };

            let left_child_id = match self.nodes[&target_id].left {
                Some(x) => x,
                None => panic!("The inexistence of a right branch is not expected."),
            };

            match self.nodes[&left_child_id].current_value {
                Some(_) => return left_child_id,
                None => target_id = self.nodes[&left_child_id].left.unwrap(),
            }
        }
    }

    fn find_right_value_in_branch(self: &Self, id: usize) -> usize {
        let mut target_id = id;

        loop {
            match self.nodes[&target_id].current_value {
                Some(_) => return target_id,
                None => {},
            };

            let right_child_id = match self.nodes[&target_id].right {
                Some(x) => x,
                None => panic!("The inexistence of a right branch is not expected."),
            };

            match self.nodes[&right_child_id].current_value {
                Some(_) => return right_child_id,
                None => target_id = self.nodes[&right_child_id].right.unwrap(),
            }
        }
    }

    fn update_snail_current_value(self: &mut Self, snail_id: usize, value: usize) {
        self.nodes.get_mut(&snail_id).unwrap().current_value.replace(value);
    }

    fn dfs(self: &Self) -> Vec<usize> {
        let mut ids = Vec::new();

        let mut visited = HashMap::new();
        let mut id_stack = VecDeque::new();
        id_stack.push_back(self.root);

        loop {
            if id_stack.is_empty() {
                break;
            }

            let current_id = id_stack.pop_front().unwrap();
            visited.insert(current_id, true);
            ids.push(current_id);

            if self.nodes[&current_id].current_value.is_some() {
                continue;
            }

            if !visited.contains_key(&self.nodes[&current_id].right.unwrap()) {
                id_stack.push_front(self.nodes[&current_id].right.unwrap());
            }

            if !visited.contains_key(&self.nodes[&current_id].left.unwrap()) {
                id_stack.push_front(self.nodes[&current_id].left.unwrap());
            }
        }

        ids
    }
}

impl Add for BinTree {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let left_nodes = self.dfs();
        let right_nodes = other.dfs();
        let root_id = left_nodes.iter().max().unwrap() + 1;

        let mut bin: BinTree = BinTree { root: root_id, nodes: HashMap::new(), max_node_id: 0 };

        // Add root
        bin.nodes.insert(bin.root, SnailFish::new(None, 0));

        // Update left side
        for node_idx in left_nodes.iter() {
            bin.nodes.insert(*node_idx, self.nodes[node_idx]);
            bin.nodes.get_mut(node_idx).unwrap().current_depth += 1;
        }

        bin.nodes.get_mut(&root_id).unwrap().left = Some(left_nodes[0]);
        bin.nodes.get_mut(&left_nodes[0]).unwrap().parent = Some(root_id);

        // Update right side
        let right_shift_id = root_id + 1;
        bin.nodes.get_mut(&root_id).unwrap().right = Some(other.root + right_shift_id);

        // Update right side IDs
        for snail_id in &right_nodes {
            let updated_id = snail_id + right_shift_id;
            bin.nodes.insert(updated_id, other.nodes[&snail_id]);

            bin.nodes.get_mut(&updated_id).unwrap().current_depth += 1;

            match bin.nodes[&updated_id].parent {
                Some(x) => {
                    bin.nodes.get_mut(&updated_id).unwrap().parent.replace(x + right_shift_id);
                    {}
                },
                None => {},
            }

            if bin.nodes[&updated_id].current_value.is_some() {
                continue;
            }

            match bin.nodes[&updated_id].left {
                Some(x) => {
                    bin.nodes.get_mut(&updated_id).unwrap().left.replace(x + right_shift_id);
                    {}
                },
                None => {},
            }

            match bin.nodes[&updated_id].right {
                Some(x) => {
                    bin.nodes.get_mut(&updated_id).unwrap().right.replace(x + right_shift_id);
                    {}
                },
                None => {},
            }
        }

        bin.nodes.get_mut(&(other.root + right_shift_id)).unwrap().parent = Some(root_id);
        bin.max_node_id = right_nodes.iter().max().unwrap() + right_shift_id;
        bin
    }
}

impl fmt::Display for BinTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut id_stack = VecDeque::new();
        id_stack.push_back(self.root as i64);
        loop {
            if id_stack.len() == 0 {
                break;
            }
            let current_id = id_stack.pop_front().unwrap();
            match current_id {
                -1 => write!(f, "["),
                -2 => write!(f, ","),
                -3 => write!(f, "]"),
                _ => {
                    if self.nodes[&(current_id as usize)].current_value.is_some() {
                        write!(f, "{}", self.nodes[&(current_id as usize)].current_value.unwrap())
                    } else {
                        id_stack.push_front(-3);
                        id_stack
                            .push_front(self.nodes[&(current_id as usize)].right.unwrap() as i64);
                        id_stack.push_front(-2);
                        id_stack
                            .push_front(self.nodes[&(current_id as usize)].left.unwrap() as i64);
                        id_stack.push_front(-1);
                        write!(f, "")
                    }
                },
            }
            .unwrap();
        }
        write!(f, "")
    }
}
