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

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod test {
    use super::solve;

    #[test]
    fn solution() {
        assert_eq!(2447, solve());
    }
}
