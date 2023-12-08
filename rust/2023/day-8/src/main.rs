use std::{collections::HashMap, fs};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct NetworkNode {
    location: String,
    left: String,
    right: String,
}

impl NetworkNode {
    fn new(location: String, left: String, right: String) -> NetworkNode {
        NetworkNode {
            location,
            left,
            right,
        }
    }
}

fn solve_part1(input: &str) -> u32 {
    let (directions, network) = parse_input(input);

    let maximum_epochs = 50;
    let mut steps: u32 = 0;

    let mut current_node = network.get("AAA");

    'outer: for _ in 1..maximum_epochs {
        for step in &directions {
            match current_node {
                Some(c) if c.location == "ZZZ" => break 'outer,
                _ => {
                    current_node = match step {
                        Direction::Left => network.get(&current_node.unwrap().left as &str),
                        Direction::Right => network.get(&current_node.unwrap().right as &str),
                    };
                    steps += 1;
                }
            }
        }
    }

    steps
}

fn solve_part2(input: &str) -> u32 {
    let (directions, network) = parse_input(input);

    let maximum_epochs = 500;
    let mut steps: u32 = 0;

    let mut current_nodes: Vec<&NetworkNode> = network.iter().filter(|x| x.0.ends_with("A")).map(|x| x.1).collect();

    'outer: for _ in 1..maximum_epochs {
        for step in &directions {
            steps += 1;

            for i in 0..current_nodes.len() {
                let left_node = network.get(&current_nodes[i].left).unwrap();
                let right_node = network.get(&current_nodes[i].right).unwrap();
                
                current_nodes[i] = match step {
                    Direction::Left => &left_node,
                    Direction::Right => &right_node,
                };
            }

            if current_nodes.iter().all(|x| x.location.ends_with("Z")) {
                println!("LAST: Step {}, nodes: {:?}", steps, current_nodes);
                break 'outer;
            }
        }
    }

    steps
}

fn parse_input(input: &str) -> (Vec<Direction>, HashMap<String, NetworkNode>) {
    let raw_data = fs::read_to_string(input).unwrap();

    let directions: Vec<Direction> = raw_data
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|x| match x {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        })
        .collect();

    let network: HashMap<String, NetworkNode> = raw_data
        .lines()
        .skip(2)
        .map(|x| {
            let (location, targets) = x.split_once(" = ").unwrap();
            let targets = targets.replace(['(', ')'], "");
            let (left, right) = targets.split_once(", ").unwrap();

            (
                location.to_owned(),
                NetworkNode::new(
                    String::from(location),
                    String::from(left),
                    String::from(right),
                ),
            )
        })
        .collect();

    (directions, network)
}

fn main() {
    println!("Part 1: {}", solve_part1("input.txt"));
    println!("Part 2: {}", solve_part2("input.txt"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_demo_input_for_part_1() {
        assert_eq!(2, solve_part1("demo-input.txt"));
    }

    #[test]
    fn test_demo_input_for_part_1_bis() {
        assert_eq!(6, solve_part1("demo-input-2.txt"));
    }

    #[test]
    fn test_demo_input_for_part_2() {
        assert_eq!(6, solve_part2("demo-input-part-2.txt"));
    }
}
