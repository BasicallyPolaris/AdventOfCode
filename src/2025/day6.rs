use std::env;
use std::fs;
use std::num;

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
    task_two(numbers_strings_array, &operations);

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

fn task_two(numbers: &[&str], operations: &[&str]) {
    let numbers_array: Vec<u64> = {
        let mut result_vec = vec![];

        if !numbers.is_empty() {
            for i in 0..numbers[0].len() {
                let mut current = String::new();

                for j in 0..numbers.len() {
                    if let Some(c) = numbers[j].chars().nth(i) {
                        current.push(c);
                    }
                }

                result_vec.push(
                    current
                        .trim()
                        .parse::<u64>()
                        .expect("Couldn't parse number"),
                );
            }
        }
        result_vec
    };

    let mut result = 0;

    for i in 1..operations.len() {
        match operations[i] {
            ADD_STRING => {
                result += numbers_array.iter().fold(0, |acc, nums| {
                    acc + nums
                        .get(i)
                        .expect("Something went wrong while adding up array entries")
                })
            }
            MUL_STRING => {
                result += numbers_array.iter().fold(1, |acc, nums| {
                    acc * nums
                        .get(i)
                        .expect("Something went wrong while adding up array entries")
                })
            }
            _ => panic!("Unexpected operation: {}", operation),
        }
    }

    println!("Task 1 - The result of the homework is: {}", result);
}
