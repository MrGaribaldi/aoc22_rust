

pub fn part_one(input: &str) -> Option<u32> {
    let mut num_overlapping_pairs = 0u32;
    for l in input.lines() {
        let mut pairs = l.split(',');

        let first_pair = pairs.next()?;
        let mut first_pair = first_pair.split('-').collect::<Vec<&str>>().into_iter();
        let a1n = first_pair.next()?.parse::<u32>().unwrap();
        let a1_2n = first_pair.next()?.parse::<u32>().unwrap();
        let mut second_pair = pairs.next()?.split('-').collect::<Vec<&str>>().into_iter();
        let b1n = second_pair.next()?.parse::<u32>().unwrap();
        let b1_2n = second_pair.next()?.parse::<u32>().unwrap();
        let overlaps = if a1n == b1n {
            1
        }else if a1n <= b1n {
            u32::from(b1_2n <= a1_2n)
        } else {u32::from(a1_2n <= b1_2n)};

        num_overlapping_pairs += overlaps;
    }
    Some(num_overlapping_pairs)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut num_overlapping_pairs = 0u32;
    for l in input.lines() {
        let mut pairs = l.split(',');

        let first_pair = pairs.next()?;
        let mut first_pair = first_pair.split('-').collect::<Vec<&str>>().into_iter();
        let a1n = first_pair.next()?.parse::<u32>().unwrap();
        let a1_2n = first_pair.next()?.parse::<u32>().unwrap();
        let mut second_pair = pairs.next()?.split('-').collect::<Vec<&str>>().into_iter();
        let b1n = second_pair.next()?.parse::<u32>().unwrap();
        let b1_2n = second_pair.next()?.parse::<u32>().unwrap();
        let overlaps =
        if a1_2n < b1n || b1_2n < a1n {
            0
        }else {
            1
        };
        num_overlapping_pairs += overlaps;
    }
    println!("Found {} overlaps in {} lines", num_overlapping_pairs, input.lines().count());
    Some(num_overlapping_pairs)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
