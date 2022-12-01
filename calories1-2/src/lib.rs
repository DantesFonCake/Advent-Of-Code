use std::{io::{BufReader, BufRead}, fs::File};

pub fn calculate_elfs_calories(path: &str) -> Vec<i32> {
    let mut elfs = vec![0];
    let file = BufReader::new(File::open(path).expect("Could not find file"));
    for line in file.lines().filter_map(|x| x.ok()){
        match line.as_str() {
            "\n" | "" => elfs.push(0),
            num => *elfs.last_mut().expect("Failed to get last element") += num.parse::<i32>().expect("Failed to parse number")
        }
    }

    elfs
}