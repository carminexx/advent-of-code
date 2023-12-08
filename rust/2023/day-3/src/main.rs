use std::fs;

// Problem constrains:
// - Part schematic is a square matrix
// - Part numbers DO repeat

#[derive(Debug, PartialEq, Clone)]
struct PartNumber {
    value: u32,
    pos_start: usize,
    pos_end: usize,
    is_gear: bool,
}

impl PartNumber {
    fn new(value: u32, pos_start: usize, pos_end: usize) -> PartNumber {
        PartNumber {
            value,
            pos_start,
            pos_end,
            is_gear: false,
        }
    }

    fn gear(pos: usize) -> PartNumber {
        PartNumber {
            value: 0,
            pos_start: pos,
            pos_end: pos,
            is_gear: true,
        }
    }
}

fn solve_part1(input: &str) -> u32 {
    let raw_data = fs::read_to_string(input).unwrap();
    let schematics: Vec<&str> = raw_data.lines().filter(|x| !x.is_empty()).collect();

    let mut valid_parts: Vec<u32> = Vec::new();

    let mut first_line = extract_valid_parts(
        schematics.first().unwrap(),
        parse_line(schematics.first().unwrap()),
        None,
        Some(schematics[1]),
    );
    valid_parts.append(&mut first_line);

    schematics.windows(3).for_each(|s| {
        let top = s[0];
        let current = s[1];
        let bottom = s[2];

        let mut nth_line =
            extract_valid_parts(current, parse_line(current), Some(top), Some(bottom));
        valid_parts.append(&mut nth_line);
    });

    let mut last_line = extract_valid_parts(
        schematics.last().unwrap(),
        parse_line(schematics.last().unwrap()),
        schematics.get(schematics.len() - 2).copied(),
        None,
    );
    valid_parts.append(&mut last_line);

    valid_parts.iter().sum()
}

fn parse_line(line: &str) -> Vec<PartNumber> {
    let mut line_parts: Vec<PartNumber> = Vec::new();
    let mut acc: String = String::new();

    line.chars().enumerate().for_each(|(pos, char)| match char {
        c if c.is_ascii_digit() => acc.push(c),
        _ => {
            if char == '*' {
                line_parts.push(PartNumber::gear(pos));
            }
            if !acc.is_empty() {
                line_parts.push(PartNumber::new(
                    acc.parse().unwrap(),
                    pos - acc.len(),
                    pos - 1,
                ));
                acc = String::new();
            }
        }
    });

    if !acc.is_empty() {
        line_parts.push(PartNumber::new(
            acc.parse().unwrap(),
            line.len() - acc.len(),
            line.len(),
        ));
    }

    line_parts
}

fn extract_valid_parts(
    line: &str,
    parts: Vec<PartNumber>,
    top_line: Option<&str>,
    bottom_line: Option<&str>,
) -> Vec<u32> {
    parts
        .iter()
        .filter(|part| {
            let safe_start_pos = if part.pos_start == 0 {
                0
            } else {
                part.pos_start - 1
            };
            let safe_end_pos = if part.pos_end == line.len() {
                line.len()
            } else {
                part.pos_end + 1
            };

            // Around
            let around = is_part_symbol(line.chars().nth(safe_start_pos))
                || is_part_symbol(line.chars().nth(safe_end_pos));

            // Top line (including diagonals)
            let top = top_line
                .unwrap_or("")
                .chars()
                .enumerate()
                .filter(|(pos, _)| pos >= &(safe_start_pos) && pos <= &(safe_end_pos))
                .any(|(_, c)| is_part_symbol(Some(c)));

            // Bottom line (including diagonals)
            let bottom = bottom_line
                .unwrap_or("")
                .chars()
                .enumerate()
                .filter(|(pos, _)| pos >= &(safe_start_pos) && pos <= &(safe_end_pos))
                .any(|(_, c)| is_part_symbol(Some(c)));

            around || top || bottom
        })
        .map(|x| x.value)
        .collect()
}

fn is_part_symbol(char: Option<char>) -> bool {
    match char {
        Some(char) => !char.is_ascii_digit() && char != '.',
        None => false,
    }
}

fn main() {
    println!("Part 1: {}", solve_part1("input.txt"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_line_parsing() {
        let lines: &str = "467..114.....*........35..633.......#...617*....33";
        let parsed_parts = parse_line(lines);

        println!("{:?}", parsed_parts);

        assert_eq!(true, parsed_parts.contains(&PartNumber::new(467, 0, 2)));
        assert_eq!(true, parsed_parts.contains(&PartNumber::new(114, 5, 7)));
        assert_eq!(true, parsed_parts.contains(&PartNumber::new(35, 22, 23)));
        assert_eq!(true, parsed_parts.contains(&PartNumber::new(633, 26, 28)));
        assert_eq!(true, parsed_parts.contains(&PartNumber::new(617, 40, 42)));
        assert_eq!(true, parsed_parts.contains(&PartNumber::new(33, 48, 50)));
    }

    #[test]
    fn test_valid_parts() {
        let top_line: &str = "...*......";
        let line: &str = "..35..633.";
        let bottom_line: &str = "......#...";

        let parsed_parts = parse_line(line);

        let valid_parts =
            extract_valid_parts(line, parsed_parts, Some(top_line), Some(bottom_line));

        assert_eq!(true, valid_parts.contains(&35));
        assert_eq!(true, valid_parts.contains(&633));
    }

    #[test]
    fn test_valid_parts_bottom_line() {
        let top_line: &str = "...$.*....";
        let line: &str = ".664.598..";

        let parsed_parts = parse_line(line);
        let valid_parts = extract_valid_parts(line, parsed_parts, Some(top_line), None);

        assert_eq!(true, valid_parts.contains(&664));
        assert_eq!(true, valid_parts.contains(&598));
    }

    #[test]
    fn test_demo_input_for_part_1() {
        assert_eq!(4361, solve_part1("demo-input.txt"));
    }
}
