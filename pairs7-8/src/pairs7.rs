use std::{fs::File, io::{BufReader, BufRead}};

use pairs7_8::get_range;

fn main() {
    let file = File::open("input.txt").expect("Failed to open");
    let file = BufReader::new(file);
    let matched = file.lines()
    .filter_map(|x| x.ok())
    .map(|x| {
        let mut x = x.split(',');
        let first = x.next().unwrap();
        let second = x.next().unwrap();
        if contains(first, second) {
            return 1;
        }
        0
    })
    .sum::<u32>();

    println!("{matched}")
}

fn contains(first: &str, second: &str) -> bool{
    let (first_lower, first_higher) = get_range(first);
    let (second_lower, second_higher) = get_range(second);
    if first_lower >= second_lower && first_higher <= second_higher {
        return true;
    }
    if second_lower >= first_lower && second_higher <= first_higher {
        return true;
    }

    false
}