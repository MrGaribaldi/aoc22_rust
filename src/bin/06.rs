
#[derive(Debug)]
struct RingBuffer<const N:usize>{
    data: [char; N],
    data_index: usize
}

impl<const N:usize>  RingBuffer<N>{
    fn default() -> RingBuffer<N> {
        let r: RingBuffer<N> = RingBuffer { data: [' '; N], data_index: 0 };
        r
    }
    fn add(&mut self, input:char){
        self.data[self.data_index]= input;
        self.data_index = (self.data_index+1)%N;
    }
    fn scan(&self) -> bool {
        for i in 0usize..N {
            for j in (i+1)..N {
                if self.data[i] == self.data[j] { return false }
            }
        }
        true
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut rb = RingBuffer::<4>::default();
    let char_input = input.chars();
    for (i, c) in char_input.enumerate(){
        rb.add(c);
        if i > 2 {
            let found = rb.scan();
            if found { return Some((i+1) as u32) }
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rb = RingBuffer::<14>::default();
    let char_input = input.chars();
    for (i, c) in char_input.enumerate(){
        rb.add(c);
        if i > 2 {
            let found = rb.scan();
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
