use std::{fmt::Debug, fs, path::Path};

pub fn read_input_from_path<P>(path: P) -> String
where
    P: AsRef<Path> + Debug + Copy,
{
    fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Could not read input from path: {:?}", path))
}

pub fn read_example() -> String {
    read_input_from_path("src/bin/day1/input.example")
}

pub fn read_input() -> String {
    read_input_from_path("input.in")
}
