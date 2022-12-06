use std::collections::HashSet;
use std::fs;

use anyhow::{Ok, Result};
use lazy_static::lazy_static;

const INPUT_PATH: &str = "inputs/day6.txt";
const SOP_SIZE: usize = 4;
const SOM_SIZE: usize = 14;

lazy_static! {
    static ref STACKS_FILE: String =
        fs::read_to_string(INPUT_PATH).expect("Day5 - Inputs: Can't parse stacks");
}

fn part1(input: &str) -> u32 {
    let mut count = SOP_SIZE as u32;
    let mut input_seeker = input.chars();

    for _ in 0..input.len() {
        let char_set: HashSet<char> = input_seeker.clone().take(SOP_SIZE).collect();
        if char_set.len() == SOP_SIZE {
            break;
        }
        count = count + 1;
        input_seeker.next();
    }
    count
}

fn part2(input: &str) -> u32 {
    let mut count = SOM_SIZE as u32;
    let mut input_seeker = input.chars();

    for _ in 0..input.len() {
        let char_set: HashSet<char> = input_seeker.clone().take(SOM_SIZE).collect();
        if char_set.len() == SOM_SIZE {
            break;
        }
        count = count + 1;
        input_seeker.next();
    }
    count
}

fn main() -> Result<()> {
    println!(
        "Part 1 - Chars to first start-of-packet: <{}>",
        part1(&STACKS_FILE)
    );
    println!(
        "Part 2 - Chars to first start-of-message: <{}>",
        part2(&STACKS_FILE)
    );
    Ok(())
}
