use std::{fs::File, io::{BufReader, BufRead}};

pub fn read_and_fold<F>(path: &str, fold: F) -> u32
where
    F: Fn(u32, &str, &str) -> u32,
{
    let file = File::open(path).expect("Failed to open");
    let file = BufReader::new(file);
    file.lines()
        .filter_map(|x| x.ok())
        .map(|x| {
            let mut iter = x.split_whitespace().map(|y| y.to_owned());
            (iter.next(), iter.next())
        })
        .filter_map(|x| {
            if let (Some(first), Some(second)) = x {
                Some((first, second))
            } else {
                None
            }
        })
        .fold(0u32, |init, (opponent, you)| fold(init, &opponent, &you))
}
