use std::fs;

fn solve_part1(input: &str) -> u32 {
    let raw = fs::read_to_string(input).unwrap();

    let times: Vec<u16> = raw
        .lines()
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let distances: Vec<u16> = raw
        .lines()
        .last()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut total_solutions: Vec<u16> = Vec::new();

    for x in 0..times.len() {
        let mut solutions: u16 = 0;
        // Since the input of part 1 is very small and solutions all integers, just brute force each race
        // A cleaner alternative solution would be a classic quadratic formula solver

        let race_duration = times[x];
        let race_distance = distances[x];

        for i in 1..race_duration {
            if i * (race_duration - i) > race_distance {
                solutions += 1;
            }
        }
        total_solutions.push(solutions);
    }

    total_solutions.iter().map(|x| *x as u32).product()
}

fn solve_part2(input: &str) -> u32 {
    let raw = fs::read_to_string(input).unwrap();

    let race_duration: u64 = raw
        .lines()
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();
    let race_distance: u64 = raw
        .lines()
        .last()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();

    // Actually also part 2 is still fast with the simple brute-force method
    let mut solutions = 0;
    for i in 1..race_duration {
        if i * (race_duration - i) > race_distance {
            solutions += 1;
        }
    }

    solutions
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
        assert_eq!(288, solve_part1("demo-input.txt"));
    }

    #[test]
    fn test_demo_input_for_part_2() {
        assert_eq!(71503, solve_part2("demo-input.txt"));
    }

    #[test]
    fn test_solve_part_1() {
        assert_eq!(1660968, solve_part1("input.txt"));
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(26499773, solve_part2("input.txt"));
    }
}
