use std::env;
use std::fs;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let is_test_mode = args.iter().any(|arg| arg == "--test");
    const INPUT_BASE: &str = "src/2025/input/";

    let filename = if is_test_mode {
        format!("{}{}", INPUT_BASE, "day1_test.input")
    } else {
        format!("{}{}", INPUT_BASE, "day1.input")
    };

    println!("📆 Day 1 {}:", if is_test_mode { "(DEBUG)" } else { "" });
    println!("------------");
    let contents =
        fs::read_to_string(&filename).expect(&format!("Failed to read file: {}", filename));
    let commands: Vec<&str> = contents.lines().collect();

    task_one(&commands);
    task_two(&commands);
    println!();
}

fn task_one(commands: &Vec<&str>) {
    const COUNT_GOAL: u8 = 0;
    let mut position: i32 = 50;
    let mut password = 0;

    for command in commands {
        let (direction, number_str) = command.split_at(1);
        match number_str.parse::<i32>() {
            Ok(number) => match direction {
                "L" => {
                    position -= number;
                    position %= 100;
                }
                "R" => {
                    position += number;
                    position %= 100;
                }
                _ => {
                    eprintln!("Unexpected direction.");
                }
            },
            Err(e) => {
                eprintln!("Failed to parse number '{}': {}", number_str, e);
            }
        }
        if position == COUNT_GOAL as i32 {
            password += 1;
        }
    }

    println!("1: The password is: {}", password);
}

fn task_two(commands: &Vec<&str>) {
    let mut position: i32 = 50;
    let mut password = 0;

    for command in commands {
        let (direction, number_str) = command.split_at(1);
        match number_str.parse::<i32>() {
            Ok(rotation) => match direction {
                "L" => {
                    let absolute_position =
                        if position > 0 { 100 - position } else { 0 } + rotation;

                    password += absolute_position.abs() / 100;

                    position = rotate_left(position, rotation);
                }
                "R" => {
                    let absolute_position = position + rotation;

                    password += absolute_position.abs() / 100;

                    position = rotate_right(position, rotation);
                }
                _ => {
                    eprintln!("Unexpected direction.");
                }
            },
            Err(e) => {
                eprintln!("Failed to parse number '{}': {}", number_str, e);
            }
        }
    }

    println!("2: The password is: {}", password);
}

fn rotate_left(position: i32, rotation: i32) -> i32 {
    let mod_rotation = rotation % 100;
    let rotated_value = position - mod_rotation;

    if rotated_value < 0 {
        100 + rotated_value
    } else {
        rotated_value
    }
}

fn rotate_right(position: i32, rotation: i32) -> i32 {
    let mod_rotation = rotation % 100;
    let rotated_value = position + mod_rotation;

    if rotated_value >= 100 {
        rotated_value % 100
    } else {
        rotated_value
    }
}
