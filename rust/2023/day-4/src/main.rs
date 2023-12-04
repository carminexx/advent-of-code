use std::{collections::{HashSet, HashMap}, fs, str::FromStr};
use anyhow::{anyhow, Context, Error, Result};
use itertools::Itertools;

 #[derive(Debug, Clone)]
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
    fn scratched_winnings(&self) -> u8 {
        self.scratched_numbers.intersection(&self.winning_numbers).count() as u8
    }

    fn calculate_card_score(&self) -> u32 {
        match &self.scratched_winnings() {
            0 => 0,
            _ => 2u32.pow(self.scratched_winnings() as u32 - 1) as u32
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

    let mut winned_cards: Vec<ScratchCard> = original_cards.iter().map(|x| x.1.clone()).collect::<Vec<_>>();
    let mut score: u32 = 0;

    while let Some(card) = winned_cards.pop() {
        //winned_cards.push(card.clone()); // Put also fir
        score += 1;
        if card.scratched_winnings() == 0 {
            continue;
        }

        for c in 1..=card.scratched_winnings() {
            match original_cards.get(&(card.id + c as u32)) {
                Some(c) => winned_cards.push(c.clone()),
                None => continue
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

    #[test]
    fn test_demo_input_for_part2() {
        assert_eq!(30, solve_part2("demo-input.txt"));
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(13080971, solve_part2("input.txt"));
    }
}
