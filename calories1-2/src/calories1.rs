use calories1_2::calculate_elfs_calories;

fn main() {
    let elfs = calculate_elfs_calories("input.txt");

    println!("{:?}", elfs.iter().max_by(|x, y| x.cmp(y)))
}
