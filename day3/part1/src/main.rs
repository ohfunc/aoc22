use std::collections::HashMap;
use std::fs;
use std::path::Path;

gflags::define! {
    // The file to use for input.
    -f, --file: &Path
}

gflags::define! {
    -h, --help = false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_priorities_test() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw".to_string();

        assert_eq!(calculate_priorities(input), 157);
    }
}

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn priorities_from_rucksack(sack: &str) -> usize {
    // Ensure input is the size we expect.
    if sack.len() % 2 != 0 {
        panic!("this should never happen");
    }

    let (comp_one, comp_two) = sack.split_at(sack.len() / 2);

    let mut chars_one: HashMap<char, bool> = HashMap::new();
    for (_, c) in comp_one.chars().enumerate() {
        *chars_one.entry(c).or_default() = true;
    }

    let mut duplicate: char = '\0';

    for (_, c) in comp_two.chars().enumerate() {
        duplicate = match chars_one.get(&c) {
            Some(_) => c,
            None => continue,
        };

        break;
    }

    if duplicate == '\0' || !duplicate.is_ascii_alphabetic() {
        panic!("this should never happen");
    }
    
    ALPHABET.chars().position(|c| c == duplicate).unwrap() + 1
}

fn calculate_priorities(input: String) -> usize {
    let mut priorities = 0;
    for sack in input.lines() {
        priorities += priorities_from_rucksack(sack);
    }
    priorities
}

fn main() {
    gflags::parse();
    if HELP.flag || !FILE.is_present() {
        gflags::print_help_and_exit(0);
    }

    let input = fs::read_to_string(FILE.flag)
        .expect("unable to read input");

    println!("the sum of item priorities is: {}", calculate_priorities(input));
}
