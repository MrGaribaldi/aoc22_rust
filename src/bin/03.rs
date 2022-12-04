use std::collections::{BTreeSet};

use itertools::Itertools;


fn get_priority(input: u8)-> u32 {
    if input > 91 {
        input as u32- 96
    }else{
        input as u32 - 38
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut summed_priorities = 0u32;    
    for l in input.lines() {
        let (first, second) = l.split_at(l.len()/2);
        for c in first.chars() {
            if second.contains(c) {
                summed_priorities   += get_priority(c as u8);
                break;
            }
        }
    }
    Some(summed_priorities)
}

fn get_group_badge(group_data: Vec<&str>) -> char {
    let mut rucksacks = Vec::<BTreeSet::<char>>::new();
    //this can be done much better, but at least this will work..
    rucksacks.reserve(3);   
    rucksacks.push(BTreeSet::<char>::new());
    rucksacks.push(BTreeSet::<char>::new());
    rucksacks.push(BTreeSet::<char>::new());
    for (i, l) in group_data.into_iter().enumerate() {
        for c in l.chars() {
            rucksacks[i].insert(c);
        }
    }
    let first_intersection:BTreeSet<char> = rucksacks[0].intersection(&rucksacks[1]).cloned().collect();
    let second_intersection:Vec<char> = rucksacks[2].intersection(&first_intersection).cloned().collect();

    second_intersection[0]
    
}

pub fn part_two(input: &str) -> Option<u32> {
    //let mut group_vec = Vec::new();
    let mut sum_pri:u32 = 0;
    for l in input.lines().chunks(3).into_iter() {
         let badge = get_group_badge(l.collect_vec());
         sum_pri += get_priority(badge as u8);
    }
    Some(sum_pri)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
