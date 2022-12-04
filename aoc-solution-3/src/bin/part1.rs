use std::fs;

fn char_to_priority(item: char) -> u32 {
    match item {
        'a'..='z' => item as u32 - 'a' as u32 + 1,
        'A'..='Z' => item as u32 - 'A' as u32 + 27,
        _ => panic!("THE END OF DAYS IS NIGH")
    }
}

fn get_rucksack_priority(val: &str) -> u32 {
    let (sack_a, sack_b) = val.split_at(val.len() / 2);

    let priority_check_fn = |val: char| {
        if sack_a.contains(val) && sack_b.contains(val) {
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
        .map(&get_rucksack_priority)
        .sum();

    println!("Hello, world! Your score is: <{}>", score);
}
