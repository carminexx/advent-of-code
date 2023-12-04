use std::{collections::HashSet, fs, str::FromStr};
use anyhow::{anyhow, Context, Error, Result};
use itertools::Itertools;

 #[derive(Debug)]
 struct ScratchCard {
    id: u32,
    winning_numbers: HashSet<u8>,
    scratched_numbers: HashSet<u8>
}

impl FromStr for ScratchCard {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (id, numbers) = s
            .splitn(2, ": ")
            .collect_tuple()
            .context("failed to parse around ':'")?;

        let (_ ,id) = id
            .split_ascii_whitespace()
            .collect_tuple()
            .context("failed to parse around ' '")?;

        let id = id.parse().context("failed to parse scratchcard id")?;

        let (winning_numbers, scratched_numbers) = numbers
            .splitn(2, " | ")
            .map(|x| x.split_ascii_whitespace().map(|y| y.parse().unwrap()).collect())
            .collect_tuple()
            .context("failed to parse scratchcard numbers")?;

        Ok(ScratchCard { id, winning_numbers, scratched_numbers })
    }
}

impl ScratchCard {
    fn calculate_card_score(&self) -> u32 {
        let scratched_winning_numbers = self.scratched_numbers.intersection(&self.winning_numbers).collect::<Vec<&u8>>();

        match scratched_winning_numbers.len() {
            0 => 0,
            _ => 2u32.pow(scratched_winning_numbers.len() as u32 - 1) as u32
        }
    }
}

fn solve_part1() -> u32 {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.parse::<ScratchCard>()
                .context("failed to parse scratchcard")
                .unwrap()
        })
        .map(|card| card.calculate_card_score())
        .sum()
}

fn main() {
    println!("Part one: {}", solve_part1());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_card() {
        let raw_card = String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        let parsed_card = ScratchCard::from_str(&raw_card).unwrap();
        assert_eq!(1, parsed_card.id);
        assert_eq!(HashSet::from([41, 48, 83, 86, 17]), parsed_card.winning_numbers);
        assert_eq!(HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]), parsed_card.scratched_numbers);
        assert_eq!(8, parsed_card.calculate_card_score());
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(26914, solve_part1());
    }
}
