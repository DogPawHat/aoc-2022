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

#[derive(Debug, Clone)]
struct ElfCrateStacks {
    stacks: Vec<Vec<String>>,
}

#[derive(Debug, Clone)]
struct ElfCrateStackTops {
    stack_tops: Vec<String>,
}

#[derive(Debug, Clone)]
struct ElfCrateMove {
    move_num: usize,
    from_stack: usize,
    to_stack: usize,
}

#[derive(Debug, Clone)]
struct ElfCrateMoves(Vec<ElfCrateMove>);

impl ElfCrateStacks {
    fn get_tops_of_stacks(&self) -> ElfCrateStackTops {
        let stack_tops: Vec<String> = self
            .stacks
            .iter()            
            .map(|stack| stack.last().map(String::as_str).unwrap_or_else(|| " "))
            .map(String::from)
            .collect();

        ElfCrateStackTops { stack_tops }
    }

    fn move_crates(&mut self, elf_move: &ElfCrateMove) {
        for _ in 0..elf_move.move_num {
            if let Some(moved_crate) = self.stacks[elf_move.from_stack - 1].pop() {
                self.stacks[elf_move.to_stack - 1].push(moved_crate);
            }
        }
    }
}

impl FromStr for ElfCrateStacks {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stacks: Vec<Vec<String>> = s
            .lines()
            .map(|l| l.split(",").skip(1).map(String::from).collect())
            .collect();

        Ok(Self { stacks })
    }
}

impl fmt::Display for ElfCrateStackTops {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stack_top_str: String = self.stack_tops.clone().into_iter().collect();
        write!(f, "{}", stack_top_str)
    }
}

impl FromStr for ElfCrateMoves {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let move_vec: Result<Vec<ElfCrateMove>> = MOVE_REGEX
            .captures_iter(s)
            .map(|cap| {
                let move_num: usize = cap["move_num"].parse()?;
                let from_stack: usize = cap["from_stack"].parse()?;
                let to_stack: usize = cap["to_stack"].parse()?;
                Ok(ElfCrateMove {
                    move_num,
                    from_stack,
                    to_stack,
                })
            })
            .collect();
        Ok(Self(move_vec?))
    }
}

fn part1(elf_stacks: &ElfCrateStacks, elf_moves: &ElfCrateMoves) -> ElfCrateStacks {
    let mut stacks_workspace = elf_stacks.clone();
    for crate_move in elf_moves.0.iter() {
        stacks_workspace.move_crates(&crate_move);
    }
    stacks_workspace
}

fn main() -> Result<()> {
    let elf_stacks: ElfCrateStacks = STACKS_FILE.parse()?;
    let elf_moves: ElfCrateMoves = MOVES_FILE.parse()?;

    let part1_score = part1(&elf_stacks, &elf_moves);

    println!("Part 1: <{}>", part1_score.get_tops_of_stacks().to_string());
    Ok(())
}
