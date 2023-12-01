use std::{fs::read_to_string, collections::HashSet};

fn main() {
    let filename = "input.txt";
    let priority_sum: i32 = read_to_string(filename) 
        .unwrap()
        .lines()
        .map(|x| {
            let (comp_a, comp_b) = x.split_at(x.len() / 2);
            let elems_comp_a: HashSet<char> = comp_a.chars().collect();
            let elems_comp_b: HashSet<char> = comp_b.chars().collect();

            // TODO

        })
        .sum();

        println!("Sum of priorities: {}", priority_sum);
 }

fn elem_to_priority(elem) {
    // TODO
}
