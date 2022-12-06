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

fn count_to_first_marker(input: &str, marker_size: usize) -> u32 {
    let mut count = marker_size as u32;
    let mut input_seeker = input.chars();

    for _ in 0..input.len() {
        let char_set: HashSet<char> = input_seeker.clone().take(marker_size).collect();
        if char_set.len() == marker_size {
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
        count_to_first_marker(&STACKS_FILE, SOP_SIZE)
    );
    println!(
        "Part 2 - Chars to first start-of-message: <{}>",
        count_to_first_marker(&STACKS_FILE, SOM_SIZE)
    );
    Ok(())
}
