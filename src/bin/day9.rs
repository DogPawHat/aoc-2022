use std::{collections::HashSet, fs, str::FromStr};

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
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ElfRopePos {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ElfRopeState {
    head: ElfRopePos,
    tail: ElfRopePos,
}

struct ElfRopeStateStepper {
    direction: Option<Direction>,
    current_state: ElfRopeState,
}

impl Iterator for ElfRopeStateStepper {
    type Item = ElfRopeState;

    fn next(&mut self) -> Option<Self::Item> {
        let remaining_direction = self.direction?;

        let next_state = match remaining_direction {
            Direction::Up(idx) => {
                let next_x = self.current_state.head.x + 1;
                let remaining_steps = idx - 1;
                if remaining_steps == 0 {
                    self.direction = None;
                } else {
                    self.direction = Some(Direction::Up(remaining_steps));
                }
                let next_head = ElfRopePos {
                    x: next_x,
                    y: self.current_state.head.y,
                };

                let mut next_tail = self.current_state.tail;
                if next_tail.x < next_head.x - 1 {
                    next_tail = ElfRopePos {
                        x: next_head.x - 1,
                        y: next_head.y,
                    };
                }
                ElfRopeState {
                    head: next_head,
                    tail: next_tail,
                }
            }
            Direction::Down(idx) => {
                let next_x = self.current_state.head.x - 1;
                let remaining_steps = idx - 1;
                if remaining_steps == 0 {
                    self.direction = None;
                } else {
                    self.direction = Some(Direction::Down(remaining_steps));
                }
                let next_head = ElfRopePos {
                    x: next_x,
                    y: self.current_state.head.y,
                };

                let mut next_tail = self.current_state.tail;
                if next_tail.x > next_head.x + 1 {
                    next_tail = ElfRopePos {
                        x: next_head.x + 1,
                        y: next_head.y,
                    };
                }
                ElfRopeState {
                    head: next_head,
                    tail: next_tail,
                }
            }
            Direction::Left(idx) => {
                let next_y = self.current_state.head.y - 1;
                let remaining_steps = idx - 1;
                if remaining_steps == 0 {
                    self.direction = None;
                } else {
                    self.direction = Some(Direction::Down(remaining_steps));
                }
                let next_head = ElfRopePos {
                    x: self.current_state.head.x,
                    y: next_y,
                };

                let mut next_tail = self.current_state.tail;
                if next_tail.y > next_head.y + 1 {
                    next_tail = ElfRopePos {
                        x: next_head.x,
                        y: next_head.y + 1,
                    };
                }
                ElfRopeState {
                    head: next_head,
                    tail: next_tail,
                }
            }
            Direction::Right(idx) => {
                let next_y = self.current_state.head.y + 1;
                let remaining_steps = idx - 1;
                if remaining_steps == 0 {
                    self.direction = None;
                } else {
                    self.direction = Some(Direction::Down(remaining_steps));
                }
                let next_head = ElfRopePos {
                    x: self.current_state.head.x,
                    y: next_y,
                };

                let mut next_tail = self.current_state.tail;
                if next_tail.y < next_head.y - 1 {
                    next_tail = ElfRopePos {
                        x: next_head.x,
                        y: next_head.y - 1,
                    };
                }
                ElfRopeState {
                    head: next_head,
                    tail: next_tail,
                }
            }
        };

        self.current_state = next_state;
        Some(next_state)
    }
}

impl ElfRopeState {
    fn rope_state_iter(&self, direction: Direction) -> ElfRopeStateStepper {
        ElfRopeStateStepper {
            direction: Some(direction),
            current_state: *self,
        }
    }
}

impl FromStr for Direction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let splitted = s.split_once(" ").context("Failure to split")?;

        let direction_vector: i32 = splitted.1.parse()?;

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
    let init_state = ElfRopeState {
        tail: ElfRopePos { x: 0, y: 0 },
        head: ElfRopePos { x: 0, y: 0 },
    };

    let mut tail_pos_set = HashSet::from([init_state.tail]);

    let mut rope_state = init_state;
    let mut next_rope_state = init_state;

    for direction in directions {
        for new_rope_state in rope_state.rope_state_iter(direction) {
            tail_pos_set.insert(new_rope_state.tail);
            next_rope_state = new_rope_state;
        }
        rope_state = next_rope_state;
    }

    tail_pos_set.len()
}

fn main() -> Result<()> {
    let parsed_puzzle_directions_res: Result<Vec<_>> =
        PUZZLE_FILE.lines().map(|line| line.parse()).collect();

    println!(
        "Day 9 - Rope Tail Visits: <{}>",
        part1(parsed_puzzle_directions_res?)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, Direction, EXAMPLE_FILE, TEST_EXAMPLE_DIRECTIONS};

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
