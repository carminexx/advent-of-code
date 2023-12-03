use lazy_static::lazy_static;
use std::{collections::HashMap, fs};

lazy_static! {
    static ref COLOR_CONSTRAINS: HashMap<&'static str, u8> = 
        HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
}
const INPUT_PREFIX: &str = "Game ";

// Part 1

fn solve() -> u32 {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let (game_id, game_rounds) = line
                .strip_prefix(INPUT_PREFIX)
                .unwrap()
                .split_once(':')
                .unwrap();
            let mut subsets = game_rounds.trim().split(';');

            if subsets.any(|x| {
                let mut cubes = x.trim().split(',');
                cubes.any(|c| {
                    let (value, color) = c.trim().split_once(' ').unwrap();

                    COLOR_CONSTRAINS.get(color).unwrap() < &value.parse::<u8>().unwrap()
                })
            }) {
                // Game invalid, ignore game ID
                0
            } else {
                // Game valid, return game ID to sum
                game_id.parse::<u32>().unwrap()
            }
        })
        .sum()
}

fn solve_part2() -> u32 {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let (game_id, game_rounds) = line
                .strip_prefix(INPUT_PREFIX)
                .unwrap()
                .split_once(':')
                .unwrap();
            let subsets = game_rounds.trim().split(';');

            let mut max_cubes_in_a_game: HashMap<&str, u8> = HashMap::new();

            subsets.for_each(|x| {
                let cubes = x.trim().split(',');
                cubes.for_each(|c| {
                    let (value, color) = c.trim().split_once(' ').unwrap();

                    max_cubes_in_a_game.insert(color, 
                    match max_cubes_in_a_game.get(color) {
                        Some(number) => {
                            if number > &value.parse::<u8>().unwrap() {
                                *number
                            } else {
                                value.parse::<u8>().unwrap()
                            }
                        },
                        None => value.parse::<u8>().unwrap()
                    });
                })
            });

            // Calculate power of cubes

            // Note: the cast to u32 is needed to avoid overflow when calculating the product, probably
            // a cleaner solution by casting only the product itself and not all other elements is possible
            max_cubes_in_a_game.iter().map(|x| u32::from(*x.1)).product::<u32>()
            
        })
        .sum()
}

fn main() {
    println!("Part one: {}", solve());
    println!("Part two: {}", solve_part2());
}

#[cfg(test)]
mod test {
    use crate::solve_part2;

    use super::solve;

    #[test]
    fn solution() {
        assert_eq!(2447, solve());
        assert_eq!(56322, solve_part2());
    }
}
