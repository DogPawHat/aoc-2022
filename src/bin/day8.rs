use std::{fs, str::FromStr};

use anyhow::bail;
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

#[derive(Debug)]
struct ElfForest(Vec<Vec<ElfTree>>);

struct UpForestIter{
    forest: Vec<Vec<ElfTree>>,
    x: usize,
    y: usize,
}

impl UpForestIter {
    pub fn iter(&self) -> U
}

impl Iterator for UpForestIter {
    type Item = &ElfTree;

    fn next(&mut self) -> Option<Self::Item> {
        let next_item = self.forest.get(self.x)?.get(self.y)?;
        self.y = self.y - 1;
        Some(next_item)
    }
    
}

impl FromStr for ElfForest {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let forest_map_res: Result<Vec<Vec<_>>> = s
            .lines()
            .map(|line| {
                let column = line
                    .chars()
                    .map(|s| {
                        let height = String::from(s).parse()?;
                        Ok(ElfTree { height })
                    })
                    .collect();
                column
            })
            .collect();

        let forest_map = forest_map_res?;

        assert!(forest_map.first().unwrap().len() == forest_map.last().unwrap().len());
        assert!(forest_map[0].len() == forest_map[1].len());
        assert!(forest_map[1].len() == forest_map.last().unwrap().len());

        Ok(Self(forest_map))
    }
}

impl ElfForest {
    fn get_max_x(&self) -> usize {
        self.0[0].len()
    }

    fn get_max_y(&self) -> usize {
        self.0.len()
    }

    fn is_tree_visible(&self, x: usize, y: usize) -> bool {
        let tree = self.0.get(x).unwrap().get(y).unwrap();

    }
}


fn part1(_forest: &ElfForest) -> Result<u32> {
    todo!()
}

fn main() -> Result<()> {
    let forest: ElfForest = INPUT_FILE.parse()?;

    println!(
        "Part 1 - Trees visible from outside grid: <{}>",
        part1(&forest)?
    );
    Ok(())
}
