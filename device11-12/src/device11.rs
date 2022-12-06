use std::{io::BufReader, fs::File};

use device11_12::find_unique_start;

fn main() {
    let file = File::open("input.txt").unwrap();
    let file = BufReader::new(file);
    let answer = find_unique_start(file, 4).expect("Did not found start");

    println!("{answer}");
}


