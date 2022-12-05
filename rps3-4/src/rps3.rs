use rps3_4::read_and_fold;

fn main() {
    println!("{}", read_and_fold("input.txt", |score, opponent, you| score + calculate_score(opponent, you)));
}


fn calculate_score(opponent: &str, you: &str) -> u32 {
    match (opponent, you) {
        ("A", "X") => 3 + 1,
        ("A", "Y") => 6 + 2,
        ("A", "Z") => 0 + 3,
        ("B", "X") => 0 + 1,
        ("B", "Y") => 3 + 2,
        ("B", "Z") => 6 + 3,
        ("C", "X") => 6 + 1,
        ("C", "Y") => 0 + 2,
        ("C", "Z") => 3 + 3,
        _ => panic!("Unkown combination"),
    }
}
