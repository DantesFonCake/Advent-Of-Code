use std::{collections::{HashSet, VecDeque}, io::Read};

pub fn check_sequence(sequence: &VecDeque<char>) -> bool {
    let mut uniq = HashSet::with_capacity(sequence.len());

    sequence.iter().all(move |x| uniq.insert(*x))
}

pub fn find_unique_start(mut file: impl Read, start_len: usize) -> Option<usize> {
    let mut sequence = VecDeque::with_capacity(start_len);
    let mut buf = vec![0; 32];
    let mut position =0;
    while file.read(&mut buf).expect("read_until failed") != 0 {
        let s = String::from_utf8(buf).expect("from_utf8 failed");
        for c in s.chars() {
            sequence.push_back(c);
            if sequence.len() == start_len {
                if check_sequence(&sequence) {
                    return Some(position + 1);
                }

                sequence.pop_front();
            }
        
            position += 1;
        }

        buf = s.into_bytes();
    }

    None
}