use std::env;
use std::fs;

const ADD_STRING: &str = "+";
const MUL_STRING: &str = "*";

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let is_test_mode = args.iter().any(|arg| arg == "--test");
    const INPUT_BASE: &str = "src/2025/input/";

    let filename = if is_test_mode {
        format!("{}{}", INPUT_BASE, "day6_test.input")
    } else {
        format!("{}{}", INPUT_BASE, "day6.input")
    };

    println!("📆 Day 6 {}:", if is_test_mode { "(DEBUG)" } else { "" });
    println!("------------");
    let file_content = fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Failed to read file: {}", filename));

    let lines: Vec<&str> = file_content.lines().collect();

    let (numbers_strings_array, operations_array) = lines.split_at(lines.len() - 1);
    let numbers_array: Vec<Vec<u64>> = numbers_strings_array
        .iter()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|n| n.parse::<u64>().expect("Couldn't parse numbers"))
                .collect()
        })
        .collect();

    let operations: Vec<&str> = operations_array[0].split_ascii_whitespace().collect();

    task_one(&numbers_array, &operations);
    task_two(lines);

    println!();
}

fn task_one(numbers: &[Vec<u64>], operations: &[&str]) {
    let mut result = 0;

    for i in 0..operations.len() {
        match operations[i] {
            ADD_STRING => {
                result += numbers.iter().fold(0, |acc, nums| {
                    acc + nums
                        .get(i)
                        .expect("Something went wrong while adding up array entries")
                })
            }
            MUL_STRING => {
                result += numbers.iter().fold(1, |acc, nums| {
                    acc * nums
                        .get(i)
                        .expect("Something went wrong while adding up array entries")
                })
            }
            _ => panic!("Unexpected operation: {}", operations[i]),
        }
    }

    println!("Task 1 - The result of the homework is: {}", result);
}

fn task_two(lines: Vec<&str>) {
    if lines.is_empty() {
        return;
    }

    let first_row = lines[0];

    let result = {
        let mut result: u64 = 0;
        let mut current_numbers: Vec<u64> = vec![];
        let mut current = String::new();

        for i in (0..first_row.len()).rev() {
            for j in 0..lines.len() {
                let current_char = lines
                    .get(j)
                    .expect("Couldnt get line number")
                    .chars()
                    .nth(i)
                    .unwrap_or(' ');

                match current_char {
                    ' ' => {
                        if j == lines.len() - 1 && !current.is_empty() {
                            current_numbers.push(current.parse::<u64>().unwrap_or_else(|_| {
                                panic!("Couldn't parse current numbers string: {}", current)
                            }));
                            current.clear();
                            continue;
                        }
                    }
                    '+' => {
                        current_numbers.push(current.parse::<u64>().unwrap_or_else(|_| {
                            panic!("Couldn't parse current numbers string: {}", current)
                        }));
                        result += current_numbers.iter().sum::<u64>();
                        current_numbers.clear();
                        current.clear();
                    }
                    '*' => {
                        current_numbers.push(current.parse::<u64>().unwrap_or_else(|_| {
                            panic!("Couldn't parse current numbers string: {}", current)
                        }));
                        result += current_numbers.iter().fold(1, |acc, x| acc * x);
                        current_numbers.clear();
                        current.clear();
                    }
                    _ => {
                        // Should be a number
                        current.push(current_char);
                    }
                }
            }
        }

        result
    };

    println!("Task 2 - The result of the homework is: {}", result);
}
