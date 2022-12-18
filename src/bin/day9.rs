use std::{fs, ops::Index, str::FromStr, collections::HashSet};

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
    static ref TEST_EXAMPLE_DIRECTIONS: Vec<Direction> = vec![
        Direction::Right(4),
        Direction::Up(4),
        Direction::Left(3),
        Direction::Down(1),
        Direction::Right(4),
        Direction::Down(1),
        Direction::Left(5),
        Direction::Right(2),
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ElfRopePos {
    x: u32,
    y: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ElfRopeState {
    head: ElfRopePos,
    tail: ElfRopePos,
}

trait AssociatedPos {
    type Pos;
}

impl AssociatedPos for ElfRopePos {
    type Pos = ElfRopePos;
}

impl ElfRopeState {
    fn process_direction(&self, rope_direction: Direction) -> Self {
        match rope_direction {
            Direction::Up(idx) => {
                let new_head = ElfRopePos {
                    x: self.head.x + idx,
                    y: self.head.y,
                };

                let mut new_tail = self.tail;

                if self.tail.x < new_head.x - 1 {
                    new_tail = ElfRopePos {
                        x: new_head.x - 1,
                        y: new_head.y,
                    };
                }

                Self {
                    head: new_head,
                    tail: new_tail,
                }
            }
            Direction::Down(idx) => {
                let new_head = ElfRopePos {
                    x: self.head.x - idx,
                    y: self.head.y,
                };

                let mut new_tail = self.tail;

                if self.tail.x > new_head.x + 1 {
                    new_tail = ElfRopePos {
                        x: new_head.x + 1,
                        y: new_head.y,
                    };
                }

                Self {
                    head: new_head,
                    tail: new_tail,
                }
            }
            Direction::Left(idx) => {
                let new_head = ElfRopePos {
                    x: self.head.x,
                    y: self.head.y - idx,
                };

                let mut new_tail = self.tail;

                if self.tail.y > new_head.y + 1 {
                    new_tail = ElfRopePos {
                        x: new_head.x,
                        y: new_head.y + 1,
                    };
                }

                Self {
                    head: new_head,
                    tail: new_tail,
                }
            }
            Direction::Right(idx) => {
                let new_head = ElfRopePos {
                    x: self.head.x,
                    y: self.head.y + idx,
                };

                let mut new_tail = self.tail;

                if self.tail.y < new_head.y - 1 {
                    new_tail = ElfRopePos {
                        x: new_head.x,
                        y: new_head.y - 1,
                    };
                }

                Self {
                    head: new_head,
                    tail: new_tail,
                }
            }
        }
    }
}

impl FromStr for Direction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let splitted = s.split_once(" ").context("Failure to split")?;

        let direction_vector: u32 = splitted.1.parse()?;

        match splitted.0 {
            "U" => Ok(Direction::Up(direction_vector)),
            "D" => Ok(Direction::Down(direction_vector)),
            "L" => Ok(Direction::Left(direction_vector)),
            "R" => Ok(Direction::Right(direction_vector)),
            _ => bail!("Failure to determine direction"),
        }
    }
}

fn part1(directions: Vec<Direction>) -> usize {
    let init_state = ElfRopeState{
        tail: ElfRopePos{
            x: 0,
            y: 0,
        },
        head: ElfRopePos{
            x: 0,
            y: 0
        }
    };

    let mut tail_pos_set = HashSet::from([init_state.tail]);

    let mut rope_state = init_state;
    for direction in directions {
        rope_state = rope_state.process_direction(direction);
        tail_pos_set.insert(rope_state.tail);
    }

    tail_pos_set.len()
}

fn main() -> Result<()> {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{Direction, EXAMPLE_FILE, TEST_EXAMPLE_DIRECTIONS, part1};

    #[test]
    fn it_parses_files_correctly() {
        let parsed_direction: anyhow::Result<Vec<Direction>> =
            EXAMPLE_FILE.lines().map(|line| line.parse()).collect();

        assert!(parsed_direction.is_ok());
        assert_eq!(parsed_direction.unwrap(), *TEST_EXAMPLE_DIRECTIONS);
    }

    #[test]
    fn it_runs_part_1_for_example() {
        let parsed_direction: anyhow::Result<Vec<Direction>> =
            EXAMPLE_FILE.lines().map(|line| line.parse()).collect();

        assert!(parsed_direction.is_ok());
        assert_eq!(part1(parsed_direction.unwrap()), 13);
    }
}
