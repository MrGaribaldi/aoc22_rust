use std::{str::Lines};

#[derive(Default, Debug)]
struct Supplies{
    stacks: Vec<Stack>
}

impl Supplies{
    fn new(input:&mut  Lines) -> Option<Supplies> {
        let mut s: Supplies = Supplies::default();
        let mut first: bool = true;
        for l in input {
            //completed parsing input, exit loop and then return the Supplies Stack.
            if l.trim().is_empty() { 
                for i in 0..s.stacks.len() {
                    s.stacks[i].reverse();
                }
                return Some(s)
            }else{
            }
            if first {
                //Take length of line, add 1, since last entry will stop directly after ']' and then divide by 4, since each column has 4 spaces/characters. 
                let num_columns = (l.len()+1) / 4;
                s.set_stacks(num_columns);
                first = false;

            }
            
            let line = l;
            for i in 0..(line.len()+1)/4 {
                let index = (i*4)+1;
                let possible_char = line.chars().nth(index).unwrap();
                if possible_char == ' ' { continue; }
                else if possible_char == '1' {break;} //we don't want to add the column indexing
                s.stacks[i].add(possible_char);

            }

        }
        
        None
    }

    /* Simple helper function that will increase number of stacks until it matches @num.
     */ 
    fn set_stacks(&mut self, num:usize){
        while self.stacks.len() < num  {
        self.stacks.push(Stack::default());
        }
    }

    fn move_crate(&mut self, num_items:usize, source: usize, dest: usize) {
        let src = source-1;
        let dst = dest-1;
        for _ in 0..num_items {
            let c = self.stacks[src].pop(); 
            if c.is_none() { return; } 
            self.stacks[dst].add(c.unwrap());
        }
    }

    fn move_crate9001(&mut self, num_items:usize, source: usize, dest: usize) {
        let src = source-1;
        let dst = dest-1;
        let mut v_vec:Vec<char> = Vec::<char>::new();
        for _ in 0..num_items {
            
            let c = self.stacks[src].pop(); 
            if c.is_none() { break; } //stack is empty, no need to keep looping
            v_vec.push(c.unwrap());
            
        }
        for c in v_vec.into_iter().rev() {
            self.stacks[dst].add(c);
        }
    }
    fn get_top_crates(&mut self) -> String {
        let mut ret_val = String::default();
        for i in 0..self.stacks.len() {
            let c = self.stacks[i].pop();
            ret_val.push(c.unwrap_or_default());
        }
        ret_val
    }
}
#[derive(Default, Debug)]
struct Stack{
    crates: Vec<char>
}
impl Stack{
    fn add(&mut self, value: char) {
        self.crates.push(value);
    }
    fn pop(&mut self ) -> Option<char> {
        self.crates.pop()
    }
    fn reverse(&mut self){
        self.crates.reverse();
    }
}



pub fn part_one(input: &str) -> Option<String> {
    let mut lines = input.lines();
    let mut supplies = Supplies::new(&mut lines)?;
    for l in lines {
        let mut moves = l.split_whitespace();
        moves.next();
        let num_items:usize = moves.next().unwrap().parse().unwrap();
        moves.next();
        let source:usize =  moves.next().unwrap().parse().unwrap();
        moves.next();
        let dest:usize =  moves.next().unwrap().parse().unwrap();

        supplies.move_crate(num_items, source, dest);

    }

    Some(supplies.get_top_crates())
}

pub fn part_two(input: &str) -> Option<String> {
    let mut lines = input.lines();
    let mut supplies = Supplies::new(&mut lines)?;
    for l in lines {
        let mut moves = l.split_whitespace();
        moves.next();
        let num_items:usize = moves.next().unwrap().parse().unwrap();
        moves.next();
        let source:usize =  moves.next().unwrap().parse().unwrap();
        moves.next();
        let dest:usize =  moves.next().unwrap().parse().unwrap();

        supplies.move_crate9001(num_items, source, dest);

    }

    Some(supplies.get_top_crates())
    
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_owned()));
    }
}
