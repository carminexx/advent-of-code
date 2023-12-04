use anyhow::{Context, Error, Result};
use itertools::Itertools;
use std::{collections::HashSet, fs, str::FromStr};

#[derive(Debug)]
struct ScratchCard {
    id: u32,
    winning_numbers: HashSet<u8>,
    scratched_numbers: HashSet<u8>,
}

impl FromStr for ScratchCard {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (id, numbers) = s
            .splitn(2, ": ")
            .collect_tuple()
            .context("failed to parse around ':'")?;

        let (_, id) = id
            .split_ascii_whitespace()
            .collect_tuple()
            .context("failed to parse around ' '")?;

        let id = id.parse().context("failed to parse scratchcard id")?;

        let (winning_numbers, scratched_numbers) = numbers
            .splitn(2, " | ")
            .map(|x| {
                x.split_ascii_whitespace()
                    .map(|y| y.parse().unwrap())
                    .collect()
            })
            .collect_tuple()
            .context("failed to parse scratchcard numbers")?;

        Ok(ScratchCard {
            id,
            winning_numbers,
            scratched_numbers,
        })
    }
}

impl ScratchCard {
    fn score(&self) -> Result<u32> {
        let winning_count = u32::try_from(
            self.scratched_numbers
                .intersection(&self.winning_numbers)
                .count(),
        )
        .context("winning numbers > u32::MAX")?;

        Ok(match winning_count {
            0 => 0,
            n => 2_u32.pow(n - 1),
        })
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
        .map(|card| card.score().unwrap())
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
        let card: ScratchCard = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
            .parse()
            .unwrap();

        assert_eq!(1, card.id);
        assert_eq!(HashSet::from([41, 48, 83, 86, 17]), card.winning_numbers);
        assert_eq!(
            HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]),
            card.scratched_numbers
        );
        assert_eq!(8, card.score().unwrap());
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(26914, solve_part1());
    }
}
