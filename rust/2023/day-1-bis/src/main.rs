use std::fs::read_to_string;

// Note: Correct answer for part 2 is: 53592

fn main() {
    let filename = "input.txt";
    let mut lines: Vec<String> = read_to_string(filename) 
        .unwrap()
        .lines() 
        .map(String::from)
        .map(clean_string_to_digits_overlapping)
        .map(|c| c.chars()
                            .filter(|x| x.is_digit(10))
                            .collect())
        .collect();

    // Notes for improvement: I think I can somehow merge this second loop within the first iterator stream above
    for line in &mut lines {
        let first = line.chars().nth(0).unwrap_or('\0');
        let last = line.chars().last().unwrap_or('\0');

        *line = format!("{first}{last}");
    }

    let sum: i32 = lines.iter().map(|x| x.parse().unwrap_or(0)).sum();
    println!("{}", sum);
}

// NOTE: This method doesn't allow overlapping values, and fails the assignment of part 2 (see new clean_string_to_digits_overlapping method)
fn clean_string_to_digits(input_str: String) -> String {
    // Toxic way to make multiple-replaces in a single line, considering that the line is very small
    // A more sustainable solution would be either regexps or the Aho-Corasick algorithm

    input_str.replace("one", "1")
                                    .replace("two", "2")
                                    .replace("three", "3")
                                    .replace("four", "4")
                                    .replace("five", "5")
                                    .replace("six", "6")
                                    .replace("seven", "7")
                                    .replace("eight", "8")
                                    .replace("nine", "9")
    
}

fn clean_string_to_digits_overlapping(input_str: String) -> String {
    // A little less toxic way to make multiple-replaces in a single line, considering that the line is very small
    // A more sustainable solution would still be either regexps or the Aho-Corasick algorithm
    // This needs to account also for overlapping matches (e.g: "xtwone3four" => "x2134")

    // NOTE: This method actually solves the problem, but leaves a dirty string without actually removing all matches.
    // For example, the xtwone3four becomes x2wo134 instead of x2134.
    // Anyway, in the main loop all characters are removed so this is not an issue

    let patterns = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let replace_with = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let mut acc: String = input_str;

    // Replace patterns one by one, and prefix- and -suffix all pattern to avoid truncating overlapping ones
    for(i, pattern) in patterns.iter().enumerate() {
        acc = acc.replace(pattern, format!("{}{}{}", pattern, replace_with[i], pattern).as_str());
    }

    // Removes all patterns in the accumulator string
    for pattern in patterns {
        acc = acc.replace(pattern, "");
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_string() {
        assert_eq!(clean_string_to_digits(String::from("three")), "3");
        assert_eq!(clean_string_to_digits(String::from("75sevenzdrpkv1onetwo")), "757zdrpkv112");
        assert_eq!(clean_string_to_digits(String::from("mxmkjvgsdzfhseightonetwoeight7")), "mxmkjvgsdzfhs81287");
    }

    #[test]
    // Note that this unit test does not pass (see comments in clean_string_to_digits_overlapping)
    // It's left here just as a reference for a potentially "cleaner" solution
    fn test_clean_string_overlapping() {
        assert_eq!(clean_string_to_digits_overlapping(String::from("xtwone3four")), "x2134");
        assert_eq!(clean_string_to_digits_overlapping(String::from("eightwothree")), "823");
        assert_eq!(clean_string_to_digits_overlapping(String::from("mxmkjvgsdzfhseightonetwoeight7")), "mxmkjvgsdzfhs81287");
    }
}