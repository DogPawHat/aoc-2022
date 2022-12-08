use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::iter::Iterator;
use std::rc::Rc;

use anyhow::{anyhow, bail, Context};
use lazy_static::lazy_static;

pub type Error = anyhow::Error;
pub type Result<T> = anyhow::Result<T>;

const INPUT_PATH: &str = "inputs/day7.txt";
const BIG_DIRECTORY_SIZE: u32 = 100000;

lazy_static! {
    static ref INPUT_FILE: String =
        fs::read_to_string(INPUT_PATH).expect("Day 7 - Inputs: Can't parse stacks");
}

enum ElfTerminalBlock {
    CdCommandUp,
    CdCommandDown {
        dir_name: String,
    },
    LsCommand {
        dir_listing: Vec<(ElfFileAttribute, String)>,
    },
}

enum ElfFileAttribute {
    Dir,
    Size(u32),
}

#[derive(Debug)]
enum ElfParentDirectory {
    Root,
    NonRoot(usize),
}

#[derive(Debug)]
struct ElfFile {
    size: u32,
}

#[derive(Debug)]
struct ElfDirectory {
    parent: ElfParentDirectory,
    directories: HashMap<String, usize>,
    files: HashMap<String, usize>,
}

#[derive(Debug)]
struct ElfFileSystem {
    directories: Vec<ElfDirectory>,
    files: Vec<ElfFile>,
}

#[derive(Debug)]
struct ElfDirectorys(HashMap<String, Rc<RefCell<ElfDirectory>>>);

#[derive(Debug)]
struct ElfFiles(HashMap<String, ElfFile>);

fn process_blocks(block: &str) -> Result<ElfTerminalBlock> {
    match block.trim().lines().next() {
        Some("ls") => {
            let dir_listing_res: Result<Vec<(ElfFileAttribute, String)>> = block
                .lines()
                .skip(1)
                .map(|line| line.split_once(" ").ok_or(anyhow!("derp")))
                .map(|split_res| {
                    let (attr, name) = split_res.context("bleg")?;
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
        Some("cd /") => Ok(ElfTerminalBlock::CdCommandUp),
        Some(cd_dir) => Ok(ElfTerminalBlock::CdCommandDown {
            dir_name: String::from(cd_dir.split_once(" ").ok_or(anyhow!("derp"))?.1),
        }),
        None => bail!("derp"),
    }
}

impl ElfFileSystem {
    fn new() -> Self {
        Self {
            directories: vec![],
            files: vec![],
        }
    }

    // fn get_root_dir(&self) -> &ElfDirectory {
    //     &self.directories[0]
    // }

    // fn get_root_dir_mut(&mut self) -> &mut ElfDirectory {
    //     &mut self.directories[0]
    // }

    // fn get_dir(&self, parent: &mut ElfDirectory, name: &str) -> &ElfDirectory {
    //     let idx = parent.directories.get(name).ok_or(anyhow!("pain"))?;

    //     &self.directories[*idx]
    // }

    // fn get_dir_mut(&mut self, parent: &mut ElfDirectory, name: &str) -> &mut ElfDirectory {
    //     let idx = parent.directories.get(name).ok_or(anyhow!("pain"))?;

    //     &mut self.directories[*idx]
    // }

    // fn get_file(&self, parent: &mut ElfDirectory, name: &str) -> &ElfFile {
    //     let idx = parent.files.get(name).ok_or(anyhow!("pain"))?;

    //     &self.files[*idx]
    // }

    // fn get_file_mut(&mut self, parent: &mut ElfDirectory, name: &str) -> &mut ElfFile {
    //     let idx = parent.directories.get(name).ok_or(anyhow!("pain"))?;

    //     &mut self.files[*idx]
    // }

    fn get_total_size(&self) -> u32 {
        self.files.iter().map(|f| f.size).sum()
    }

    fn process_command(
        &mut self,
        cursor_dir_idx: &mut usize,
        block: &ElfTerminalBlock,
    ) -> Result<()> {
        match block {
            ElfTerminalBlock::CdCommandUp => {
                let cursor_dir = self
                    .directories
                    .get(*cursor_dir_idx)
                    .ok_or(anyhow!("aldjfoa"))?;

                match cursor_dir.parent {
                    ElfParentDirectory::Root => bail!("OH DEAR"),
                    ElfParentDirectory::NonRoot(idx) => {
                        *cursor_dir_idx = idx;
                    }
                }
            }
            ElfTerminalBlock::CdCommandDown { dir_name } => {
                let cursor_dir = self
                    .directories
                    .get(*cursor_dir_idx)
                    .ok_or(anyhow!("aldjfoa"))?;

                *cursor_dir_idx = *cursor_dir
                    .directories
                    .get(dir_name.as_str())
                    .ok_or(anyhow!("Did not get index for {}", &dir_name))?;
            }
            ElfTerminalBlock::LsCommand { dir_listing } => {
                for (attr, name) in dir_listing.iter() {
                    match attr {
                        ElfFileAttribute::Dir => {
                            self.directories.push(ElfDirectory {
                                parent: ElfParentDirectory::NonRoot(*cursor_dir_idx),
                                directories: HashMap::new(),
                                files: HashMap::new(),
                            });

                            let len = self.directories.len();
                            let cursor_dir = self
                                .directories
                                .get_mut(*cursor_dir_idx)
                                .ok_or(anyhow!("aldjfoa"))?;

                            cursor_dir.directories.insert(name.clone(), len - 1);
                        }
                        ElfFileAttribute::Size(size) => {
                            self.files.push(ElfFile { size: *size });

                            let len = self.directories.len();
                            let cursor_dir = self
                                .directories
                                .get_mut(*cursor_dir_idx)
                                .ok_or(anyhow!("aldjfoa"))?;

                            cursor_dir.files.insert(name.clone(), len - 1);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

fn part1(terminal_output: &str) -> Result<u32> {
    let mut efs = ElfFileSystem::new();
    let commands_iter = terminal_output.split("$").map(&process_blocks);

    for command_res in commands_iter {
        let mut cursor_dir_idx: usize = 0;
        match command_res {
            Err(err) => bail!("Ahhhhh, {}", err),
            Ok(block) => efs.process_command(&mut cursor_dir_idx, &block)?,
        }
    }

    Ok(efs.get_total_size())
}

fn main() -> Result<()> {
    println!(
        "Part 1 - Total size of directories with size > {}: <{}>",
        BIG_DIRECTORY_SIZE,
        part1(&INPUT_FILE)?
    );
    Ok(())
}
