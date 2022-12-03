use std::{char, collections::HashSet};

use lib::io_utils::read_input_for_day;

fn main() {
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}

fn split_rucksack_into_compartments(rucksack: String) -> (String, String) {
    let (a, b) = rucksack.split_at(rucksack.len() / 2);
    (a.into(), b.into())
}

fn find_common_el(xs: Vec<String>) -> char {
    let sets: Vec<HashSet<char>> = xs.iter().map(|s| s.chars().collect()).collect();

    let common_chars: Vec<&char> = sets[0]
        .iter()
        .filter(|c| sets[1..].iter().all(|s| s.contains(c)))
        .collect();
    *common_chars.first().unwrap().to_owned()
}

fn char_to_priority(c: char) -> u32 {
    match c.is_lowercase() {
        true => c as u32 - 96,       // 'a' is 97
        false => c as u32 - 64 + 26, // 'A' is 65
    }
}

fn part_one() -> u32 {
    let input = read_input_for_day(3);
    input
        .lines()
        .map(|line| {
            let (a, b) = split_rucksack_into_compartments(line.into());
            let common_el = find_common_el(vec![a, b]);
            char_to_priority(common_el)
        })
        .sum()
}

fn part_two() -> u32 {
    let input = read_input_for_day(3);
    let lines_vec: Vec<&str> = input.lines().collect();

    let mut sum = 0;
    for i in (0..lines_vec.len()).step_by(3) {
        let ls = lines_vec[i..i + 3].iter().map(|s| s.to_string()).collect();
        let c = find_common_el(ls);
        sum += char_to_priority(c);
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 7716);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 2973);
    }
}
