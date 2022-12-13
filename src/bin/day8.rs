use std::fs;

use anyhow::{anyhow, bail};
use lazy_static::lazy_static;

pub type Error = anyhow::Error;
pub type Result<T> = anyhow::Result<T>;

const INPUT_PATH: &str = "inputs/day8.txt";

lazy_static! {
    static ref INPUT_FILE: String =
        fs::read_to_string(INPUT_PATH).expect("Day 7 - Inputs: Can't parse stacks");
}

fn part1(file: &str) -> Result<u32> {
    todo!()
}

fn main() -> Result<()> {
    println!(
        "Part 1 - Trees visible from outside grid: <{}>",
        part1(&INPUT_FILE)?
    );
    Ok(())
}
