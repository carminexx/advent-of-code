use anyhow::{Context, Error, Result};
use itertools::Itertools;
use std::{fs, str::FromStr};

#[derive(Debug, Clone)]
struct ScratchCard {
    id: usize,
    winning_count: usize,
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
                    .collect_vec()
            })
            .collect_tuple()
            .context("failed to parse scratchcard numbers")?;

        Ok(ScratchCard::new(id, &winning_numbers, &scratched_numbers))
    }
}

impl ScratchCard {
    fn new(id: usize, winning_numbers: &[u8], scratched_numbers: &[u8]) -> Self {
        let winning_count = winning_numbers
            .iter()
            .filter(|n| scratched_numbers.contains(n))
            .count();

        Self { id, winning_count }
    }

    fn score(&self) -> usize {
        match self.winning_count {
            0 => 0,
            n => 2usize.pow(n as u32 - 1),
        }
    }
}

fn solve_part1() -> usize {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.parse::<ScratchCard>()
                .context("failed to parse scratchcard")
                .unwrap()
        })
        .map(|card| card.score())
        .sum()
}

fn solve_part2(input_source: &str) -> usize {
    let cards = fs::read_to_string(input_source)
        .unwrap()
        .lines()
        .map(|line| {
            line.parse::<ScratchCard>()
                .context("failed to parse scratchcard")
                .unwrap()
        })
        .collect_vec();

    let mut won_cards = cards.iter().map(|card| card.id).collect_vec();
    let mut score = 0;

    while let Some(card_id) = won_cards.pop() {
        score += 1;

        let winning_count = cards[card_id - 1].winning_count;

        if winning_count == 0 {
            continue;
        }

        for idx in 1..=winning_count {
            let winning_card_id = card_id + idx;

            if winning_card_id <= cards.len() {
                won_cards.push(winning_card_id);
            }
        }
    }

    score
}

fn main() {
    println!("Part one: {}", solve_part1());
    println!("Part two: {}", solve_part2("input.txt"));
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
        assert_eq!(4, card.winning_count);
        assert_eq!(8, card.score());
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(26914, solve_part1());
    }

    #[test]
    fn test_demo_input_for_part2() {
        assert_eq!(30, solve_part2("demo-input.txt"));
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(13080971, solve_part2("input.txt"));
    }
}
