use std::fs;

fn solve() -> u32 {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().filter(char::is_ascii_digit).collect())
        .map(|digits: Vec<_>| {
            let a = digits.first().unwrap();
            let b = digits.last().unwrap();

            a.to_digit(10).unwrap() * 10 + b.to_digit(10).unwrap()
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
        assert_eq!(55621, solve());
    }
}
