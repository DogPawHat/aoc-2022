use std::fs;

const INPUT_PATH_STR: &str = "aoc-solution-2/input.txt";

#[derive(Debug)]
enum ElfRPSError {
    CharToPlayErr,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum RPSPlay {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Eq, PartialEq)]
enum RPSMatchResult {
    Win,
    Loss,
    Draw,
}

#[derive(Debug)]
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

fn get_required_response(enemy: &RPSPlay, desired_result: &RPSMatchResult) -> RPSPlay {
    match (enemy, desired_result) {
        (a, b) if b == &RPSMatchResult::Draw => a.clone(),
        (RPSPlay::Rock, RPSMatchResult::Win) => RPSPlay::Paper,
        (RPSPlay::Rock, RPSMatchResult::Loss) => RPSPlay::Scissors,
        (RPSPlay::Paper, RPSMatchResult::Win ) => RPSPlay::Scissors,
        (RPSPlay::Paper,  RPSMatchResult::Loss) => RPSPlay::Rock,
        (RPSPlay::Scissors, RPSMatchResult::Win) => RPSPlay::Rock,
        (RPSPlay::Scissors, RPSMatchResult::Loss) => RPSPlay::Paper,
        (_, _) => enemy.clone(),
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

fn map_str_to_result(val: &str) -> Result<RPSMatchResult, ElfRPSError> {
    match val {
        "X" => Ok(RPSMatchResult::Loss),
        "Y" => Ok(RPSMatchResult::Draw),
        "Z" => Ok(RPSMatchResult::Win),
        _ => Err(ElfRPSError::CharToPlayErr),
    }
}

impl RPSMatch {
    fn new(enemy: RPSPlay, desired_result: RPSMatchResult) -> Self {
        let response = get_required_response(&enemy, &desired_result);
        let result = resolve_match(&enemy, &response);
        let shape_score = score_shape(&response);
        let match_score = score_result(&result);
        assert!(desired_result == result, "Results are not good");
        RPSMatch(shape_score + match_score)
    }

    fn from_str_pair(enemy: &str, desired_result: &str) -> Result<Self, ElfRPSError> {
        Ok(Self::new(
            map_enemy_to_play(enemy)?,
            map_str_to_result(desired_result)?,
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
