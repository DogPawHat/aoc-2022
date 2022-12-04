use std::collections::HashSet;
use std::fs;
use std::ops::RangeInclusive;

struct ElfPairs(RangeInclusive<i32>, RangeInclusive<i32>);

impl ElfPairs {
    fn is_fully_overlapped(&self) -> bool {
        let lesser_hash_a: HashSet<i32> = self.0.clone().collect();
        let lesser_hash_b: HashSet<i32> = self.1.clone().collect();

        !lesser_hash_a.is_disjoint(&lesser_hash_b) || !lesser_hash_b.is_disjoint(&lesser_hash_a)
    }
}

fn parse_elf_job_range(job_range_str: &str) -> RangeInclusive<i32> {
    let job_range_collected: Vec<i32> = job_range_str.split("-")
        .map(|num| num.parse())
        .collect::<Result<_, _>>()
        .unwrap();
    
    job_range_collected[0]..=job_range_collected[1]
}

fn parse_elf_job_pair(line: &str) -> ElfPairs {
    let mut range_iter = line.split(",")
        .map(parse_elf_job_range)
        .take(2);
    ElfPairs(range_iter.next().unwrap(), range_iter.next().unwrap())
}

fn main() {
    let file = fs::read_to_string("aoc-solution-4/input.txt").expect("could not read input");

    let overlapped_jobs: i32 = file
        .lines()
        .map(parse_elf_job_pair)
        .map(|pair| pair.is_fully_overlapped() )
        .map(|overlap| if overlap { 1 } else { 0 } )
        .sum();

    println!("Hello, world! The score is <{}>", overlapped_jobs);
}
