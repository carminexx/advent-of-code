use std::fs;

fn solve() -> i32 {
    let filename = "input.txt";
    let mut lines: Vec<String> = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .map(|c| c.chars().filter(|x| x.is_digit(10)).collect())
        .collect();

    // Notes for improvement: I think I can somehow merge this second loop within the first iterator stream above
    for line in &mut lines {
        let first = line.chars().nth(0).unwrap_or('\0');
        let last = line.chars().last().unwrap_or('\0');

        *line = format!("{first}{last}");
    }

    lines.iter().map(|x| x.parse().unwrap_or(0)).sum()
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
