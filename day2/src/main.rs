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

#[derive(Debug, PartialEq)]
enum Results {
    UNKNOWN,
    LOSE,
    DRAW,
    WIN
}

fn round(opponent: &str, you: &str) -> Result<i32, &'static str> {
    let res = match opponent {
        OPPONENT_ROCK => 
            match you {
                YOU_SCISSORS => Results::LOSE,
                YOU_ROCK => Results::DRAW,
                YOU_PAPER => Results::WIN,
                _ => Results::UNKNOWN,
            },
        OPPONENT_PAPER =>
            match you {
                YOU_ROCK => Results::LOSE,
                YOU_PAPER => Results::DRAW,
                YOU_SCISSORS => Results::WIN,
                _ => Results::UNKNOWN,
            },
        OPPONENT_SCISSORS =>
            match you {
                YOU_PAPER => Results::LOSE,
                YOU_SCISSORS => Results::DRAW,
                YOU_ROCK => Results::WIN,
                _ => Results::UNKNOWN,
            },
        _ => Results::UNKNOWN,
    };

    if res == Results::UNKNOWN {
        return Err("invalid result");
    }

    let shape_score = match you {
        YOU_ROCK => ROCK_POINTS,
        YOU_PAPER => PAPER_POINTS,
        YOU_SCISSORS => SCISSORS_POINTS,
        _ => 0,
    };

    let outcome_score = match res {
        Results::UNKNOWN => 0,
        Results::LOSE => LOSE_POINTS,
        Results::DRAW => DRAW_POINTS,
        Results::WIN => WIN_POINTS,
    };

    return Ok(outcome_score + shape_score);
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
    
    return score;
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
