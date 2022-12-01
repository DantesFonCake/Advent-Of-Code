fn main() {
    let mut elfs = calories1_2::calculate_elfs_calories("input.txt");

    elfs.sort_by(|x, y| y.cmp(x));

    println!("{:?}", elfs.iter().take(3).sum::<i32>())
}