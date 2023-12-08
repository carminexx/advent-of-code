use std::fs;

// Problem constrains:
// - Part schematic is a square matrix
// - Part numbers DO repeat

#[derive(Debug, PartialEq, Clone)]
struct PartNumber {
    value: u32,
    pos_start: usize,
    pos_end: usize,
}

fn solve(input: &str) -> u32 {
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
            if !acc.is_empty() {
                line_parts.push(PartNumber {
                    value: acc.parse().unwrap(),
                    pos_start: (pos - acc.len()),
                    pos_end: (pos - 1),
                });
                acc = String::new();
            }
        }
    });

    if !acc.is_empty() {
        line_parts.push(PartNumber {
            value: acc.parse().unwrap(),
            pos_start: (line.len() - acc.len()),
            pos_end: (line.len()),
        });
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
    println!("Part 1: {}", solve("input.txt"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_line_parsing() {
        let lines: &str = "467..114.....*........35..633.......#...617*....33";
        let parsed_parts = parse_line(lines);

        println!("{:?}", parsed_parts);

        assert_eq!(
            true,
            parsed_parts.contains(
                &(PartNumber {
                    value: 467,
                    pos_start: 0,
                    pos_end: 2
                })
            )
        );
        assert_eq!(
            true,
            parsed_parts.contains(
                &(PartNumber {
                    value: 114,
                    pos_start: 5,
                    pos_end: 7
                })
            )
        );
        assert_eq!(
            true,
            parsed_parts.contains(
                &(PartNumber {
                    value: 35,
                    pos_start: 22,
                    pos_end: 23
                })
            )
        );
        assert_eq!(
            true,
            parsed_parts.contains(
                &(PartNumber {
                    value: 633,
                    pos_start: 26,
                    pos_end: 28
                })
            )
        );
        assert_eq!(
            true,
            parsed_parts.contains(
                &(PartNumber {
                    value: 617,
                    pos_start: 40,
                    pos_end: 42
                })
            )
        );
        assert_eq!(
            true,
            parsed_parts.contains(
                &(PartNumber {
                    value: 33,
                    pos_start: 48,
                    pos_end: 50
                })
            )
        );
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
        assert_eq!(4361, solve("demo-input.txt"));
    }
}
