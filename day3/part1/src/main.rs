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

fn calculate_priorities(input: String) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let line = line.as_bytes();
            let (left, right) = line.split_at(line.len() / 2);
            left
                .iter()
                .find(|item| right.contains(item))
                .map(|item| match item {
                    b'a'..=b'z' => (item - b'a') + 1,
                    _ => (item - b'A') + 27,
                } as u32)
        })
        .sum()
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
