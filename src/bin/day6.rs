use std::fmt;
use std::fs;
use std::str::FromStr;

use anyhow::{Error, Ok, Result};
use lazy_static::lazy_static;
use regex::Regex;

const STACKS_INPUT: &str = "inputs/day5/stacks.csv";

const MOVES_INPUT: &str = "inputs/day5/moves.txt";

lazy_static! {
    static ref STACKS_FILE: String =
        fs::read_to_string(STACKS_INPUT).expect("Day5 - Inputs: Can't parse stacks");
    static ref MOVES_FILE: String =
        fs::read_to_string(MOVES_INPUT).expect("Day5 - Inputs: Can't parse moves");
    static ref MOVE_REGEX: Regex = Regex::new(
        r"move (?P<move_num>[0-9]+) from (?P<from_stack>[0-9]+) to (?P<to_stack>[0-9]+)"
    )
    .expect("Day 5 - Inputs: Effed up the regex");
}

fn main() -> Result<()> {
    let elf_stacks: ElfCrateStacks = STACKS_FILE.parse()?;
    let elf_moves: ElfCrateMoves = MOVES_FILE.parse()?;

    let part1_score = part1(&elf_stacks, &elf_moves);
    let part2_score = part2(&elf_stacks, &elf_moves);

    println!(
        "Part 1 - CrateMover 9000: <{}>",
        part1_score.get_tops_of_stacks().to_string()
    );
    println!(
        "Part 1 - CrateMover 9001: <{}>",
        part2_score.get_tops_of_stacks().to_string()
    );
    Ok(())
}
