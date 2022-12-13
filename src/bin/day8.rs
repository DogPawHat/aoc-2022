use std::{collections::HashMap, fs, str::FromStr};

use anyhow::{bail};
use lazy_static::lazy_static;

pub type Error = anyhow::Error;
pub type Result<T> = anyhow::Result<T>;

const INPUT_PATH: &str = "inputs/day8.txt";

lazy_static! {
    static ref INPUT_FILE: String =
        fs::read_to_string(INPUT_PATH).expect("Day 7 - Inputs: Can't parse stacks");
}

#[derive(Debug)]
struct ElfTree {
    height: u32,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct ElfTreePos {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct ElfForest(HashMap<ElfTreePos, ElfTree>);

impl FromStr for ElfForest {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut forest_map = HashMap::new();

        let mut current_y = 0;
        for line in s.lines() {
            let mut current_x = 0;
            for tree_char in line.chars() {
                match u32::from_str(&String::from(tree_char)) {
                    Ok(height) => {
                        forest_map.insert(
                            ElfTreePos {
                                x: current_x,
                                y: current_y,
                            },
                            ElfTree { height },
                        );
                    }
                    Err(err) => bail!("Issue with: {:?}", err),
                }
                current_x = current_x + 1;
            }
            current_y = current_y + 1;
        }

        Ok(Self(forest_map))
    }
}

fn part1(_forest: &ElfForest) -> Result<u32> {
    todo!()
}

fn main() -> Result<()> {
    let forest = ElfForest::from_str(&INPUT_FILE)?;

    println!(
        "Part 1 - Trees visible from outside grid: <{}>",
        part1(&forest)?
    );
    Ok(())
}
