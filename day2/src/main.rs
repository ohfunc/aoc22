use std::fs;

// Appreciative of your help yesterday, one Elf gives you an encrypted strategy 
// guide (your puzzle input) that they say will be sure to help you win. "The 
// first column is what your opponent is going to play: A for Rock, B for Paper, 
// and C for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.
// 
// The second column, you reason, must be what you should play in response: X 
// for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, 
// so the responses must have been carefully chosen.
// 
// The winner of the whole tournament is the player with the highest score. 
// Your total score is the sum of your scores for each round. The score for a 
// single round is the score for the shape you selected (1 for Rock, 2 for Paper,
// and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 
// 3 if the round was a draw, and 6 if you won).
//
// For example, suppose you were given the following strategy guide:
//
// A Y
// B X
// C Z
// 
// This strategy guide predicts and recommends the following:
//
// In the first round, your opponent will choose Rock (A), and you should choose
// Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
//
// In the second round, your opponent will choose Paper (B), and you should choose 
// Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
//
// The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.
//
// In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).
//
// What would your total score be if everything goes exactly according to your strategy guide?

// PAPER beats ROCK
// ROCK beats SCISSORS
// SCISSORS beats PAPER

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

#[derive(Debug)]
enum Results {
    UNKNOWN,
    LOSE,
    DRAW,
    WIN
}

fn round(opponent: &str, you: &str) -> i32 {
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

    return outcome_score + shape_score;
}

fn calculate_score(input: String) -> i32 {
    let mut score = 0;

    for line in input.lines() {
        // Unwrapping is bad and you should feel bad.
        let opponent = line.split_whitespace().nth(0).unwrap();
        let you = line.split_whitespace().nth(1).unwrap();

        score += round(opponent, you);
    }
    
    return score;
}

fn main() { 
    let file_path = "./input.txt";
    let input = fs::read_to_string(file_path)
        .expect("unable to read input");

    println!("your total score is: {}", calculate_score(input));
}
