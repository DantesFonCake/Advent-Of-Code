use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};
use rucksack5_6::to_priority;

fn main() {
    let file = File::open("input.txt").expect("Failed to open file");
    let mut lines = BufReader::new(file)
        .lines()
        .filter_map(|x| x.ok());
    let mut sum = 0;
    while let Some(first) = lines.next(){
        let first = first.chars().collect::<HashSet<_>>();
        let second = lines.next().unwrap().chars().collect::<HashSet<_>>();
        let third = lines.next().unwrap().chars().collect::<HashSet<_>>();

        let first_and_second = first.intersection(&second).cloned().collect::<HashSet<_>>();
        let all_three = first_and_second.intersection(&third);
        
        sum += all_three.map(|x| to_priority(x)).sum::<u32>();
    }

    println!("{sum}")
}