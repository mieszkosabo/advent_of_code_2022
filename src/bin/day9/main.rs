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

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    DownRight,
    DownLeft,
    UpLeft,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Coord(i32, i32);

impl Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl From<Coord> for HeadTailRelationship {
    fn from(c: Coord) -> Self {
        if c.0 == 0 && c.1 == 0 {
            return HeadTailRelationship::Covers;
        }

        // adjacent
        if c.0.abs() <= 1 && c.1.abs() <= 1 {
            let dir: Direction = c.into();
            return HeadTailRelationship::Adjacent(dir);
        }

        // two step
        let dir: Direction = c.into();
        HeadTailRelationship::TwoStep(dir)
    }
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "R" => Direction::Right,
            "U" => Direction::Up,
            "L" => Direction::Left,
            "D" => Direction::Down,
            _ => panic!("unexpected direction"),
        }
    }
}

impl From<Direction> for Coord {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Coord(0, 1),
            Direction::Down => Coord(0, -1),
            Direction::Left => Coord(-1, 0),
            Direction::Right => Coord(1, 0),
            Direction::UpRight => Coord(1, 1),
            Direction::DownRight => Coord(1, -1),
            Direction::DownLeft => Coord(-1, -1),
            Direction::UpLeft => Coord(-1, 1),
        }
    }
}

impl From<Coord> for Direction {
    fn from(c: Coord) -> Self {
        let normalized = Coord(c.0.signum(), c.1.signum());
        match normalized {
            Coord(0, 1) => Direction::Up,
            Coord(0, -1) => Direction::Down,
            Coord(-1, 0) => Direction::Left,
            Coord(1, 0) => Direction::Right,
            Coord(1, 1) => Direction::UpRight,
            Coord(1, -1) => Direction::DownRight,
            Coord(-1, -1) => Direction::DownLeft,
            Coord(-1, 1) => Direction::UpLeft,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum HeadTailRelationship {
    /// H and T are in the same spot
    Covers,

    /// eg.: or .....
    /// .....   ..H..
    /// .TH..   .T...
    /// .....   .....
    Adjacent(Direction),

    /// eg.: or
    /// .....   ...H.
    /// .....   .....
    /// .T.H.   ..T..
    /// .....   .....
    TwoStep(Direction),
}

type Motion = (Direction, usize);

fn move_head(state: HeadTailRelationship, head_move: Direction) -> HeadTailRelationship {
    match state {
        HeadTailRelationship::Covers => HeadTailRelationship::Adjacent(head_move),
        HeadTailRelationship::Adjacent(dir) => {
            let new_coord = Coord::from(dir) + Coord::from(head_move);
            println!("head move {:?}", new_coord);
            let dir = new_coord.into();
            println!("head move dir {:?}", dir);

            dir
        }
        HeadTailRelationship::TwoStep(_) => panic!("impossible state"),
    }
}

fn move_tail(state: HeadTailRelationship) -> (HeadTailRelationship, Option<Direction>) {
    match state {
        HeadTailRelationship::Covers => (HeadTailRelationship::Covers, None),
        HeadTailRelationship::Adjacent(dir) => (HeadTailRelationship::Adjacent(dir), None),
        HeadTailRelationship::TwoStep(dir) => match dir {
            Direction::UpRight => (HeadTailRelationship::Adjacent(Direction::Right), Some(dir)),
            Direction::DownRight => (HeadTailRelationship::Adjacent(Direction::Right), Some(dir)),
            Direction::DownLeft => (HeadTailRelationship::Adjacent(Direction::Left), Some(dir)),
            Direction::UpLeft => (HeadTailRelationship::Adjacent(Direction::Left), Some(dir)),
            dir => (HeadTailRelationship::Adjacent(dir), Some(dir)),
        },
    }
}

fn simulation_interpreter(series: Vec<Motion>) -> usize {
    let mut positions_set: HashSet<Coord> = HashSet::new();
    let mut current_state = HeadTailRelationship::Covers;
    let mut current_position = Coord(0, 0);

    for (dir, repeat_times) in series.iter() {
        for _ in 0..*repeat_times {
            let new_state = move_head(current_state, *dir);
            println!("after Head move: {:?}", new_state);
            let (new_state, tail_move) = move_tail(new_state);
            println!("after Tail move: {:?}", new_state);
            current_state = new_state;

            if let Some(tail_move) = tail_move {
                current_position = current_position + tail_move.into();
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
