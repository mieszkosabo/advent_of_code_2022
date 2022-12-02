use std::{fmt::Debug, fs, path::Path};

pub fn read_input_from_path<P>(path: P) -> String
where
    P: AsRef<Path> + Debug + Copy,
{
    fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Could not read input from path: {:?}", path))
}

pub fn read_input_for_day(day: u32) -> String {
    read_input_from_path(format!("src/bin/day{}/input.in", day).as_str())
}
