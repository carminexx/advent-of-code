use std::fs;

// WIP

fn solve() -> u32 {
    let raw_data = fs::read_to_string("input.txt").unwrap();

    let schematics: Vec<&str> = raw_data.lines().collect();

    /**
    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..
    */

    // TODO: Add separated evaluation for first and last line (or instead of using windows use chunks from itertools)

    schematics.windows(3).for_each(|s| {
        let first = s[0];
        let second = s[1];
        let third = s[2];

        println!("First: {first}, Second: {second}, Third: {third}");
    });

    0
}

fn main() {
    println!("Part 1: {}", solve());
}
