use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use std::iter::Iterator;
use std::path::PathBuf;

use anyhow::{anyhow, bail};
use itertools::Itertools;
use lazy_static::lazy_static;

pub type Error = anyhow::Error;
pub type Result<T> = anyhow::Result<T>;

const INPUT_PATH: &str = "inputs/day7.txt";
const BIG_DIRECTORY_SIZE: u32 = 100000;

lazy_static! {
    static ref INPUT_FILE: String =
        fs::read_to_string(INPUT_PATH).expect("Day 7 - Inputs: Can't parse stacks");
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct ElfContentIndex {
    full_path: PathBuf,
}

#[derive(Debug)]
struct ElfContent {
    parent: Option<ElfContentIndex>,
    kind: ElfContentKind,
}

#[derive(Debug)]
enum ElfContentKind {
    File { size: u32 },
    Dir { children: Vec<ElfContentIndex> },
}

impl ElfContent {
    fn is_valid(&self) {
        match &self.kind {
            ElfContentKind::File { size: _ } => assert!(self.parent.is_some()),
            ElfContentKind::Dir { children: _ } => (),
        }
    }
}

struct ElfFileSystem {
    root: ElfContentIndex,
    items: HashMap<ElfContentIndex, ElfContent>,
}

#[derive(Debug)]
enum ElfTerminalBlock {
    CdCommandUp,
    CdCommandRoot,
    CdCommandDown {
        dir_name: String,
    },
    LsCommand {
        dir_listing: Vec<(ElfFileAttribute, String)>,
    },
}

#[derive(Debug)]
enum ElfFileAttribute {
    Dir,
    Size(u32),
}

impl ElfFileSystem {
    fn new() -> Self {
        Self {
            items: HashMap::new(),
            root: ElfContentIndex {
                full_path: PathBuf::from("invalid"),
            },
        }
    }

    fn get_size_of_node(&self, index: &ElfContentIndex) -> Result<u32> {
        let this_node = self
            .items
            .get(&index)
            .ok_or_else(|| anyhow!("did not get node"))?;
        match &this_node.kind {
            ElfContentKind::File { size } => Ok(*size),
            ElfContentKind::Dir { children } => {
                children.iter().map(|child| self.get_size_of_node(&child)).sum()
            }
        }
    }

    // fn get_total_size(&self) -> Result<u32> {
    //     self.get_size_of_node(&self.root)
    // }

    fn get_part1_size(&self) -> Result<u32> {
        self.items.iter().filter(|(_idx, content)| {
            if let ElfContentKind::Dir { children: _ } = content.kind {
                true
            } else {
                false
            }
        }).map(|(idx, _content)| self.get_size_of_node(idx)).map_ok(|size| {
            if size < BIG_DIRECTORY_SIZE {
                size
            } else {
                0
            }
        }).sum()
    }

    fn process_command(
        &mut self,
        pos: &ElfContentIndex,
        block: &ElfTerminalBlock,
    ) -> Result<ElfContentIndex> {
        match block {
            ElfTerminalBlock::CdCommandRoot => {
                let new_root_idx = ElfContentIndex {
                    full_path: PathBuf::from(r"efs-root"),
                };

                self.root = new_root_idx;
                self.items = HashMap::from([(
                    self.root.clone(),
                    ElfContent {
                        parent: None,
                        kind: ElfContentKind::Dir { children: vec![] },
                    },
                )]);
                Ok(self.root.clone())
            }
            ElfTerminalBlock::CdCommandUp => {
                let node = self.items.get(&pos).ok_or_else(|| anyhow!("No node"))?;
                match &node.parent {
                    None => bail!("Can't CD past root"),
                    Some(parent_pos) => Ok(parent_pos.clone()),
                }
            }
            ElfTerminalBlock::CdCommandDown { dir_name } => {
                self.items.get(&pos).ok_or_else(|| anyhow!("No node"))?;
                let new_pos = ElfContentIndex {
                    full_path: [
                        pos.full_path
                            .to_str()
                            .ok_or(anyhow!("path to string fail"))?,
                        dir_name.as_str(),
                    ]
                    .iter()
                    .collect(),
                };
                assert!(self.items.contains_key(&new_pos));
                Ok(new_pos)
            }
            ElfTerminalBlock::LsCommand { dir_listing } => {
                for (attr, name) in dir_listing.iter() {
                    let new_pos = ElfContentIndex {
                        full_path: [
                            pos.full_path
                                .to_str()
                                .ok_or(anyhow!("path to string fail"))?,
                            name.as_str(),
                        ]
                        .iter()
                        .collect(),
                    };

                    let parent_node = self.items.get_mut(&pos).ok_or_else(|| {
                        anyhow!("no parent!!!")
                    })?;
                    if let ElfContentKind::Dir { children } = &mut parent_node.kind {
                        children.push(new_pos.clone());
                    } else {
                        bail!("parent is a file not a dir!!!");
                    }

                    let kind = match attr {
                        ElfFileAttribute::Dir => ElfContentKind::Dir { children: vec![] },
                        ElfFileAttribute::Size(size) => ElfContentKind::File { size: *size },
                    };
                    let content = ElfContent {
                        parent: Some(pos.clone()),
                        kind,
                    };
                    content.is_valid();

                    self.items.insert(new_pos, content);
                }
                Ok(pos.clone())
            }
        }
    }
}

fn process_blocks(block: &str) -> Result<ElfTerminalBlock> {
    let f_block = block.trim().lines().next();
    match f_block {
        Some("ls") => {
            let dir_listing_res: Result<Vec<(ElfFileAttribute, String)>> = block
                .lines()
                .skip(1)
                .map(|line| {
                    line.split_once(" ")
                        .ok_or_else(|| anyhow!("Problem with line: {:?}", line))
                })
                .map(|split_res| {
                    let (attr, name) = split_res?;
                    let thing = match attr {
                        "dir" => (ElfFileAttribute::Dir, String::from(name)),
                        _ => (
                            ElfFileAttribute::Size(attr.parse::<u32>()?),
                            String::from(name),
                        ),
                    };
                    Ok(thing)
                })
                .collect();
            Ok(ElfTerminalBlock::LsCommand {
                dir_listing: dir_listing_res?,
            })
        }
        Some("cd /") => Ok(ElfTerminalBlock::CdCommandRoot),
        Some("cd ..") => Ok(ElfTerminalBlock::CdCommandUp),
        Some(cd_dir) => Ok(ElfTerminalBlock::CdCommandDown {
            dir_name: String::from(
                cd_dir
                    .split_once(" ")
                    .ok_or(anyhow!("Problem with line: {:?}", cd_dir))?
                    .1,
            ),
        }),
        None => bail!("Look at this shit: {:?}", block),
    }
}

fn part1(terminal_output: &str) -> Result<u32> {
    let mut efs = ElfFileSystem::new();
    let mut track_pos = efs.root.clone();

    let commands_iter = terminal_output
        .split("$")
        .filter(|&s| s != "")
        .map(&process_blocks);

    for command_res in commands_iter {
        match command_res {
            Err(err) => bail!("Ahhhhh, {}", err),
            Ok(block) => track_pos = efs.process_command(&track_pos, &block)?,
        }
    };

    Ok(efs.get_part1_size()?)
}

fn main() -> Result<()> {
    println!(
        "Part 1 - Total size of directories with size < {}: <{}>",
        BIG_DIRECTORY_SIZE,
        part1(&INPUT_FILE)?
    );
    Ok(())
}
