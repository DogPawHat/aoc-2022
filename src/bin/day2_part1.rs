use std::fs;

const INPUT_PATH_STR: &str = "inputs/day2.txt";

#[derive(Debug)]
enum ElfRPSError {
    CharToPlayErr,
}

#[derive(Debug, PartialEq)]
enum RPSPlay {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
enum RPSMatchResult {
    Win,
    Loss,
    Draw,
}

struct RPSMatch(i32);

fn score_shape(shape: &RPSPlay) -> i32 {
    match shape {
        RPSPlay::Rock => 1,
        RPSPlay::Paper => 2,
        RPSPlay::Scissors => 3,
    }
}

fn score_result(result: &RPSMatchResult) -> i32 {
    match result {
        RPSMatchResult::Loss => 0,
        RPSMatchResult::Draw => 3,
        RPSMatchResult::Win => 6,
    }
}

fn resolve_match(enemy: &RPSPlay, response: &RPSPlay) -> RPSMatchResult {
    match (enemy, response) {
        (a, b) if a == b => RPSMatchResult::Draw,
        (RPSPlay::Rock, RPSPlay::Paper) => RPSMatchResult::Win,
        (RPSPlay::Rock, RPSPlay::Scissors) => RPSMatchResult::Loss,
        (RPSPlay::Paper, RPSPlay::Scissors) => RPSMatchResult::Win,
        (RPSPlay::Paper, RPSPlay::Rock) => RPSMatchResult::Loss,
        (RPSPlay::Scissors, RPSPlay::Rock) => RPSMatchResult::Win,
        (RPSPlay::Scissors, RPSPlay::Paper) => RPSMatchResult::Loss,
        (_, _) => RPSMatchResult::Draw,
    }
}

fn map_enemy_to_play(val: &str) -> Result<RPSPlay, ElfRPSError> {
    match val {
        "A" => Ok(RPSPlay::Rock),
        "B" => Ok(RPSPlay::Paper),
        "C" => Ok(RPSPlay::Scissors),
        _ => Err(ElfRPSError::CharToPlayErr),
    }
}

fn map_response_to_play(val: &str) -> Result<RPSPlay, ElfRPSError> {
    match val {
        "X" => Ok(RPSPlay::Rock),
        "Y" => Ok(RPSPlay::Paper),
        "Z" => Ok(RPSPlay::Scissors),
        _ => Err(ElfRPSError::CharToPlayErr),
    }
}

impl RPSMatch {
    fn new(enemy: RPSPlay, response: RPSPlay) -> Self {
        let result = resolve_match(&enemy, &response);
        let shape_score = score_shape(&response);
        let match_score = score_result(&result);
        RPSMatch(match_score + shape_score)
    }

    fn from_str_pair(enemy: &str, response: &str) -> Result<Self, ElfRPSError> {
        Ok(Self::new(
            map_enemy_to_play(enemy)?,
            map_response_to_play(response)?,
        ))
    }
}

fn main() {
    let score: i32 = fs::read_to_string(INPUT_PATH_STR)
        .expect("Error reading input")
        .lines()
        .map(|line| (&line[0..1], &line[2..3]))
        .map(|(enemy, response)| RPSMatch::from_str_pair(enemy, response))
        .map(|rps_match_res| rps_match_res.expect("Error parsing match"))
        .map(|m| m.0)
        .sum();

    println!("Hello, world! Your Score is: <{}>", score);
}
