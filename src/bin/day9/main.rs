use core::panic;
use std::{
    collections::HashSet,
    ops::{Add, Sub},
};

use lib::io_utils::read_input_for_day;

fn main() {
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}

fn part_one() -> usize {
    let input = read_input_for_day(9);
    let motions = parse_input(input);

    simulation_interpreter(motions, 2)
}

fn part_two() -> usize {
    let input = read_input_for_day(9);
    let motions = parse_input(input);

    simulation_interpreter(motions, 10)
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Coord(i32, i32);

impl Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Coord {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Coord(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl From<&str> for Coord {
    fn from(s: &str) -> Self {
        match s {
            "R" => Coord(1, 0),
            "U" => Coord(0, 1),
            "L" => Coord(-1, 0),
            "D" => Coord(0, -1),
            _ => panic!("unexpected direction"),
        }
    }
}

impl Coord {
    fn new() -> Self {
        Self(0, 0)
    }
}

fn move_head(state: Coord, direction: Coord) -> Coord {
    state + direction
}

/// move one position towards the other with the algorithm
/// as specified in the task
fn move_toward(position: Coord, other_position: Coord) -> Coord {
    let delta = other_position - position;
    if delta.0.abs() <= 1 && delta.1.abs() <= 1 {
        position
    } else {
        let tail_move = Coord(delta.0.signum(), delta.1.signum());
        position + tail_move
    }
}

fn simulation_interpreter(series: Vec<Motion>, ropes_num: usize) -> usize {
    let mut tail_positions_set: HashSet<Coord> = HashSet::new();

    // current_positions[0] is head
    // current_positions[n - 1] is tail
    let mut current_positions = vec![Coord::new(); ropes_num];

    for (dir, repeat_times) in series.iter() {
        for _ in 0..*repeat_times {
            current_positions[0] = move_head(current_positions[0], *dir);
            for rope_idx in 1..ropes_num {
                current_positions[rope_idx] =
                    move_toward(current_positions[rope_idx], current_positions[rope_idx - 1]);
            }

            tail_positions_set.insert(current_positions[ropes_num - 1]);
        }
    }

    tail_positions_set.len()
}

type Motion = (Coord, usize);

fn parse_input(input: String) -> Vec<Motion> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split_ascii_whitespace();
            (
                iter.next().unwrap().into(),
                iter.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}
