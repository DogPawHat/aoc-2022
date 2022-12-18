use std::{fs, str::FromStr};

use anyhow::{bail, Context};
use lazy_static::lazy_static;

pub type Error = anyhow::Error;
pub type Result<T> = anyhow::Result<T>;

const EXAMPLE_INPUT_PATH: &str = "inputs/day8/example.txt";
const PUZZLE_INPUT_PATH: &str = "inputs/day8/puzzle.txt";

lazy_static! {
    static ref EXAMPLE_FILE: String =
        fs::read_to_string(EXAMPLE_INPUT_PATH).expect("Day 9 - Can't parse example file");
    static ref PUZZLE_FILE: String =
        fs::read_to_string(PUZZLE_INPUT_PATH).expect("Day 9 - Can't parse puzzle file");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up(u8),
    Down(u8),
    Left(u8),
    Right(u8),
}

impl FromStr for Direction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let splitted = s.split_once(" ").context("Failure to split")?;

        let direction_vector: u8 = splitted.1.parse()?;

        match splitted.0 {
            "U" => Ok(Direction::Up(direction_vector)),
            "D" => Ok(Direction::Down(direction_vector)),
            "L" => Ok(Direction::Left(direction_vector)),
            "R" => Ok(Direction::Right(direction_vector)),
            _ => bail!("Failure to determine direction"),
        }
    }
}

// fn part1(forest: &ElfForest) -> Result<usize> {
//     let visiblity_matrix = make_full_matrix(&forest);

//     println!("{}", visiblity_matrix.to_string());

//     Ok(visiblity_matrix.count_visible_trees())
// }

fn main() -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::Direction;
    use crate::EXAMPLE_FILE;

    #[test]
    fn it_parses_files_correctly() {
        let expected_vec = vec![
            Direction::Right(4),
            Direction::Up(4),
            Direction::Left(3),
            Direction::Down(1),
            Direction::Right(4),
            Direction::Down(1),
            Direction::Left(5),
            Direction::Right(2),
        ];

        let parsed_direction: anyhow::Result<Vec<Direction>> =
            EXAMPLE_FILE.lines().map(|line| line.parse()).collect();

        assert!(parsed_direction.is_ok());
        assert_eq!(parsed_direction.unwrap(), expected_vec);
    }
}
