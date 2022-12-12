use core::panic;
use std::{collections::HashSet, ops::Add};

use lib::io_utils::read_input_for_day;

fn main() {
    println!("Part One: {}", part_one());
}

fn part_one() -> usize {
    let input = read_input_for_day(9);
    let motions = parse_input(input);

    simulation_interpreter(motions)
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Coord(i32, i32);

impl Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
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

type HeadTailRelationship = Coord;

type Motion = (Coord, usize);

fn move_head(state: HeadTailRelationship, head_move: Coord) -> HeadTailRelationship {
    state + head_move
}

fn move_tail(state: HeadTailRelationship) -> (HeadTailRelationship, Option<Coord>) {
    if state.0.abs() <= 1 && state.1.abs() <= 1 {
        (state, None)
    } else if state.0.abs() > 1 {
        let tail_move = Some(Coord(state.0.signum(), state.1.signum()));
        (Coord(state.0.signum(), 0), tail_move)
    } else {
        let tail_move = Some(Coord(state.0.signum(), state.1.signum()));
        (Coord(0, state.1.signum()), tail_move)
    }
}

fn simulation_interpreter(series: Vec<Motion>) -> usize {
    let mut positions_set: HashSet<Coord> = HashSet::new();
    let mut current_state = Coord(0, 0);
    let mut current_position = Coord(0, 0);

    for (dir, repeat_times) in series.iter() {
        for _ in 0..*repeat_times {
            let new_state = move_head(current_state, *dir);
            println!("after Head move: {:?}", new_state);
            let (new_state, tail_move) = move_tail(new_state);
            println!("after Tail move: {:?}", new_state);
            current_state = new_state;

            if let Some(tail_move) = tail_move {
                current_position = current_position + tail_move;
                println!("moved tail {:?}", current_position);
            }
            println!();
            println!("{:?}", current_position);
            positions_set.insert(current_position);
        }
    }

    positions_set.len()
}

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
