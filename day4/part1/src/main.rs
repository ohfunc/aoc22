use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_supersets_test() {
        // 2-8,3-7 and 3-7,6-6 contain proper subsets/supersets
        let input = String::from(
            "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
        );

        assert_eq!(find_supersets(input), 2);
    }
}

fn find_supersets(input: String) -> usize {
    input
        .lines()
        .filter(|line| {
            let (elf1, elf2) = line.split_once(",").unwrap();
            let (min1, max1) = elf1.split_once("-").unwrap();
            let (min2, max2) = elf2.split_once("-").unwrap();

            let min1: u32 = min1.parse().unwrap();
            let min2: u32 = min2.parse().unwrap();
            let max1: u32 = max1.parse().unwrap();
            let max2: u32 = max2.parse().unwrap();

            (min1 <= min2 && max1 >= max2) || (min1 >= min2 && max1 <= max2)
        })
        .count()
}

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    println!(
        "The number of assignments pairs that have a range that fully contain the other is: {}",
        find_supersets(input)
    );
}
