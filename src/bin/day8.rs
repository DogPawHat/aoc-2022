use std::{fs, str::FromStr};

use anyhow::Context;
use lazy_static::lazy_static;

pub type Error = anyhow::Error;
pub type Result<T> = anyhow::Result<T>;

const INPUT_PATH: &str = "inputs/day8.txt";

lazy_static! {
    static ref INPUT_FILE: String =
        fs::read_to_string(INPUT_PATH).expect("Day 7 - Inputs: Can't parse stacks");
}

#[derive(Debug)]
struct ElfForest(Vec<Vec<u32>>, usize, usize);

impl FromStr for ElfForest {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        let grid: Vec<Vec<u32>> = input
            .trim()
            .lines()
            .map(|row| {
                row.chars()
                    .map(|c| c.to_digit(10).context("Invalid digit"))
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<_>>>()?;
        let rows = grid.len() - 1;
        let cols = grid[0].len() - 1;
        Ok(Self(grid, rows, cols))
    }
}

fn part1(ElfForest(grid, rows, cols): &ElfForest) -> u32 {
    (1..*rows).fold((2 * (rows + cols)) as u32, |visible, row| {
        (1..*cols).fold(visible, |visible, col| {
            let height = grid[row][col];
            if (0..col).all(|i| grid[row][i] < height)
                || (0..row).all(|j| grid[j][col] < height)
                || (col + 1..=*cols).all(|i| grid[row][i] < height)
                || (row + 1..=*rows).all(|j| grid[j][col] < height)
            {
                visible + 1
            } else {
                visible
            }
        })
    })
}

fn part2(ElfForest(grid, rows, cols): &ElfForest) -> u32 {
    (1..*rows).fold(0u32, |score, row| {
        (1..*cols).fold(score, |score, col| {
            let height = grid[row][col];
            let left = (0..col)
                .enumerate()
                .rev()
                .find(|(_, c)| grid[row][*c] >= height)
                .map(|(i, _)| col - i)
                .unwrap_or(col);
            let right = (col + 1..=*cols)
                .enumerate()
                .find(|(_, c)| grid[row][*c] >= height)
                .map(|(i, _)| i + 1)
                .unwrap_or(cols - col);
            let up = (0..row)
                .enumerate()
                .rev()
                .find(|(_, r)| grid[*r][col] >= height)
                .map(|(i, _)| row - i)
                .unwrap_or(row);
            let down = (row + 1..=*rows)
                .enumerate()
                .find(|(_, r)| grid[*r][col] >= height)
                .map(|(i, _)| i + 1)
                .unwrap_or(rows - row);
            score.max((left * up * right * down) as u32)
        })
    })
}

fn main() -> Result<()> {
    let forest: ElfForest = INPUT_FILE.parse()?;

    println!(
        "Part 1 - Trees visible from outside grid: <{}>",
        part1(&forest)
    );

    println!(
        "Part 2 - Highest scenic score for trees: <{}>",
        part2(&forest)
    );

    Ok(())
}
