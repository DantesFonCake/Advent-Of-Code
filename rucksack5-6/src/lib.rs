pub fn to_priority(x: &char) -> u32 {
    if x.is_ascii_lowercase() {
        return *x as u32 + 1 - 'a' as u32;
    } else if x.is_ascii_uppercase() {
        return *x as u32 + 1 - 'A' as u32 + 26;
    }

    panic!("Non ascii char provided")
}
