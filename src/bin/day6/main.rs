use lib::io_utils::read_input_for_day;

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

fn find_first_marker_idx(input: String, unique_fragment_len: usize) -> usize {
    let mut acc = vec![];
    let mut idx = 1;
    for c in input.chars() {
        if acc.contains(&c) {
            // rm elements before the duplicate (including the duplicate)
            acc = acc.split(|el| el == &c).nth(1).unwrap().to_vec();
            acc.push(c);
        } else {
            acc.push(c);
            if acc.len() == unique_fragment_len {
                break;
            }
        }

        idx += 1;
    }

    idx
}

fn part_one() -> usize {
    let input = read_input_for_day(6);

    find_first_marker_idx(input, 4)
}

fn part_two() -> usize {
    let input = read_input_for_day(6);

    find_first_marker_idx(input, 14)
}

#[cfg(test)]
mod tests {
    use crate::find_first_marker_idx;

    #[test]
    fn example_tests_4() {
        let test_cases = vec![
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        for (input, answer) in test_cases {
            assert_eq!(find_first_marker_idx(input.into(), 4), answer);
        }
    }

    #[test]
    fn example_tests_14() {
        let test_cases = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];

        for (input, answer) in test_cases {
            assert_eq!(find_first_marker_idx(input.into(), 14), answer);
        }
    }
}
