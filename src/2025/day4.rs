use std::env;
use std::fs;

const PAPER_ROLL_CHAR: char = '@';
const REMOVED_ROL_CHAR: char = 'x';

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let is_test_mode = args.iter().any(|arg| arg == "--test");
    const INPUT_BASE: &str = "src/2025/input/";

    let filename = if is_test_mode {
        format!("{}{}", INPUT_BASE, "day4_test.input")
    } else {
        format!("{}{}", INPUT_BASE, "day4.input")
    };

    println!("📆 Day 4 {}:", if is_test_mode { "(DEBUG)" } else { "" });
    println!("------------");
    let contents =
        fs::read_to_string(&filename).expect(&format!("Failed to read file: {}", filename));
    let diagram_lines: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    task_one(&diagram_lines);
    task_two(&diagram_lines);
    println!();
}

fn task_one(diagram_lines: &Vec<Vec<char>>) {
    let x_dim = diagram_lines[0].len();
    let y_dim = diagram_lines.len();

    let mut diagram_lines_copy = diagram_lines.clone();

    let mut reachable_roll_count: u32 = 0;

    for i in 0..y_dim {
        for j in 0..x_dim {
            // Only check for actual paper rolls
            if diagram_lines[i][j] != PAPER_ROLL_CHAR {
                continue;
            }

            let mut adjacent_roll_count = 0;
            for off_y in -1..=1 {
                for off_x in -1..=1 {
                    if off_y == 0 && off_x == 0 {
                        continue;
                    }
                    let y = (i as isize) + off_y;
                    let x = (j as isize) + off_x;

                    // Don't check out of grid
                    if y < 0 || y >= (y_dim as isize) || x < 0 || x >= (x_dim as isize) {
                        continue;
                    }

                    if diagram_lines[y as usize][x as usize] == PAPER_ROLL_CHAR {
                        adjacent_roll_count += 1;
                    }
                }
            }

            if adjacent_roll_count < 4 {
                reachable_roll_count += 1;
                diagram_lines_copy[i][j] = REMOVED_ROL_CHAR;
            }
        }
    }

    println!(
        "Task 1 - The reachable paper roll count is: {}",
        reachable_roll_count
    );
}

fn check_neighboring_fields(
    curr_x: usize,
    curr_y: usize,
    x_dim: usize,
    y_dim: usize,
    mut callback: impl FnMut(usize, usize),
) {
    for off_y in -1..=1 {
        for off_x in -1..=1 {
            if off_y == 0 && off_x == 0 {
                continue;
            }
            let y = (curr_y as isize) + off_y;
            let x = (curr_x as isize) + off_x;

            // Don't check out of grid
            if y < 0 || y >= (y_dim as isize) || x < 0 || x >= (x_dim as isize) {
                continue;
            }

            callback(x as usize, y as usize);
        }
    }
}

fn check_specific_indices_for_removal(
    diagram_lines: &mut Vec<Vec<char>>,
    working_set: &mut Vec<(usize, usize)>,
    reachable_roll_count: &mut u32,
    (x, y): (usize, usize),
) {
    let x_dim = diagram_lines[0].len();
    let y_dim = diagram_lines.len();

    if diagram_lines[y][x] != PAPER_ROLL_CHAR {
        return;
    }

    let mut adjacent_roll_count = 0;

    check_neighboring_fields(x, y, x_dim, y_dim, |x, y| {
        if diagram_lines[y][x] == PAPER_ROLL_CHAR {
            adjacent_roll_count += 1;
        }
    });

    if adjacent_roll_count < 4 {
        *reachable_roll_count += 1;
        diagram_lines[y][x] = REMOVED_ROL_CHAR;

        // Add neighboring indices to a working set as they need to be rechecked
        check_neighboring_fields(x, y, x_dim, y_dim, |x, y| {
            if diagram_lines[y][x] == PAPER_ROLL_CHAR {
                working_set.push((x, y));
            }
        });
    }
}

fn mark_and_count_removable_rolls(
    diagram_lines: &mut Vec<Vec<char>>,
    working_set: &mut Vec<(usize, usize)>,
    reachable_roll_count: &mut u32,
) {
    let x_dim = diagram_lines[0].len();
    let y_dim = diagram_lines.len();

    for i in 0..y_dim {
        for j in 0..x_dim {
            check_specific_indices_for_removal(
                diagram_lines,
                working_set,
                reachable_roll_count,
                (j, i),
            );
        }
    }
}

pub fn task_two(diagram_lines: &Vec<Vec<char>>) {
    let mut diagram_lines_copy = diagram_lines.clone();
    let mut reachable_roll_count: u32 = 0;

    let mut working_set: Vec<(usize, usize)> = Vec::new();

    mark_and_count_removable_rolls(
        &mut diagram_lines_copy,
        &mut working_set,
        &mut reachable_roll_count,
    );

    while let Some(indices) = working_set.pop() {
        check_specific_indices_for_removal(
            &mut diagram_lines_copy,
            &mut working_set,
            &mut reachable_roll_count,
            indices,
        );
    }
}

pub fn task_two_alt(diagram_lines: &Vec<Vec<char>>) {
    let mut diagram_lines_copy = diagram_lines.clone();
    let mut reachable_roll_count: u32 = 0;

    let mut working_set: Vec<(usize, usize)> = Vec::new();

    for y in 0..diagram_lines.len() {
        for x in 0..diagram_lines[0].len() {
            if diagram_lines[y][x] == PAPER_ROLL_CHAR {
                working_set.push((x, y));
            }
        }
    }

    while let Some(indices) = working_set.pop() {
        check_specific_indices_for_removal(
            &mut diagram_lines_copy,
            &mut working_set,
            &mut reachable_roll_count,
            indices,
        );
    }
}
