use std::fs;

use anyhow::{anyhow, bail};
use lazy_static::lazy_static;

pub type Error = anyhow::Error;
pub type Result<T> = anyhow::Result<T>;

const INPUT_PATH: &str = "inputs/day8.txt";
const BIG_DIRECTORY_SIZE: u32 = 100000;

lazy_static! {
    static ref INPUT_FILE: String =
        fs::read_to_string(INPUT_PATH).expect("Day 7 - Inputs: Can't parse stacks");
}

fn main() -> Result<()> {
    println!(
        "Part 1 - Total size of directories with size > {}: <{}>",
        BIG_DIRECTORY_SIZE,
        part1(&INPUT_FILE)?
    );
    Ok(())
}
