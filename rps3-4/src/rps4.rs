use rps3_4::read_and_fold;

fn main(){
    println!("{}", read_and_fold("input.txt", |score, opponent, you| score + calculate_score(opponent, you)));
}

fn calculate_score(opponent: &str, you: &str) -> u32{
    match (opponent, you) {
        ("A", "X") => 0 + 3,
        ("A", "Y") => 3 + 1,
        ("A", "Z") => 6 + 2,
        ("B", "X") => 0 + 1,
        ("B", "Y") => 3 + 2,
        ("B", "Z") => 6 + 3,
        ("C", "X") => 0 + 2,
        ("C", "Y") => 3 + 3,
        ("C", "Z") => 6 + 1,
        _ => panic!("Unkown combination"),
    }
}