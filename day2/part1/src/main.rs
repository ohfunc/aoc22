use std::fs;
use std::path::Path;

gflags::define! {
    // The file to use for input.
    -f, --file: &Path
}

gflags::define! {
    -h, --help = false
}

const ROCK_POINTS: i32 = 1;
const PAPER_POINTS: i32 = 2;
const SCISSORS_POINTS: i32 = 3;

const LOSE_POINTS: i32 = 0;
const DRAW_POINTS: i32 = 3;
const WIN_POINTS: i32 = 6;

const OPPONENT_ROCK: &str = "A";
const OPPONENT_PAPER: &str = "B";
const OPPONENT_SCISSORS: &str = "C";

const YOU_ROCK: &str = "X";
const YOU_PAPER: &str = "Y";
const YOU_SCISSORS: &str = "Z";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_win() {
        // Paper beats rock, paper = 2pt, win = 6pt, 7pt total.
        assert_eq!(round(OPPONENT_ROCK, YOU_PAPER), Ok(8));
    }

    #[test]
    fn round_draw() {
        // Scissors * scissors = draw. Scissors = 3pts, draw = 3pts, 6pt total.
        assert_eq!(round(OPPONENT_SCISSORS, YOU_SCISSORS), Ok(6));
    }

    #[test]
    fn round_lose() {
        // Rock beats scissors! Scissors = 3 pts, lose = 0 pts, 3pt total.
        assert_eq!(round(OPPONENT_ROCK, YOU_SCISSORS), Ok(3));
    }

    #[test]
    fn round_bad_input() {
        assert_eq!(round(OPPONENT_PAPER, "D"), Err("invalid player input"));
        assert_eq!(round("X", YOU_ROCK), Err("invalid result"));
    }

    #[test]
    fn calculate_total_test() {
        let input = "A Y
B X
C Z
        ".to_string();

        assert_eq!(calculate_total(input), 15);
    }
}

fn round(opponent: &str, you: &str) -> Result<i32, &'static str> {
    let shape_score = match you {
        YOU_ROCK => ROCK_POINTS,
        YOU_PAPER => PAPER_POINTS,
        YOU_SCISSORS => SCISSORS_POINTS,
        _ => return Err("invalid player input"),
    };

    let outcome_score = match opponent {
        OPPONENT_ROCK => 
            match you {
                YOU_SCISSORS => LOSE_POINTS,
                YOU_ROCK => DRAW_POINTS,
                YOU_PAPER => WIN_POINTS,
                _ => return Err("invalid result"),
            },
        OPPONENT_PAPER =>
            match you {
                YOU_ROCK => LOSE_POINTS,
                YOU_PAPER => DRAW_POINTS,
                YOU_SCISSORS => WIN_POINTS,
                _ => return Err("invalid result"),
            },
        OPPONENT_SCISSORS =>
            match you {
                YOU_PAPER => LOSE_POINTS,
                YOU_SCISSORS => DRAW_POINTS,
                YOU_ROCK => WIN_POINTS,
                _ => return Err("invalid result"),
            },
        _ => return Err("invalid result"),
    };

    Ok(outcome_score + shape_score)
}

fn calculate_total(input: String) -> i32 {
    let mut score = 0;

    for line in input.lines() {
        let opponent = match line.split_whitespace().nth(0) {
            Some(x) => x,
            None => continue,
        };

        let you = match line.split_whitespace().nth(1) {
            Some(x) => x,
            None => continue,
        };

        let r = match round(opponent, you) {
            Ok(x) => x,
            Err(_) => continue,
        };
        
        score += r;
    }
    
    score
}

fn main() { 
    gflags::parse();
    if HELP.flag || !FILE.is_present() {
        gflags::print_help_and_exit(0);
    }

    let input = fs::read_to_string(FILE.flag)
        .expect("unable to read input");

    println!("your total score is: {}", calculate_total(input));
}
