use std::collections::VecDeque;

#[derive(Debug)]
pub struct CrateStacks {
    pub stacks: Vec<VecDeque<String>>,
}

impl CrateStacks {
    pub fn get_tops(&self) -> String{
        self.stacks.iter()
        .filter_map(|x| x.get(x.len()-1))
        .fold(String::new(), |mut init, x| {
            init.push_str(x);
            return init;
            }
        )   
    }

    pub fn collect(lines: &mut impl Iterator<Item = String>) -> CrateStacks
    {
        let mut stacks = Vec::new();
        for line in lines {
            if line.is_empty() {
                break;
            }

            let mut pointer = 0;
            let mut chars = line.char_indices();
            println!("{line}");
            while let Some((i, c)) = chars.next() {
                if c == ' '{
                    chars.next();
                    chars.next();
                    chars.next();
                } else if c=='['{
                    let Some((start, _)) = chars.next() else {
                        println!("failed to find container start");
                        break;
                    };
                    let Some((end,_)) = chars.find(|(_, c)| *c == ']') else {
                        println!("failed to find ]");
                        break; 
                    };
                    
                    let container = &line[start..end];
                    for _ in stacks.len()..=pointer {
                        stacks.push(VecDeque::new());
                    }
                    
                    
                    stacks[pointer].push_front(container.to_owned());
                    chars.next();
                }
                pointer+=1;
            }
        }

        CrateStacks { stacks }
    }
}
