use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;

use anyhow::{bail, Result};
use lazy_static::lazy_static;

const INPUT_PATH: &str = "inputs/day7.txt";
const BIG_DIRECTORY_SIZE: u32 = 100000;
const TOTAL_DISK_SIZE: u32 = 70000000;
const UNUSED_DISK_SIZE_TARGET: u32 = 30000000;

lazy_static! {
    static ref INPUT_FILE: String =
        fs::read_to_string(INPUT_PATH).expect("Day 7 - Inputs: Can't parse stacks");
}

trait ElfSized {
    fn full_size(&self) -> u32;
}

#[derive(Debug)]
struct ElfDirectorys(HashMap<String, Rc<RefCell<ElfDirectory>>>);

#[derive(Debug)]
struct ElfFiles(HashMap<String, ElfFile>);

#[derive(Debug)]
struct ElfFile {
    size: u32,
}

#[derive(Debug)]
enum ElfParentDirectory {
    Root,
    NonRoot(Rc<RefCell<ElfDirectory>>),
}

#[derive(Debug)]
struct ElfDirectory {
    parent: ElfParentDirectory,
    directories: ElfDirectorys,
    files: ElfFiles,
}

enum ElfTerminalLine {
    CdCommand { dir: String },
    LsCommand,
    FileListing { size: u32, file: String },
    DirectoryListing { dir: String },
}

impl ElfSized for ElfDirectory {
    fn full_size(&self) -> u32 {
        self.directories
            .0
            .iter()
            .map(|c| c.1.try_borrow())
            .filter_map(|f| f.ok())
            .map(|g| g.full_size())
            .sum::<u32>()
            + self.files.0.iter().map(|c| c.1.full_size()).sum::<u32>()
    }
}

impl ElfSized for ElfFile {
    fn full_size(&self) -> u32 {
        self.size
    }
}

impl ElfDirectory {
    fn special_size(&self) -> u32 {
        let full = self.full_size();
        let this_special = if full < BIG_DIRECTORY_SIZE {
            full
        } else {
            0
        };

        self.directories
            .0
            .iter()
            .map(|c| c.1.try_borrow())
            .filter_map(|f| f.ok())
            .map(|g| g.special_size())
            .sum::<u32>() + this_special
    }

    fn get_size_sort(&self) -> Vec<(&String, &Rc<RefCell<ElfDirectory>>)> {
        let mut dir_vec: Vec<_> = self.directories.0.iter().collect();

        dir_vec.sort_by(|a, b| {
            let a_size = a.1.try_borrow().unwrap().full_size();
            let b_size = b.1.try_borrow().unwrap().full_size();

            a_size.cmp(&b_size)
        });

        dir_vec
    }
}


fn process_terminal_line(
    dir_rc_cell: Rc<RefCell<ElfDirectory>>,
    command: &ElfTerminalLine,
) -> Result<Rc<RefCell<ElfDirectory>>> {
    match command {
        ElfTerminalLine::DirectoryListing { dir } => {
            dir_rc_cell.try_borrow_mut()?.directories.0.insert(
                dir.clone(),
                Rc::new(RefCell::new(ElfDirectory {
                    parent: ElfParentDirectory::NonRoot(Rc::clone(&dir_rc_cell)),
                    files: ElfFiles(HashMap::new()),
                    directories: ElfDirectorys(HashMap::new()),
                })),
            );
            Ok(Rc::clone(&dir_rc_cell))
        }
        ElfTerminalLine::FileListing { size, file } => {
            dir_rc_cell
                .try_borrow_mut()?
                .files
                .0
                .insert(file.clone(), ElfFile { size: *size });
            Ok(Rc::clone(&dir_rc_cell))
        }
        ElfTerminalLine::CdCommand { dir } => match dir.as_str() {
            "/" => Ok(Rc::clone(&dir_rc_cell)),
            ".." => {
                let parent = &(*dir_rc_cell.try_borrow()?).parent;

                match parent {
                    ElfParentDirectory::Root => bail!("Can not cd past root!"),
                    ElfParentDirectory::NonRoot(parent_dir) => Ok(Rc::clone(&parent_dir)),
                }
            }
            _ => {
                if let Some(child_dir_cell) = dir_rc_cell.borrow_mut().directories.0.get_mut(dir) {
                    Ok(Rc::clone(&child_dir_cell))
                } else {
                    bail!("Processing error")
                }
            }
        },
        ElfTerminalLine::LsCommand => Ok(Rc::clone(&dir_rc_cell)),
    }
}

fn line_mapper(line: &str) -> Result<ElfTerminalLine> {
    let line_space_split: Vec<_> = line.split(" ").collect();

    match (line_space_split[0], line_space_split[1]) {
        ("$", "cd") => Ok(ElfTerminalLine::CdCommand {
            dir: String::from(line_space_split[2]),
        }),
        ("$", "ls") => Ok(ElfTerminalLine::LsCommand),
        ("dir", _) => Ok(ElfTerminalLine::DirectoryListing {
            dir: String::from(line_space_split[1]),
        }),
        (_, _) => match line_space_split[0].parse::<u32>() {
            Ok(size) => Ok(ElfTerminalLine::FileListing {
                size,
                file: String::from(line_space_split[1]),
            }),
            Err(error) => Err(anyhow::Error::from(error)),
        },
    }
}

fn construct_file_system(terminal_output: &str) -> Result<Rc<RefCell<ElfDirectory>>> {
    let command_iter = terminal_output.lines().map(line_mapper);

    let root_dir_rc_cell = Rc::new(RefCell::new(ElfDirectory {
        parent: ElfParentDirectory::Root,
        directories: ElfDirectorys(HashMap::new()),
        files: ElfFiles(HashMap::new()),
    }));

    {
        let mut cursor_dir_root_rc = Rc::clone(&root_dir_rc_cell);

        for line_res in command_iter {
            match line_res {
                Ok(line) => {
                    cursor_dir_root_rc = process_terminal_line(cursor_dir_root_rc, &line)?;
                }
                _ => bail!("Ahhhhhhhhh"),
            }
        }
    }

    Ok(root_dir_rc_cell)
}

fn part1(terminal_output: &str) -> Result<u32> {
    let root_dir = construct_file_system(terminal_output)?;

    let size = root_dir.try_borrow()?.special_size();
    Ok(size)
}

// fn part2(terminal_output: &str) -> Result<u32> {
//     let root_dir = construct_file_system(terminal_output)?;

//     let full_root_size = root_dir.try_borrow()?.full_size();
//     let free_size = TOTAL_DISK_SIZE - full_root_size;
//     let need_to_delete = UNUSED_DISK_SIZE_TARGET - free_size;
//     dbg!(full_root_size);
//     dbg!(free_size);
//     dbg!(need_to_delete);

//     let mut smallest_dir = Rc::clone(&root_dir);
//     loop {
//         let smol_rc = Rc::clone(&smallest_dir);
//         let smol_ref = smol_rc.try_borrow()?;
//         let res = smol_ref.get_smallest_dir()?;
//         if let Some((_smol_key, smol_dir)) = res {
//             let smol_size = smol_dir.try_borrow()?.full_size();
//             if smol_size < need_to_delete {
//                 break;
//             }
//             smallest_dir = smol_dir;
//         }
//     }

//     let size = smallest_dir.try_borrow()?.full_size();
//     Ok(size)
// }

fn main() -> Result<()> {
    println!(
        "Part 1 - Total size of directories with size > {}: <{}>",
        BIG_DIRECTORY_SIZE,
        part1(&INPUT_FILE)?
    );
    // println!(
    //     "Part 2: <{}>",
    //     part2(&INPUT_FILE)?
    // );
    Ok(())
}
