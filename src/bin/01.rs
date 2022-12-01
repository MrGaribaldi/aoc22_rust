use itertools::Itertools;
use std::str::FromStr;

#[derive(Default)]
struct MaxCaloriesCarried(u32, u32, u32);

impl MaxCaloriesCarried {
    fn insert(&mut self, calories: u32) {
        if calories < self.2 {
            return;
        }
        if calories < self.1 {
            self.2 = calories;
            return;
        }
        self.2 = self.1;
        if calories < self.0 {
            self.1 = calories;
            return;
        }
        self.1 = self.0;
        self.0 = calories;

    }

    fn get_total(&self) -> u32 {
        self.0 + self.1 + self.2
    }
    
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut highest_calorie_count: u32 = 0;
    let mut current_calories: u32 = 0;

    for l in input.lines() {
        if l.is_empty() {
            if highest_calorie_count < current_calories {
                highest_calorie_count = current_calories;
            } 
            current_calories = 0;
            continue;
        }
        current_calories += l.parse::<u32>().unwrap();
    }


    if current_calories > highest_calorie_count {
        Some(current_calories)
    } else {
        Some(highest_calorie_count)
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut mcc:MaxCaloriesCarried = MaxCaloriesCarried::default();
    let mut calories = 0u32;
    for l in input.lines(){
        if l.is_empty() {
            mcc.insert(calories);
            calories = 0;
            continue;
        }
        calories += l.parse::<u32>().unwrap();
    }
    Some(mcc.get_total())
}

///Solutions of others, will supplement with proper attribution tomorrow.
fn part_one_iterator_style(input: &str) -> Option<u32>{
    let mut calories:Vec<usize> = input.lines().group_by(|s| s.is_empty()).into_iter().filter_map(|g| g.1.filter_map(|s| usize::from_str(s).ok()).sum1()).collect();
    calories.sort();
    let calories = calories.into_iter().rev().next();
    Some( calories.unwrap() as u32)
}

fn part_two_iterator_style(input: &str) -> Option<u32>{
    let mut calories:Vec<usize> = input.lines().group_by(|s| s.is_empty()).into_iter().filter_map(|g| g.1.filter_map(|s| usize::from_str(s).ok()).sum1()).collect();
    calories.sort();
    let calories = calories.into_iter().rev().take(3).sum::<usize>();
    Some( calories as u32)
}


fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(1, part_one_iterator_style, input);
    advent_of_code::solve!(2, part_two, input);
    advent_of_code::solve!(2, part_two_iterator_style, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000u32));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(41000u32));
    }
}
