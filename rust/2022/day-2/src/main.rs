use std::fs::read_to_string;

// Note: Correct answer for part 1 is: 15523

fn main() {
    let filename = "input.txt";
    let final_score: i32 = read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .map(get_score)
        .sum();

    println!("Final score: {}", final_score);
}

#[derive(PartialEq, PartialOrd, Debug)]
#[repr(u8)]
enum Card {
    Rock = 1,     // A, X  -  value 3
    Paper = 2,    // B, Y  -  value 2
    Scissors = 3, // C, Z  -  value 1
}

#[repr(u8)]
enum Outcome {
    Win = 6,  // 6 points
    Draw = 3, // 3 points
    Loss = 0, // 0 points
}

fn get_score(challenge: String) -> i32 {
    let opponent = code_to_card(challenge.chars().next().unwrap());
    let player = code_to_card(challenge.chars().last().unwrap());

    // Is as_ref() a good practice here or it's better to just implement Copy trait for Card enum?
    let outcome = get_outcome(player.as_ref().unwrap(), opponent.as_ref().unwrap());

    (outcome.unwrap() as u8 + player.unwrap() as u8) as i32
}

fn get_outcome(player: &Card, opponent: &Card) -> Option<Outcome> {
    if player == opponent {
        return Some(Outcome::Draw);
    }

    match (player, opponent) {
        (Card::Rock, Card::Scissors) => Some(Outcome::Win),
        (Card::Scissors, Card::Paper) => Some(Outcome::Win),
        (Card::Paper, Card::Rock) => Some(Outcome::Win),
        (Card::Scissors, Card::Rock) => Some(Outcome::Loss),
        (Card::Paper, Card::Scissors) => Some(Outcome::Loss),
        (Card::Rock, Card::Paper) => Some(Outcome::Loss),
        _ => None,
    }
}

fn code_to_card(code: char) -> Option<Card> {
    match code {
        'A' | 'X' => Some(Card::Rock),
        'B' | 'Y' => Some(Card::Paper),
        'C' | 'Z' => Some(Card::Scissors),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cards() {
        assert_eq!(code_to_card('A'), Some(Card::Rock));
        assert_eq!(code_to_card('B'), Some(Card::Paper));
        assert_eq!(code_to_card('C'), Some(Card::Scissors));
        assert_eq!(code_to_card('X'), Some(Card::Rock));
        assert_eq!(code_to_card('Y'), Some(Card::Paper));
        assert_eq!(code_to_card('Z'), Some(Card::Scissors));
    }

    #[test]
    fn test_score() {
        assert_eq!(get_score(String::from("A Y")), 8); // Win
        assert_eq!(get_score(String::from("B X")), 1); // Loss
        assert_eq!(get_score(String::from("C Z")), 6); // Draw

        assert_eq!(get_score(String::from("B Z")), 9); // Win
        assert_eq!(get_score(String::from("A Z")), 3); // Loss
    }
}
