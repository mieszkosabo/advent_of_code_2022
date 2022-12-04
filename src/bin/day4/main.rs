use std::ops::RangeInclusive;

use lib::io_utils::read_input_for_day;

fn main() {
    println!("Part One {}", part_one());
    println!("Part Two {}", part_two());
}

#[derive(Debug)]
enum Intersection {
    Bellow,
    BellowOverlap,
    Contained,
    Contains,
    Same,
    AboveOverlap,
    Above,
}

trait Intersect {
    fn intersect(&self, other: &RangeInclusive<i32>) -> Intersection;
}

impl Intersect for RangeInclusive<i32> {
    fn intersect(&self, other: &RangeInclusive<i32>) -> Intersection {
        let self_min = self.clone().min().unwrap();
        let self_max = self.clone().max().unwrap();

        let other_min = other.clone().min().unwrap();
        let other_max = other.clone().max().unwrap();

        if self_max < other_min {
            return Intersection::Bellow;
        }

        if self_min > other_max {
            return Intersection::Above;
        }

        if self_min == other_min && self_max == other_max {
            return Intersection::Same;
        }

        if self_min >= other_min && self_max <= other_max {
            return Intersection::Contained;
        }

        if self_min <= other_min && self_max >= other_max {
            return Intersection::Contains;
        }

        // the only cases left are BellowOverlap and AboveOverlap
        // so we can just simply compare two min values:
        if self_min < other_min {
            Intersection::BellowOverlap
        } else {
            Intersection::AboveOverlap
        }
    }
}

fn section_to_assignment(s: &str) -> RangeInclusive<i32> {
    let mut iter = s.split('-');
    let start = iter.next().unwrap().parse::<i32>().unwrap();
    let end = iter.next().unwrap().parse::<i32>().unwrap();

    start..=end
}

fn get_assignments() -> Vec<(RangeInclusive<i32>, RangeInclusive<i32>)> {
    let input = read_input_for_day(4);
    input
        .lines()
        .into_iter()
        .map(|line| line.split(',').collect::<Vec<&str>>())
        .map(|sections| {
            let s1 = sections[0];
            let s2 = sections[1];

            (section_to_assignment(s1), section_to_assignment(s2))
        })
        .collect()
}

fn part_one() -> usize {
    get_assignments()
        .into_iter()
        .filter(|(r1, r2)| {
            matches!(
                r1.intersect(r2),
                Intersection::Contained | Intersection::Contains | Intersection::Same
            )
        })
        .count()
}

fn part_two() -> usize {
    get_assignments()
        .into_iter()
        .filter(|(r1, r2)| !matches!(r1.intersect(r2), Intersection::Above | Intersection::Bellow))
        .count()
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 569);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 936);
    }
}
