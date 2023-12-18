# 🎄 Advent of Code

Some of my solutions for [Advent of Code](https://adventofcode.com/) puzzles, mostly done during my spare time or coffee breaks.

## 2023

|        | Language | Problem                                                   | Solution                                                                                        | Part 1  | Part 2 |
|--------|----------|-----------------------------------------------------------|-------------------------------------------------------------------------------------------------|---------|--------|
| Day 1  | Rust     |[Trebuchet?!](https://adventofcode.com/2023/day/1)         | [main.rs](https://github.com/carminexx/Advent-Of-Code/blob/develop/rust/2023/day-1/src/main.rs) | ✓       | ✓      |
| Day 2  | Rust     |[Cube Conundrum](https://adventofcode.com/2023/day/2)      | [main.rs](https://github.com/carminexx/Advent-Of-Code/blob/develop/rust/2023/day-2/src/main.rs) | ✓       | ✓      |
| Day 3  | Rust     |[Gear Ratios](https://adventofcode.com/2023/day/3)         | [main.rs](https://github.com/carminexx/Advent-Of-Code/blob/develop/rust/2023/day-3/src/main.rs) | ✓       | ✓      |
| Day 4  | Rust     |[Scratchcards](https://adventofcode.com/2023/day/4)        | [main.rs](https://github.com/carminexx/Advent-Of-Code/blob/develop/rust/2023/day-4/src/main.rs) | ✓       | ✓      |
| Day 5  | Rust     |[If You Give A Seed A Fertilizer](https://adventofcode.com/2023/day/5) |   | todo       | todo      |
| Day 6  | Rust     |[Wait For It](https://adventofcode.com/2023/day/6)         |  | todo       | todo      |
| Day 7  | Rust     |[Camel Cards](https://adventofcode.com/2023/day/7)         |  | todo       | todo      |
| Day 8  | Rust     |[Haunted Wasteland](https://adventofcode.com/2023/day/8)   | [main.rs](https://github.com/carminexx/Advent-Of-Code/blob/develop/rust/2023/day-8/src/main.rs) | ✓       | ✓      |


## 2022

|        | Language | Problem                                                       | Solution                                                                                        | Part 1  | Part 2 |
|--------|----------|---------------------------------------------------------------|-------------------------------------------------------------------------------------------------|---------|--------|
| Day 1  | Rust     |[Calorie Counting](https://adventofcode.com/2022/day/1)        | [main.rs](https://github.com/carminexx/Advent-Of-Code/blob/develop/rust/2022/day-1/src/main.rs) | ✓       | ✓      |
| Day 2  | Rust     |[Rock Paper Scissors](https://adventofcode.com/2022/day/2)     | [main.rs](https://github.com/carminexx/Advent-Of-Code/blob/develop/rust/2022/day-2/src/main.rs) | ✓       | ✓      |
| Day 3  | Rust     |[Rucksack Reorganization](https://adventofcode.com/2022/day/3) | [main.rs](https://github.com/carminexx/Advent-Of-Code/blob/develop/rust/2022/day-3/src/main.rs) | ✓       | ✓      |


# Input data and notes

Due to copyright requirements of Advent of Code one is not allowed to publicy share its personal generated input files.  
If you want to run my solutions, you must login with your account to adventofcode.com, download your personal inputs and use them instead of the encrypted files. 

For my reference, inputs here are committed but encrypted with `git-crypt`.  
For this reason, if you just clone and run the projects within this repository none of the actual input files works.  
Consider also that some test cases also checks against my personal solution number, you need to replace values with your solutions in Rust unit tests.

## Decrypt input files

1. Install `git-crypt`: `brew install git-crypt`
2. Unlock the repository with the personal encryption key: `git-crypt unlock ~/aoc.key`