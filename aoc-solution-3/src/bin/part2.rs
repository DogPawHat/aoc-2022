use itertools::Itertools;
use std::fs;

fn char_to_priority(item: char) -> u32 {
    match item {
        'a'..='z' => item as u32 - 'a' as u32 + 1,
        'A'..='Z' => item as u32 - 'A' as u32 + 27,
        _ => panic!("THE END OF DAYS IS NIGH"),
    }
}

fn get_rucksack_priority((elf_1, elf_2, elf_3): (&str, &str, &str)) -> u32 {
    let priority_check_fn = |val: char| {
        if elf_1.contains(val) && elf_2.contains(val) && elf_3.contains(val) {
            char_to_priority(val)
        } else {
            0
        }
    };
    let lower: u32 = ('a'..='z').map(priority_check_fn).sum();
    let upper: u32 = ('A'..='Z').map(priority_check_fn).sum();
    lower + upper
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("borked file");

    let score: u32 = input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|mut chunk| {
            (
                chunk.next().unwrap(),
                chunk.next().unwrap(),
                chunk.next().unwrap(),
            )
        })
        .map(get_rucksack_priority)
        .sum();

    println!("Hello, world! Your score is: <{}>", score);
}
