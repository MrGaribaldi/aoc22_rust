
#[derive(Debug, Default)]
struct RingBuffer{
    data: [char; 14],
    data_index: usize
}

impl RingBuffer{
    fn add(&mut self, input:char, modulo:usize){
        self.data[self.data_index]= input;
        self.data_index = (self.data_index+1)%modulo;
    }
    fn scan(&self, modulo:usize) -> bool {
        for i in 0usize..modulo {
            for j in (i+1)..modulo {
                if self.data[i] == self.data[j] { return false }
            }
        }
        true
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut rb = RingBuffer::default();
    let char_input = input.chars();
    for (i, c) in char_input.enumerate(){
        rb.add(c, 4);
        if i > 2 {
            let found = rb.scan(4);
            if found { return Some((i+1) as u32) }
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rb = RingBuffer::default();
    let char_input = input.chars();
    for (i, c) in char_input.enumerate(){
        rb.add(c, 14);
        if i > 2 {
            let found = rb.scan(14);
            if found { return Some((i+1) as u32) }
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
