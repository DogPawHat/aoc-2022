use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::IntErrorKind;

const INPUT_PATH_STR: &str = "input.txt";

struct CalorieCounter {
    largest_calories: i32,
    calorie_stack: Vec<i32>,
}

impl CalorieCounter {
    fn new() -> Self {
        CalorieCounter {
            largest_calories: 0,
            calorie_stack: vec![],
        }
    }

    fn push_to_stack(&mut self, val: i32) -> () {
        self.calorie_stack.push(val);
    }

    fn sum_and_compare_stack(&mut self) -> () {
        let count: i32 = self.calorie_stack.iter().sum();
        if count > self.largest_calories {
            self.largest_calories = count;
        }
        self.calorie_stack = vec![];
    }
}

fn get_elf_calories() -> i32 {
    let file = File::open(INPUT_PATH_STR).expect("Could not open input file");

    let reader = BufReader::new(file);
    let lines = reader.lines().filter_map(|line| line.ok());

    let mut counter = CalorieCounter::new();

    for line in lines {
        match line.parse::<i32>() {
            Err(e) => {
                if e.kind() == &IntErrorKind::Empty {
                    counter.sum_and_compare_stack();
                } else {
                    panic!("{}", e)
                }
            }
            Ok(calories) => counter.push_to_stack(calories),
        }
    }

    counter.largest_calories
}

fn main() {
    let elf_calories = get_elf_calories();
    println!(
        "Hello, world! The elf with the largest number of calories has: {}",
        elf_calories
    );
}
