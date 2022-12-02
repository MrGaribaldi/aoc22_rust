//Rock> A X
//Paper> B Y
//Scissor> C Z

use std::cmp::Ordering;

#[derive(PartialEq)]
enum GameMove{
    Rock,
    Paper,
    Scissor
}

impl PartialOrd for GameMove {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (GameMove::Rock, GameMove::Rock) | (GameMove::Paper, GameMove::Paper) |  (GameMove::Scissor, GameMove::Scissor) => Some(Ordering::Equal),
            (GameMove::Rock, GameMove::Paper) | (GameMove::Paper, GameMove::Scissor) | (GameMove::Scissor, GameMove::Rock) => Some(Ordering::Less),
            (GameMove::Rock, GameMove::Scissor) | (GameMove::Paper, GameMove::Rock) | (GameMove::Scissor, GameMove::Paper) => Some(Ordering::Greater)
        }
    }
}

impl GameMove {
    fn new(m: char) -> Option<GameMove> {
        match m {
            'A' | 'X' => Some(GameMove::Rock),
            'B' | 'Y' => Some(GameMove::Paper),
            'C' | 'Z' => Some(GameMove::Scissor),
            _ => None
        }
    }
}

fn getValue(g_move: &GameMove)-> u32{
    match g_move {
        GameMove::Rock => 1,
        GameMove::Paper => 2,
        GameMove::Scissor => 3,
    }
}

fn getScore(opponent: GameMove, player: GameMove) -> u32 {
    let move_value = getValue(&player);
    match player.partial_cmp(&opponent) {
        Some(Ordering::Equal) => 3 + move_value,
        Some(Ordering::Greater) => 6 + move_value,
        Some(Ordering::Less) => 0 + move_value,
        None => 0
    }
}

fn getLoosingMove(opponent: &char) -> GameMove {
    match opponent{
        'A' => GameMove::Scissor,
        'B' => GameMove::Rock,
        _ => GameMove::Paper
    }
}

fn getMatchingMove(opponent: &char) -> GameMove {
    match opponent{
        'A' => GameMove::Rock,
        'B' => GameMove::Paper,
        _ => GameMove::Scissor
    }
}

fn getWinningMove(opponent: &char) -> GameMove {
    match opponent{
        'A' => GameMove::Paper,
        'B' => GameMove::Scissor,
        _ => GameMove::Rock
    }
}
fn getRequiredMove( opponent: &char, desired_outcome: char) -> GameMove {
    match desired_outcome {
        'X' => getLoosingMove(&opponent),
        'Y' => getMatchingMove(&opponent),
        _ => getWinningMove(&opponent)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut score:u32 = 0; 
    for l in input.lines() {
        let mut moves = l.chars();
        let opponent = GameMove::new(moves.next()?)?;
        moves.next(); //blank
        let player = GameMove::new(moves.next()?)?;

        score += getScore(opponent, player);

    }
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut score:u32 = 0; 
    for l in input.lines() {
        let mut moves = l.chars();
        let opponent = moves.next()?;
        moves.next(); //blank
        let desired_outcome = moves.next()?;
        let required_move =  getRequiredMove( &opponent, desired_outcome);
        let opponent = GameMove::new(opponent)?;
        score += getScore(opponent, required_move);
    }
    Some(score)
}


fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
