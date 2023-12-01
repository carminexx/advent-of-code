use std::{fs::read_to_string, str::LinesAny};

// Note: Correct answer for part 1 is: 55621
// Note: Correct answer for part 2 is: 201491

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename) 
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut acc: i32 = 0;
    let mut results: Vec<i32> = Vec::new();
    for line in lines.iter() {
        match line.len() {
            0 => {
                results.push(acc);
                acc = 0;
            },
            _ => acc += line.parse().unwrap_or(0),
        }
    }

    let max: i32 = *results.iter().max().unwrap();

    // Part 2 question
    results.sort_by(|a, b| b.cmp(a)); // Standard reverse sorting; not efficient but quickly to implement
    let top_three_sum = results[0] + results[1] + results[2];

    println!("{}", max);
    println!("Part two: {}", top_three_sum);
}
