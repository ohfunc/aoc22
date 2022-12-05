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
    fn find_most_calories_test() {
        let inv = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000".to_string();
        assert_eq!(find_most_calories(inv), 45_000);
    }
}

fn find_most_calories(inv_str: String) -> i32 {
    let mut inv: Vec<i32> = Vec::new();

    let mut sum = 0;
    for line in inv_str.lines() {
        if line.is_empty() {
            inv.push(sum);
            sum = 0;
        }

        sum += line.parse().unwrap_or(0);
    }

    // Catch the corner case of the file not ending in a newline.
    if sum > 0 {
        inv.push(sum);
    }

    inv.sort();
    inv.reverse();

    let top_three = &inv[..3];

    top_three.iter().sum()
}

fn main() {
    gflags::parse();
    if HELP.flag || !FILE.is_present() {
        gflags::print_help_and_exit(0);
    }

    let input = fs::read_to_string(FILE.flag)
        .expect("unable to read input");

    println!("the top three elves are carrying {} calories", find_most_calories(input));
}
