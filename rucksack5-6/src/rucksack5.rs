use rucksack5_6::to_priority;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").expect("Failed to open file");
    let lines = BufReader::new(file)
        .lines()
        .filter_map(|x| x.ok())
        .map(|x| {
            let mid = x.len() / 2;
            let first = x[..mid].chars().collect::<HashSet<_>>();
            let second = x[mid..].chars().collect::<HashSet<_>>();
            (first, second)
        })
        .map(|(first, second)| {
            let union = first.intersection(&second);
            union.map(|x| to_priority(x)).collect::<Vec<u32>>()
        })
        .flatten()
        .sum::<u32>();

    println!("{lines}")
}
