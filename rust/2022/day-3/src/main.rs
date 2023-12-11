use std::{collections::HashSet, fs::read_to_string};

fn solve_part1(input: &str) -> u32 {
    read_to_string(input)
        .unwrap()
        .lines()
        .map(|x| {
            let (comp_a, comp_b) = x.split_at(x.len() / 2);
            let elems_comp_a: HashSet<char> = comp_a.chars().collect();
            let elems_comp_b: HashSet<char> = comp_b.chars().collect();

            let mut unique = elems_comp_a.intersection(&elems_comp_b);

            elem_to_priority(unique.next().unwrap())
        })
        .sum()
}

fn solve_part2(input: &str) -> u32 {
    let input: String = read_to_string(input).unwrap();

    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|x| {
            let common_elements = x
                .iter()
                .map(|line| line.chars().collect::<HashSet<char>>())
                .reduce(|acc, set| &acc & &set)
                .unwrap();

            elem_to_priority(common_elements.iter().collect::<Vec<_>>().first().unwrap())
        })
        .sum()
}

fn elem_to_priority(elem: &char) -> u32 {
    let mut priorities = ('a'..='z').chain('A'..='Z');
    priorities.position(|x| x == *elem).unwrap() as u32 + 1
}

fn main() {
    println!("Part 1: {}", solve_part1("input.txt"));
    println!("Part 2: {}", solve_part2("input.txt"))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_demo_input_for_part_1() {
        assert_eq!(157, solve_part1("demo-input.txt"));
    }

    #[test]
    fn test_demo_input_for_part_2() {
        assert_eq!(70, solve_part2("demo-input.txt"));
    }

    #[test]
    fn test_input_for_part_1() {
        assert_eq!(8233, solve_part1("input.txt"));
    }

    #[test]
    fn test_input_for_part_2() {
        assert_eq!(2821, solve_part2("input.txt"));
    }
}
