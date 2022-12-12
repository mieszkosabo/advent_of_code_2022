use lib::io_utils::read_input_for_day;

type Matrix2D<T> = Vec<Vec<T>>;

fn main() {
    let input = read_input_for_day(8);
    let data = parse_input(input);
    let (left, top, right, bottom) = precalculate_max_hights(&data);

    let width = data[0].len();
    let height = data.len();

    let mut visible = 0;
    for x in 1..width - 1 {
        for y in 1..height - 1 {
            if vec![left[y][x], top[x][y], right[y][x], bottom[x][y]]
                .iter()
                .any(|el| el < &data[y][x])
            {
                visible += 1;
            }
        }
    }
    visible = visible + 2 * width + 2 * height - 4;

    println!("Part One: {:?}", visible);
}

fn precalculate_max_hights(
    data: &Matrix2D<i32>,
) -> (Matrix2D<i32>, Matrix2D<i32>, Matrix2D<i32>, Matrix2D<i32>) {
    let width = data[0].len();
    let height = data.len();

    let mut max_value = 0;

    let mut max_heights_left = vec![vec![0; width]; height];

    for x in 0..width {
        for y in 0..height {
            if y == 0 {
                max_value = 0;
            }

            max_heights_left[x][y] = max_value;
            max_value = max_value.max(data[x][y]);
        }
    }

    let mut max_heights_right = vec![vec![0; width]; height];

    for x in 0..width {
        for y in (0..height).rev() {
            if y == height - 1 {
                max_value = 0;
            }

            max_heights_right[x][y] = max_value;
            max_value = max_value.max(data[x][y]);
        }
    }

    let mut max_heights_top = vec![vec![0; width]; height];

    for y in 0..height {
        for x in 0..width {
            if x == 0 {
                max_value = 0;
            }

            max_heights_top[y][x] = max_value;
            max_value = max_value.max(data[x][y]);
        }
    }

    let mut max_heights_bottom = vec![vec![0; width]; height];

    for y in 0..height {
        for x in (0..width).rev() {
            if x == width - 1 {
                max_value = 0;
            }

            max_heights_bottom[y][x] = max_value;
            max_value = max_value.max(data[x][y]);
        }
    }

    (
        max_heights_left,
        max_heights_top,
        max_heights_right,
        max_heights_bottom,
    )
}

fn parse_input(input: String) -> Matrix2D<i32> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}
