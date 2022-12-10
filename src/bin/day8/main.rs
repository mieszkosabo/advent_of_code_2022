use lib::io_utils::read_input_for_day;

type Matrix2D<T> = Vec<Vec<T>>;

fn main() {
    let input = read_input_for_day(8);
    let data = parse_input(input);
    let (top_left, bottom_right) = precalculate_max_hights(&data);

    let width = data[0].len();
    let height = data.len();

    println!("{:?}", top_left);
    println!("{:?}", bottom_right);

    let mut visible = 0;
    for x in 1..width - 1 {
        for y in 1..height - 1 {
            if vec![
                top_left[y][x].0,
                top_left[y][x].1,
                bottom_right[y][x].0,
                bottom_right[y][x].1,
            ]
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

fn precalculate_max_hights(data: &Matrix2D<i32>) -> (Matrix2D<(i32, i32)>, Matrix2D<(i32, i32)>) {
    let width = data[0].len();
    let height = data.len();

    let mut max_heights_top_left = vec![vec![(0, 0); width]; height];

    let mut max_left = 0;
    let mut max_top = 0;

    for y in 0..height {
        for x in 0..width {
            if y == 0 {
                max_top = 0;
            }
            if x == 0 {
                max_left = 0;
            }

            max_heights_top_left[y][x] = (max_top, max_left);
            max_top = if y == 0 {
                data[y][x].max(max_top)
            } else {
                println!("{}", data[y][x]);
                data[y][x].max(max_heights_top_left[y - 1][x].0)
            };
            max_left = data[y][x].max(max_left);

            // println!("{} {} {} {}", x, y, data[x][y], data[y][x]);
            // if y != 0 {
            //     max_top = data[x][y].max(max_heights_top_left[x][y - 1].0);
            // } else {
            //     max_top = data[x][y];
            // }

            // if x != 0 {
            //     max_left = data[x][y].max(max_heights_top_left[x - 1][y].1);
            // }
            // else {
            //     max_left = data[x][y];
            // }
        }
    }

    let mut max_heights_bottom_right = vec![vec![(0, 0); width]; height];

    let mut max_right = 0;
    let mut max_bottom = 0;

    for x in (0..width).rev() {
        for y in (0..height).rev() {
            if x == width - 1 {
                max_right = 0;
            }
            if y == height - 1 {
                max_bottom = 0;
            }

            max_heights_bottom_right[x][y] = (max_bottom, max_right);
            max_bottom = data[x][y].max(max_bottom);
            max_right = data[x][y].max(max_right);
        }
    }

    (max_heights_top_left, max_heights_bottom_right)
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
