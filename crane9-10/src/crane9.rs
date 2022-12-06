use std::{
    fs::File,
    io::{BufRead, BufReader}
};

use crane9_10::CrateStacks;

fn main() {
    let file = File::open("input.txt").expect("Failed to open");
    let file = BufReader::new(file);
    let mut lines = file.lines().filter_map(|x| x.ok());
    let mut stacks = CrateStacks::collect(&mut lines);

    lines.for_each(|x| execute(&mut stacks, &x));
    println!("{}", stacks.get_tops())
}


fn execute(stacks: &mut CrateStacks, cmd: &str){
    let mut parts = cmd.split(' ').skip(1).step_by(2);
    let (Some(count), Some(from), Some(to)) = (
        parts.next().and_then(|x| x.parse::<usize>().ok()), 
        parts.next().and_then(|x| x.parse::<usize>().ok()), 
        parts.next().and_then(|x| x.parse::<usize>().ok())
    ) else {
        return;
    }; 

    let stacks  = &mut stacks.stacks;
    for _ in 0..count {
        let container =stacks[from -1].pop_back().unwrap();
        stacks[to-1].push_back(container);
    }
}

