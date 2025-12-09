use std::cmp::max;
use std::env;
use std::fs;

const INPUT_BASE: &str = "src/2025/input/";
const CURR_DAY: u8 = 9;

type Coordinate = (u32, u32);

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let is_test_mode = args.iter().any(|arg| arg == "--test");

    let filename = if is_test_mode {
        format!("{}{}{}{}", INPUT_BASE, "day", CURR_DAY, "_test.input")
    } else {
        format!("{}{}{}{}", INPUT_BASE, "day", CURR_DAY, ".input")
    };

    println!(
        "📆 Day {}{}:",
        CURR_DAY,
        if is_test_mode { " (DEBUG)" } else { "" }
    );
    println!("------------");
    let file_content = fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Failed to read file: {}", filename));

    let coordinates: Vec<Coordinate> = file_content
        .lines()
        .map(|l| {
            let nums: Vec<u32> = l
                .split(',')
                .map(|n| n.trim().parse().expect("Parse error"))
                .collect();

            match nums.as_slice() {
                [x, y] => (*x, *y),
                _ => panic!("Line did not contain exactly 2 numbers"),
            }
        })
        .collect();

    task_one(&coordinates);
    task_two(&coordinates);

    println!();
}

#[derive(Debug)]
struct DistanceSquare {
    c1: Coordinate,
    c2: Coordinate,
    area: u64,
}

fn task_one(coordinates: &[Coordinate]) {
    let mut distance_squares: Vec<DistanceSquare> = vec![];

    for (i, &c1) in coordinates.iter().enumerate() {
        for &c2 in &coordinates[i + 1..] {
            distance_squares.push(DistanceSquare {
                c1: c1,
                c2: c2,
                area: (c1.0.abs_diff(c2.0) as u64 + 1) * (c1.1.abs_diff(c2.1) as u64 + 1),
            });
        }
    }

    distance_squares.sort_by(|a, b| b.area.partial_cmp(&a.area).unwrap());

    let biggest_square = distance_squares
        .first()
        .expect("Need to have at least one rectangle.");
    println!(
        "Task 1 - The largest square is {} with tiles {:?}, {:?}",
        biggest_square.area, biggest_square.c1, biggest_square.c2
    )
}

fn task_two(coordinates: &[Coordinate]) {}
