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

const WANT_LOSE: &str = "X";
const WANT_DRAW: &str = "Y";
const WANT_WIN: &str = "Z";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_draw() {
        assert_eq!(round(OPPONENT_ROCK, WANT_DRAW), Ok(4));
    }

    #[test]
    fn round_lose() {
        assert_eq!(round(OPPONENT_PAPER, WANT_LOSE), Ok(1));
    }

    #[test]
    fn round_win() {
        assert_eq!(round(OPPONENT_SCISSORS, WANT_WIN), Ok(7));
    }

    #[test]
    fn round_bad_input() {
        assert_eq!(round(OPPONENT_PAPER, "D"), Err("invalid strat input"));
        assert_eq!(round("X", WANT_WIN), Err("invalid result"));
    }

    #[test]
    fn calculate_total_test() {
        let input = "A Y
B X
C Z
        ".to_string();

        assert_eq!(calculate_total(input), 12);
    }
}

fn round(opponent: &str, strat: &str) -> Result<i32, &'static str> {
    let outcome_score = match strat {
        WANT_LOSE => LOSE_POINTS,
        WANT_DRAW => DRAW_POINTS,
        WANT_WIN => WIN_POINTS,
        _ => return Err("invalid strat input"),
    };

    let shape_score = match opponent {
        OPPONENT_ROCK => 
            match strat {
                WANT_LOSE => SCISSORS_POINTS,
                WANT_DRAW => ROCK_POINTS,
                WANT_WIN => PAPER_POINTS,
                _ => return Err("invalid result"),
            },
        OPPONENT_PAPER =>
            match strat {
                WANT_LOSE => ROCK_POINTS,
                WANT_DRAW => PAPER_POINTS,
                WANT_WIN => SCISSORS_POINTS,
                _ => return Err("invalid result"),
            },
        OPPONENT_SCISSORS =>
            match strat {
                WANT_LOSE => PAPER_POINTS,
                WANT_DRAW => SCISSORS_POINTS,
                WANT_WIN => ROCK_POINTS,
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
