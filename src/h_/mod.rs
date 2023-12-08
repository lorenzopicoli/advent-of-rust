use std::{
    cell::RefCell,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    rc::Rc,
};

#[derive(Debug)]
struct Graph {
    nodes: Vec<NodeRef>,
}

type NodeRef = Rc<RefCell<Node>>;
type NodeLink = Option<NodeRef>;

#[derive(Debug)]
struct Node {
    id: String,
    right_id: String,
    left_id: String,
    left: NodeLink,
    right: NodeLink,
}

impl Node {
    fn print_node(&self) {
        println!("{} -L-> {} -R-> {}", self.id, self.left_id, self.right_id)
    }
}

impl Graph {
    fn add_node(&mut self, id: &str, left_id: &str, right_id: &str) {
        let mut existing_left: NodeLink = None;
        let mut existing_right: NodeLink = None;
        let mut existing_self: NodeLink = None;
        for n in &self.nodes {
            let n_ref = n.borrow();
            if n_ref.id == left_id {
                existing_left = Some(Rc::clone(n));
            }
            if n_ref.id == right_id {
                existing_right = Some(Rc::clone(n));
            }
            if n_ref.id == id {
                existing_self = Some(Rc::clone(n));
            }
        }

        if existing_left.is_none() {
            let left = Rc::new(RefCell::new(Node {
                id: left_id.to_string(),
                right_id: "".to_string(),
                left_id: "".to_string(),
                left: None,
                right: None,
            }));
            self.nodes.push(Rc::clone(&left));

            if left_id == right_id {
                existing_right = Some(Rc::clone(&left));
            }
            existing_left = Some(left);
        }

        if existing_right.is_none() {
            let right = Rc::new(RefCell::new(Node {
                id: right_id.to_string(),
                right_id: "".to_string(),
                left_id: "".to_string(),
                left: None,
                right: None,
            }));
            self.nodes.push(Rc::clone(&right));
            existing_right = Some(right);
        }

        let node_rc = existing_self.unwrap_or(Rc::new(RefCell::new(Node {
            id: id.to_string(),
            right_id: right_id.to_string(),
            left_id: left_id.to_string(),
            left: None,
            right: None,
        })));

        {
            let mut node = node_rc.borrow_mut();
            node.left = existing_left;
            node.right = existing_right;
        }
        self.nodes.push(node_rc);
    }
}

fn split_str(value: &str, sub: &str) -> Vec<String> {
    let split = value.split(sub);
    split.map(|e| e.to_string()).collect::<Vec<String>>()
}

pub fn part1(reader: BufReader<File>) -> io::Result<i32> {
    let mut graph = Graph { nodes: Vec::new() };
    let mut start_node: NodeLink = None;
    let mut steps = Vec::new();
    for (line_idx, line) in reader.lines().enumerate() {
        let line = line?;
        if line_idx == 0 {
            line.chars().for_each(|c| {
                steps.push(c);
            });
            continue;
        }

        if line.is_empty() {
            continue;
        }

        let parts = split_str(&line.replace(", ", ","), " ");
        let id = parts[0].clone();
        let nodes = parts[2].clone();
        let nodes = split_str(&nodes.replace(['(', ')'], ""), ",");
        let left = nodes[0].clone();
        let right = nodes[1].clone();

        graph.add_node(&id, &left, &right);

        if id == "AAA" {
            start_node = Some(Rc::clone(&graph.nodes[graph.nodes.len() - 1]));
        }
    }

    let mut current_node = match start_node {
        Some(n) => n,
        None => panic!("No start found"),
    };
    let mut step_count = 0;
    loop {
        for step in steps.iter().cycle() {
            {
                let borrowed_current_node = current_node.borrow();
                // borrowed_current_node.print_node();

                if borrowed_current_node.id == "ZZZ" {
                    return Ok(step_count);
                }
            }
            let next_node = match step {
                'R' => Rc::clone(current_node.borrow().right.as_ref().unwrap()),
                'L' => Rc::clone(current_node.borrow().left.as_ref().unwrap()),
                _ => panic!("Invalid step"),
            };
            current_node = next_node;
            step_count += 1;
        }
    }
}

pub fn part2(reader: BufReader<File>) -> io::Result<u128> {
    let mut graph = Graph { nodes: Vec::new() };
    let mut current_nodes: Vec<NodeRef> = Vec::new();
    let mut steps = Vec::new();
    for (line_idx, line) in reader.lines().enumerate() {
        let line = line?;
        if line_idx == 0 {
            line.chars().for_each(|c| {
                steps.push(c);
            });
            continue;
        }

        if line.is_empty() {
            continue;
        }

        let parts = split_str(&line.replace(", ", ","), " ");
        let id = parts[0].clone();
        let nodes = parts[2].clone();
        let nodes = split_str(&nodes.replace(['(', ')'], ""), ",");
        let left = nodes[0].clone();
        let right = nodes[1].clone();

        graph.add_node(&id, &left, &right);

        if id.ends_with('A') {
            current_nodes.push(Rc::clone(&graph.nodes[graph.nodes.len() - 1]));
        }
    }

    let mut z_indexes: Vec<u32> = Vec::new();

    for node in current_nodes {
        let mut seen: Vec<NodeRef> = Vec::new();
        let mut current_node = Rc::clone(&node);
        for (cycle_length, step) in steps.clone().iter().cycle().enumerate() {
            let last_seen_index = seen
                .iter()
                .enumerate()
                .find(|(_, n)| n.borrow().id == current_node.borrow().id)
                .map(|(i, _)| i);

            if let Some(last_seen_index) = last_seen_index {
                if (cycle_length - last_seen_index) % steps.len() == 0 {
                    seen.iter().enumerate().for_each(|(i, n)| {
                        if n.borrow().id.ends_with('Z') {
                            z_indexes.push(i as u32);
                        }
                    });
                    break;
                }
            }

            let next_node = match step {
                'R' => Rc::clone(current_node.borrow().right.as_ref().unwrap()),
                'L' => Rc::clone(current_node.borrow().left.as_ref().unwrap()),
                _ => panic!("Invalid step"),
            };

            seen.push(Rc::clone(&current_node));
            current_node = next_node;
        }
    }

    // Calculate the LCM of z_indexes
    let lcm = z_indexes
        .iter()
        .fold(1_u128, |acc, &x| num_integer::lcm(acc, x as u128));

    Ok(lcm)
}

// Part 2 sucks because I feel like from the prompt we don't have enough information to solve it.
// The cycles could contain more than one Z node, it could take some steps to reach the cycle
// From the input (and the intended solution) all cycles are the same length and so we can
// simply take the LCM of the z indexes
pub fn solution() {
    let path = Path::new("src/h_/input.txt");
    // let path = Path::new("src/h_/example.txt");
    let mut file = File::open(path).unwrap();
    let part1 = part1(io::BufReader::new(file));
    file = File::open(path).unwrap();
    let part2 = part2(io::BufReader::new(file));

    println!("Day 8, part 1 {}", part1.unwrap());
    println!("Day 8, part 2 {}", part2.unwrap());
}
