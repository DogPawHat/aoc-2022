use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::IntErrorKind;

const INPUT_PATH_STR: &str = "aoc-solution-1/input.txt";

struct Elf {
    calories: i32,
}

struct ElfCalorieCounter {
    top_elfs: Vec<Elf>,
    elf_calorie_stack: Vec<i32>,
}

impl ElfCalorieCounter {
    fn new() -> Self {
        ElfCalorieCounter {
            top_elfs: vec![],
            elf_calorie_stack: vec![],
        }
    }

    fn push_to_calorie_stack(&mut self, val: i32) -> () {
        self.elf_calorie_stack.push(val);
    }

    fn compare_and_add_elf(&mut self, new_elf: Elf) -> () {
        if(self.top_elfs.len() == 0) {
            self.top_elfs.push(new_elf);
            return;
        }

        let mut max_count = 0;
        let mut top_elf_iter = self.top_elfs.iter().rev();
        while max_count < 3 {
            if let Some(elf_compare) = top_elf_iter.next() {
                if elf_compare.calories < new_elf.calories {
                    self.top_elfs
                        .insert(self.top_elfs.len() - max_count, new_elf);
                    break;
                }
            }
            max_count = max_count + 1;
        }
    }

    fn sum_of_top_three(&self) -> i32 {
        let mut max_count = 0;
        let mut top_elf_iter = self.top_elfs.iter().rev();
        let mut sum = 0;
        while max_count < 3 {
            if let Some(elf) = top_elf_iter.next() {
                sum = sum + elf.calories;
            }
            max_count = max_count + 1;
        }
        sum
    }

    fn sum_and_add_elf(&mut self) -> () {
        let count: i32 = self.elf_calorie_stack.iter().sum();
        let new_elf = Elf { calories: count };
        self.elf_calorie_stack = vec![];
        self.compare_and_add_elf(new_elf);
    }
}

fn get_elf_calories() -> ElfCalorieCounter {
    let file = File::open(INPUT_PATH_STR).expect("Could not open input file");

    let reader = BufReader::new(file);
    let lines = reader.lines().filter_map(|line| line.ok());

    let mut counter = ElfCalorieCounter::new();

    for line in lines {
        match line.parse::<i32>() {
            Err(e) => {
                if e.kind() == &IntErrorKind::Empty {
                    counter.sum_and_add_elf();
                } else {
                    panic!("{}", e)
                }
            }
            Ok(calories) => counter.push_to_calorie_stack(calories),
        }
    }

    counter
}

fn main() {
    let counter = get_elf_calories();
    println!(
        "Hello, world! The elf with the largest number of calories has: {}",
        counter.top_elfs.last().unwrap_or(&Elf { calories: 0}).calories
    );
    println!(
        "The top three elfs have: {}",
        counter.sum_of_top_three()
    );
}
