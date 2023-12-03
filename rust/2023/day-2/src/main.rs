use anyhow::{anyhow, Context, Error, Result};
use itertools::Itertools;
use std::{fs, str::FromStr};

type Amount = u32;
type Id = u32;

#[derive(Debug)]
struct Game {
    id: Id,
    sets: Vec<Set>,
}

#[derive(Debug, Default, Clone, Copy)]
struct Set {
    blue: Amount,
    red: Amount,
    green: Amount,
}

impl Set {
    fn is_possible(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    fn max(&self, other: &Set) -> Self {
        Self {
            red: other.red.max(self.red),
            green: other.green.max(self.green),
            blue: other.blue.max(self.blue),
        }
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (game, sets) = s
            .splitn(2, ": ")
            .collect_tuple()
            .context("failed to parse around ':'")?;

        let (_, id) = game
            .splitn(2, ' ')
            .collect_tuple()
            .context("failed to parse around ' '")?;

        let id = id.parse().context("failed to parse game id")?;

        let sets = sets
            .split("; ")
            .map(str::parse)
            .collect::<Result<_>>()
            .context("failed to parse game sets")?;

        Ok(Game { id, sets })
    }
}

impl FromStr for Set {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut set = Set::default();

        for ss in s.split(", ") {
            let (amount, color) = ss
                .split(' ')
                .collect_tuple()
                .context("failed to parse set")?;

            let amount = amount.parse().context("failed to parse cubes amount")?;

            match color {
                "blue" => set.blue = amount,
                "red" => set.red = amount,
                "green" => set.green = amount,
                _ => return Err(anyhow!("unknown cube color {color}")),
            }
        }

        Ok(set)
    }
}

fn solve_part1() -> u32 {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.parse::<Game>()
                .context("failed to parse game")
                .unwrap()
        })
        .filter(|game| game.sets.iter().all(Set::is_possible))
        .map(|game| game.id)
        .sum()
}

fn solve_part2() -> u32 {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.parse::<Game>()
                .context("failed to parse game")
                .unwrap()
        })
        .map(|Game { sets, .. }| {
            let Set { red, green, blue } =
                sets.iter().fold(Set::default(), |acc, set| acc.max(set));

            red * green * blue
        })
        .sum()
}

fn main() {
    println!("Part one: {}", solve_part1());
    println!("Part two: {}", solve_part2());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(2447, solve_part1());
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(56322, solve_part2());
    }
}
