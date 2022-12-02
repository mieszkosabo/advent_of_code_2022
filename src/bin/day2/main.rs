use std::char;

use lib::io_utils::read_input_for_day;

fn main() {
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}

#[derive(Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
enum RoundResult {
    Won,
    Draw,
    Lost,
}
impl RoundResult {
    fn to_score(&self) -> i32 {
        match self {
            RoundResult::Won => 6,
            RoundResult::Draw => 3,
            RoundResult::Lost => 0,
        }
    }

    fn from_char(letter: char) -> Self {
        match letter {
            'X' => Self::Lost,
            'Y' => Self::Draw,
            'Z' => Self::Won,
            _ => panic!("at the disco"),
        }
    }

    fn to_shape(&self, opponent: &Shape) -> Shape {
        match opponent {
            Shape::Rock => match self {
                RoundResult::Won => Shape::Paper,
                RoundResult::Draw => Shape::Rock,
                RoundResult::Lost => Shape::Scissors,
            },
            Shape::Paper => match self {
                RoundResult::Won => Shape::Scissors,
                RoundResult::Draw => Shape::Paper,
                RoundResult::Lost => Shape::Rock,
            },
            Shape::Scissors => match self {
                RoundResult::Won => Shape::Rock,
                RoundResult::Draw => Shape::Scissors,
                RoundResult::Lost => Shape::Paper,
            },
        }
    }
}

impl Shape {
    fn from_char(letter: char) -> Self {
        match letter {
            'A' => Self::Rock,
            'X' => Self::Rock,

            'B' => Self::Paper,
            'Y' => Self::Paper,

            'C' => Self::Scissors,
            'Z' => Self::Scissors,

            _ => panic!("invalid letter!"),
        }
    }

    fn to_score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn play_round(&self, other: &Shape) -> RoundResult {
        match self {
            Shape::Rock => match other {
                Shape::Rock => RoundResult::Draw,
                Shape::Paper => RoundResult::Lost,
                Shape::Scissors => RoundResult::Won,
            },
            Shape::Paper => match other {
                Shape::Rock => RoundResult::Won,
                Shape::Paper => RoundResult::Draw,
                Shape::Scissors => RoundResult::Lost,
            },
            Shape::Scissors => match other {
                Shape::Rock => RoundResult::Lost,
                Shape::Paper => RoundResult::Won,
                Shape::Scissors => RoundResult::Draw,
            },
        }
    }
}

fn part_one() -> i32 {
    let input = read_input_for_day(2);
    input
        .lines()
        .map(|l| {
            let row_elements: Vec<char> = l.split_whitespace().flat_map(|el| el.chars()).collect();
            (
                Shape::from_char(row_elements[0]),
                Shape::from_char(row_elements[1]),
            )
        })
        .fold(0, |acc, (opponent_shape, our_shape)| {
            acc + our_shape.to_score() + our_shape.play_round(&opponent_shape).to_score()
        })
}

fn part_two() -> i32 {
    let input = read_input_for_day(2);
    input
        .lines()
        .map(|l| {
            let row_elements: Vec<char> = l.split_whitespace().flat_map(|el| el.chars()).collect();
            (
                Shape::from_char(row_elements[0]),
                RoundResult::from_char(row_elements[1]),
            )
        })
        .fold(0, |acc, (opponent_shape, result)| {
            let our_shape = result.to_shape(&opponent_shape);
            acc + our_shape.to_score() + our_shape.play_round(&opponent_shape).to_score()
        })
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 13484);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 13433);
    }
}
