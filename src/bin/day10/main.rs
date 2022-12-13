use std::ops::Div;

use lib::io_utils::read_input_for_day;

fn main() {
    println!("Part One: {}", part_one());
    part_two();
}

enum Operation {
    Noop,
    Addx(i32),
}

const IMPORTANT_CYCLES: [i32; 6] = [20, 60, 100, 140, 180, 220];

fn maybe_save_signal(signals: &mut Vec<i32>, cycle: i32, x_register: i32) {
    if IMPORTANT_CYCLES.contains(&cycle) {
        println!("cycle: {}, register: {}", cycle, x_register);
        signals.push(cycle * x_register);
    }
}

fn part_one() -> i32 {
    let input = read_input_for_day(10);
    let ops = parse_input(input);

    let mut x_register = 1;
    let mut current_cycle = 1;
    let mut signals: Vec<i32> = vec![];

    ops.iter().for_each(|op| match *op {
        Operation::Noop => {
            current_cycle += 1;
            maybe_save_signal(&mut signals, current_cycle, x_register);
        }
        Operation::Addx(value) => {
            current_cycle += 1;
            maybe_save_signal(&mut signals, current_cycle, x_register);

            current_cycle += 1;
            x_register += value;
            maybe_save_signal(&mut signals, current_cycle, x_register);
        }
    });

    signals.iter().sum()
}

// ---------------------
type Crt = [[char; 40]; 6];

fn draw_crt(crt: &Crt) {
    crt.iter().for_each(|line| {
        line.iter().for_each(|pixel| {
            print!("{}", pixel);
        });
        println!();
    });
}

fn maybe_draw_pixel_on_crt(current_pixel: i32, sprite_mid: i32, crt: &mut Crt) {
    println!("curr pixel {}, sprite_mid: {}", current_pixel, sprite_mid);
    let row_idx = current_pixel.div(40);
    let column_idx = current_pixel % 40;
    if (sprite_mid - column_idx).abs() <= 1 {
        crt[row_idx as usize][column_idx as usize] = '#';
    }
}

fn part_two() {
    let input = read_input_for_day(10);
    let ops = parse_input(input);

    let mut x_register = 1;
    let mut current_cycle = 0;

    let mut crt: Crt = [['.'; 40]; 6];

    ops.iter().for_each(|op| match *op {
        Operation::Noop => {
            current_cycle += 1;
            maybe_draw_pixel_on_crt(current_cycle, x_register, &mut crt);
        }
        Operation::Addx(value) => {
            current_cycle += 1;
            maybe_draw_pixel_on_crt(current_cycle, x_register, &mut crt);

            current_cycle += 1;
            x_register += value;
            maybe_draw_pixel_on_crt(current_cycle, x_register, &mut crt);
        }
    });

    draw_crt(&crt);
}

fn parse_input(input: String) -> Vec<Operation> {
    input
        .lines()
        .map(|line| match line {
            "noop" => Operation::Noop,
            addx => Operation::Addx(
                addx.split_ascii_whitespace()
                    .nth(1)
                    .unwrap()
                    .parse()
                    .unwrap(),
            ),
        })
        .collect()
}
