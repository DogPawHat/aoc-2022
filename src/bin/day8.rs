use std::{fmt, fs, str::FromStr};

use itertools::Itertools;
use lazy_static::lazy_static;
use colored::{Colorize, ColoredString};

pub type Error = anyhow::Error;
pub type Result<T> = anyhow::Result<T>;

const INPUT_PATH: &str = "inputs/day8.txt";

lazy_static! {
    static ref INPUT_FILE: String =
        fs::read_to_string(INPUT_PATH).expect("Day 7 - Inputs: Can't parse stacks");
}

#[derive(Clone, Copy, Debug)]
struct ElfTree {
    height: u32,
}

#[derive(Clone, Copy, Debug)]
struct ElfTreeVisible {
    height: u32,
    visible: bool,
}

#[derive(Debug)]
struct ElfForest(Vec<Vec<ElfTree>>);

#[derive(Debug)]
struct ElfForestVisiblity(Vec<Vec<ElfTreeVisible>>);

fn make_edge_vec(tree_vec: &Vec<ElfTree>) -> Vec<ElfTreeVisible> {
    tree_vec
        .iter()
        .map(|tree| ElfTreeVisible {
            height: tree.height,
            visible: true,
        })
        .collect()
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
    fn northern_visibilty_matrix(&self) -> ElfForestVisiblity {
        let empty_vec = vec![];
        let mut tallest_trees_north = make_edge_vec(self.0.first().unwrap_or_else(|| &empty_vec));

        let matrix: Vec<Vec<_>> = self
            .0
            .iter()
            .enumerate()
            .map(|(idx, inner_vec)| {
                if idx == 0 {
                    make_edge_vec(inner_vec)
                } else {
                    inner_vec
                        .iter()
                        .enumerate()
                        .map(|(inner_idx, tree)| {
                            let mut visible_tree = ElfTreeVisible {
                                height: tree.height,
                                visible: false,
                            };

                            if tree.height > tallest_trees_north[inner_idx].height {
                                visible_tree.visible = true;
                                tallest_trees_north[inner_idx] = visible_tree
                            }

                            visible_tree
                        })
                        .collect()
                }
            })
            .collect();

        ElfForestVisiblity(matrix)
    }

    fn southern_visibilty_matrix(&self) -> ElfForestVisiblity {
        let empty_vec = vec![];
        let mut tallest_trees_south = make_edge_vec(self.0.last().unwrap_or_else(|| &empty_vec));

        let matrix: Vec<Vec<_>> = self
            .0
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, inner_vec)| {
                if idx == 0 {
                    make_edge_vec(inner_vec)
                } else {
                    inner_vec
                        .iter()
                        .enumerate()
                        .map(|(inner_idx, tree)| {
                            let mut visible_tree = ElfTreeVisible {
                                height: tree.height,
                                visible: false,
                            };

                            if tree.height > tallest_trees_south[inner_idx].height {
                                visible_tree.visible = true;
                                tallest_trees_south[inner_idx] = visible_tree
                            }

                            visible_tree
                        })
                        .collect()
                }
            })
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();

        ElfForestVisiblity(matrix)
    }

    fn western_visibilty_matrix(&self) -> ElfForestVisiblity {
        let matrix: Vec<Vec<_>> = self
            .0
            .iter()
            .map(|inner_vec| {
                let mut tallest_tree = ElfTreeVisible {
                    visible: true,
                    height: inner_vec.first().unwrap().height,
                };

                inner_vec
                    .iter()
                    .enumerate()
                    .map(|(inner_idx, tree)| {
                        if inner_idx == 0 {
                            return tallest_tree;
                        }

                        let mut visible_tree = ElfTreeVisible {
                            height: tree.height,
                            visible: false,
                        };

                        if tree.height > tallest_tree.height {
                            visible_tree.visible = true;
                            tallest_tree = visible_tree
                        }

                        visible_tree
                    })
                    .collect()
            })
            .collect();

        ElfForestVisiblity(matrix)
    }

    fn eastern_visibilty_matrix(&self) -> ElfForestVisiblity {
        let matrix: Vec<Vec<_>> = self
            .0
            .iter()
            .map(|inner_vec| {
                let mut tallest_tree = ElfTreeVisible {
                    visible: true,
                    height: inner_vec.last().unwrap().height,
                };

                inner_vec
                    .iter()
                    .rev()
                    .enumerate()
                    .map(| (inner_idx, tree)| {
                        if dbg!(inner_idx) == 0 {
                            return dbg!(tallest_tree);
                        }

                        let mut visible_tree = ElfTreeVisible {
                            height: tree.height,
                            visible: false,
                        };

                        if tree.height > tallest_tree.height {
                            visible_tree.visible = true;
                            tallest_tree = visible_tree;
                        }

                        visible_tree
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect()
            })
            .collect();

        ElfForestVisiblity(matrix)
    }
}

fn full_visiblity_matrix(
    north: ElfForestVisiblity,
    south: ElfForestVisiblity,
    west: ElfForestVisiblity,
    east: ElfForestVisiblity,
) -> ElfForestVisiblity {
    let mut all_visible = north.0.clone();

    for (y_idx, vec) in south.0.iter().enumerate() {
        for (x_idx, &tree) in vec.iter().enumerate() {
            if tree.visible {
                all_visible[y_idx][x_idx] = tree;
            }
        }
    }

    for (y_idx, vec) in west.0.iter().enumerate() {
        for (x_idx, &tree) in vec.iter().enumerate() {
            if tree.visible {
                all_visible[y_idx][x_idx] = tree;
            }
        }
    }

    for (y_idx, vec) in east.0.iter().enumerate() {
        for (x_idx, &tree) in vec.iter().enumerate() {
            if tree.visible {
                all_visible[y_idx][x_idx] = tree;
            }
        }
    }

    ElfForestVisiblity(all_visible)
}

impl fmt::Display for ElfForestVisiblity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res = 
            self.0
            .iter()
            .map(|row| {
                let mut new_row: Vec<_> = row.iter().map(|tree| {
                    if tree.visible {
                        tree.height.to_string().green()
                    } else {
                        tree.height.to_string().red()
                    }
                }).collect();

                new_row.push("\n".clear());
                new_row
            })
            .collect::<Vec<_>>();

        for x in res.iter() {
            for y in x.iter() {
                f.write_fmt(format_args!("{}", y))?;
            }
        }
        Ok(())
    }
}

impl ElfForestVisiblity {
    fn count_visible_trees(&self) -> usize {
        let visible_trees: Vec<&ElfTreeVisible> = self
            .0
            .iter()
            .flatten()
            .filter(|&&tree| tree.visible)
            .collect();
        visible_trees.len()
    }
}

fn part1(forest: &ElfForest) -> Result<usize> {
    let visiblity_matrix = full_visiblity_matrix(
        forest.northern_visibilty_matrix(),
        forest.southern_visibilty_matrix(),
        forest.western_visibilty_matrix(),
        forest.eastern_visibilty_matrix(),
    );


    println!("{}", visiblity_matrix.to_string());

    Ok(visiblity_matrix.count_visible_trees())
}

fn main() -> Result<()> {
    let forest: ElfForest = INPUT_FILE.parse()?;

    println!(
        "Part 1 - Trees visible from outside grid: <{}>",
        part1(&forest)?
    );
    Ok(())
}
