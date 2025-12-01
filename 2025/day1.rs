use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_test_mode = args.iter().any(|arg| arg == "--test");

    let filename = if is_test_mode {
        "day1_test.input"
    } else {
        "day1.input"
    };

    println!("Reading from: {}", filename);
    println!("---");
    let contents =
        fs::read_to_string(filename).expect(&format!("Failed to read file: {}", filename));
    let commands: Vec<&str> = contents.lines().collect();

    task_one(&commands);
    task_two(&commands);
}

fn task_one(commands: &Vec<&str>) {
    // We start at position 50 & need to count how often we reach 0 after moves.
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
        if position == COUNT_GOAL.into() {
            password += 1;
        }
    }

    println!("1: The password is: {}", password);
}

fn task_two(commands: &Vec<&str>) {
    const COUNT_GOAL: u8 = 0;
    const DIAL_MODULO: u8 = 100;
    let mut position: i32 = 50;
    let mut password = 0;

    for command in commands {
        let (direction, number_str) = command.split_at(1);
        match number_str.parse::<i32>() {
            Ok(rotation) => {
                let prev_position = position;
                let modulo_rotation = rotation % 100;

                match direction {
                    "L" => {
                        let immediate_position = position - modulo_rotation;

                        if immediate_position <= COUNT_GOAL.into() {
                            password += 1;
                        }

                        password += rotation / 100;

                        position = rotate_left(position, rotation);
                    }
                    "R" => {
                        let immediate_position = position + modulo_rotation;

                        if immediate_position >= DIAL_MODULO.into() {
                            password += 1;
                        }

                        password += rotation / 100;

                        position = rotate_right(position, rotation);
                    }
                    _ => {
                        eprintln!("Unexpected direction.");
                    }
                }
                println!(
                    "Position: {} - Command: {} - Password: {}",
                    position, command, password
                );
            }
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
