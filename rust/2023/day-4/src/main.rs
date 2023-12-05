use anyhow::{Context, Error, Result};
use itertools::Itertools;
use std::{collections::HashMap, collections::HashSet, fs, str::FromStr};

#[derive(Debug, Clone)]
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
    fn winnings(&self) -> u8 {
        self.scratched_numbers
            .intersection(&self.winning_numbers)
            .count() as u8
    }

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

// Brute-force solution, adding each card to the list and counting the final score.
// Tooks over 30-40 seconds to run, there's a lot of room for improvement for sure.
fn solve_part2(input_source: &str) -> u32 {
    let original_cards: HashMap<u32, ScratchCard> = fs::read_to_string(input_source)
        .unwrap()
        .lines()
        .map(|line| {
            line.parse::<ScratchCard>()
                .context("failed to parse scratchcard")
                .unwrap()
        })
        .map(|c| (c.id, c))
        .collect();

    let mut winned_cards: Vec<ScratchCard> = original_cards
        .iter()
        .map(|x| x.1.clone())
        .collect::<Vec<_>>();
    let mut score: u32 = 0;

    while let Some(card) = winned_cards.pop() {
        score += 1;
        if card.winnings() == 0 {
            continue;
        }

        for c in 1..=card.winnings() {
            match original_cards.get(&(card.id + c as u32)) {
                Some(c) => winned_cards.push(c.clone()),
                None => continue,
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

    #[test]
    fn test_demo_input_for_part2() {
        assert_eq!(30, solve_part2("demo-input.txt"));
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(13080971, solve_part2("input.txt"));
    }
}
