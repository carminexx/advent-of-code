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
    let raw_data = fs::read_to_string(input).unwrap();

    let directions: Vec<Direction> = raw_data
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|x| match x {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => todo!(),
        })
        .collect();

    let mut network: HashMap<&str, NetworkNode> = HashMap::new();

    raw_data.lines().skip(2).for_each(|x| {
        let (location, targets) = x.split_once(" = ").unwrap();
        let targets = targets.replace(['(', ')'], "");
        let (left, right) = targets.split_once(", ").unwrap();

        network.insert(
            location.as_ref(),
            NetworkNode::new(
                String::from(location),
                String::from(left),
                String::from(right),
            ),
        );
    });

    let maximum_epochs = 50;
    let mut steps: u32 = 0;

    let mut current_node = network.get("AAA");

    for _ in 1..maximum_epochs {
        for step in &directions {
            match current_node {
                Some(c) if c.location == "ZZZ" => return steps,
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

fn main() {
    println!("Part 1: {}", solve_part1("input.txt"));
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
}
