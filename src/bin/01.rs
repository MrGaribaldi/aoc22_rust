struct ElvesWithMostCalories(u32, u32, u32);

impl ElvesWithMostCalories {
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
    fn new() -> ElvesWithMostCalories {
        ElvesWithMostCalories(0, 0, 0)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut highest_calorie_count: u32 = 0;
    let mut current_calories: u32 = 0;

    for l in input.lines() {
        if l.len() == 0 {
            highest_calorie_count = if highest_calorie_count > current_calories {
                highest_calorie_count
            } else {
                current_calories
            };
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
    let mut elfs:ElvesWithMostCalories = ElvesWithMostCalories::new();
    let mut calories = 0u32;
    for l in input.lines(){
        if l.len() == 0 {
            elfs.insert(calories);
            calories = 0;
            continue;
        }
        calories += l.parse::<u32>().unwrap();
    }
    Some(elfs.get_total())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
