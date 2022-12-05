pub fn get_range(s: &str) -> (u32, u32){
    let mut s = s.split('-');
    (s.next().unwrap().parse().unwrap(), s.next().unwrap().parse().unwrap())
}