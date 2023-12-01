use std::fs::read_to_string;

// Note: Correct answer for part 2 is: 15702

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

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
enum Card {
    Rock = 1,     // A  -  value 3
    Paper = 2,    // B  -  value 2
    Scissors = 3, // C  -  value 1
}

#[derive(PartialEq, PartialOrd, Debug)]
#[repr(u8)]
enum Outcome {
    Win = 6,  // 6 points - value Z
    Draw = 3, // 3 points - value Y
    Loss = 0, // 0 points - value X
}

fn get_score(challenge: String) -> i32 {
    let opponent = code_to_card(challenge.chars().next().unwrap());
    let desired_outcome = code_to_outcome(challenge.chars().last().unwrap());

    let played_card = get_player_card(
        opponent.as_ref().unwrap(),
        desired_outcome.as_ref().unwrap(),
    );

    (desired_outcome.unwrap() as u8 + played_card.unwrap() as u8) as i32
}

fn get_player_card(opponent: &Card, desired_outcome: &Outcome) -> Option<Card> {
    if *desired_outcome == Outcome::Draw {
        // Is actually the best practice to implement Copy for Card; or it's possible to avoid to waste memory here?
        // (see related day-2 part 1 solution where Copy hasn't been implemented)
        return Some(opponent.clone());
    }

    match (desired_outcome, opponent) {
        (Outcome::Win, Card::Rock) => Some(Card::Paper),
        (Outcome::Win, Card::Paper) => Some(Card::Scissors),
        (Outcome::Win, Card::Scissors) => Some(Card::Rock),
        (Outcome::Loss, Card::Rock) => Some(Card::Scissors),
        (Outcome::Loss, Card::Paper) => Some(Card::Rock),
        (Outcome::Loss, Card::Scissors) => Some(Card::Paper),
        _ => None,
    }
}

// Could these two methods be defined with a custom method within the Enum like "Card::from_code(char)"?
fn code_to_card(code: char) -> Option<Card> {
    match code {
        'A' => Some(Card::Rock),
        'B' => Some(Card::Paper),
        'C' => Some(Card::Scissors),
        _ => None,
    }
}

fn code_to_outcome(code: char) -> Option<Outcome> {
    match code {
        'X' => Some(Outcome::Loss),
        'Y' => Some(Outcome::Draw),
        'Z' => Some(Outcome::Win),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cards_and_outcomes() {
        assert_eq!(code_to_card('A'), Some(Card::Rock));
        assert_eq!(code_to_card('B'), Some(Card::Paper));
        assert_eq!(code_to_card('C'), Some(Card::Scissors));
        assert_eq!(code_to_outcome('X'), Some(Outcome::Loss));
        assert_eq!(code_to_outcome('Y'), Some(Outcome::Draw));
        assert_eq!(code_to_outcome('Z'), Some(Outcome::Win));
    }

    #[test]
    fn test_score() {
        assert_eq!(get_score(String::from("A Y")), 4); // Must draw
        assert_eq!(get_score(String::from("B X")), 1); // Must lose
        assert_eq!(get_score(String::from("C Z")), 7); // Must win

        assert_eq!(get_score(String::from("B Z")), 9); // Must win
        assert_eq!(get_score(String::from("A Z")), 8); // Must win
    }
}
