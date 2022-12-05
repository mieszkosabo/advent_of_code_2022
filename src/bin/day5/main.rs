use lib::io_utils::read_input_for_day;

type Crate = Vec<char>;
type Crates = Vec<Crate>;

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    from: usize,
    to: usize,
}

type Instructions = Vec<Instruction>;

enum MovingStrategy {
    StackLike,
    AtOnce,
}

fn main() {
    println!("{:?}", part_one());
    println!("{:?}", part_two());
}

fn move_crates(crates: &mut Crates, instr: &Instruction, moving_strategy: &MovingStrategy) {
    let mut moving_els = vec![];
    for _ in 0..instr.quantity {
        let c = crates[instr.from].pop().unwrap();
        moving_els.push(c);
    }

    match *moving_strategy {
        MovingStrategy::StackLike => {
            for c in moving_els {
                crates[instr.to].push(c);
            }
        }
        MovingStrategy::AtOnce => {
            for c in moving_els.iter().rev() {
                crates[instr.to].push(*c);
            }
        }
    }
}

fn part_one() -> String {
    let input = read_input_for_day(5);
    let (crates, instructions) = input_to_domain(input);
    let mut crates = crates;

    for instr in instructions {
        move_crates(&mut crates, &instr, &MovingStrategy::StackLike);
    }

    crates.iter().map(|stack| stack.last().unwrap()).collect()
}

fn part_two() -> String {
    let input = read_input_for_day(5);
    let (crates, instructions) = input_to_domain(input);
    let mut crates = crates;

    for instr in instructions {
        move_crates(&mut crates, &instr, &MovingStrategy::AtOnce);
    }

    crates.iter().map(|stack| stack.last().unwrap()).collect()
}

// parsing -------------------------------------------------------

fn split_input(input: String) -> (Vec<String>, Vec<String>) {
    let split = input
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut split = split.split(|line| line.is_empty());

    let crates_part = split.next().unwrap().into();
    let instructions_part = split.next().unwrap().into();
    (crates_part, instructions_part)
}

fn get_crates_from_input(input: &Vec<String>) -> Crates {
    let width = input[0].len();
    let height = input.len();

    let mut crates = vec![];

    // step by 4 to skip "] ["
    for column in (1..width).step_by(4) {
        let mut curr_crate = vec![];

        // skip the last row, since it is the one containing labels
        for row in (0..height - 1).rev() {
            let maybe_crate = input[row].chars().nth(column).unwrap();
            if maybe_crate != ' ' {
                curr_crate.push(maybe_crate);
            }
        }

        crates.push(curr_crate.clone());
    }

    crates
}

fn get_instructions_from_input(input: &[String]) -> Instructions {
    input
        .iter()
        .map(|line| {
            let mut line = line.split_ascii_whitespace();
            Instruction {
                quantity: line.nth(1).unwrap().parse().unwrap(),
                from: line.nth(1).unwrap().parse::<usize>().unwrap() - 1,
                to: line.nth(1).unwrap().parse::<usize>().unwrap() - 1,
            }
        })
        .collect()
}

fn input_to_domain(input: String) -> (Crates, Instructions) {
    let (crates_input, instructions_input) = split_input(input);
    (
        get_crates_from_input(&crates_input),
        get_instructions_from_input(&instructions_input),
    )
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), "JDTMRWCQJ");
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), "VHJDDCWRD");
    }
}
