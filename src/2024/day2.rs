use std::env;
use std::fs;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let is_test_mode = args.iter().any(|arg| arg == "--test");
    const INPUT_BASE: &str = "src/2024/input/";

    let filename = if is_test_mode {
        format!("{}{}", INPUT_BASE, "day2_test.input")
    } else {
        format!("{}{}", INPUT_BASE, "day2.input")
    };

    println!("📆 Day 2 {}:", if is_test_mode { "(DEBUG)" } else { "" });
    println!("------------");
    let contents =
        fs::read_to_string(&filename).expect(&format!("Failed to read file: {}", filename));
    let lines: Vec<&str> = contents.lines().collect();

    task_one(&lines);
    task_two(&lines);
    println!();
}

fn task_one(lines: &[&str]) {
    let numbers_lists: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .filter_map(|number| Some(number.parse::<u32>().unwrap()))
                .collect()
        })
        .collect();

    let mut safe_reports_count = 0;

    for list in numbers_lists {
        if list.len() <= 1 {
            safe_reports_count += 1;
        } else {
            let ascending = list[0] < list[1];
            let mut is_safe = true;

            for i in 0..list.len() - 1 {
                if !compare_ascending_decending(list[i], list[i + 1], ascending)
                    || list[i].abs_diff(list[i + 1]) > 3
                    || !list[i].abs_diff(list[i + 1]) == 0
                {
                    is_safe = false;
                    break;
                }
            }

            if is_safe {
                safe_reports_count += 1;
            }
        }
    }

    println!("Task 1 - Safe reports count: {}", safe_reports_count);
}

fn task_two(lines: &[&str]) {
    let numbers_lists: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .filter_map(|number| Some(number.parse::<u32>().unwrap()))
                .collect()
        })
        .collect();

    let mut safe_reports_count = 0;

    for mut list in numbers_lists {
        if list.len() <= 1 {
            safe_reports_count += 1;
        } else {
            let ascending = list[0] < list[1];
            let mut is_safe = true;

            for i in 0..list.len() - 1 {
                if i + 1 < list.len() {
                    if !compare_ascending_decending(list[i], list[i + 1], ascending)
                        || list[i].abs_diff(list[i + 1]) > 3
                        || !list[i].abs_diff(list[i + 1]) == 0
                    {
                        if is_safe {
                            is_safe = false;
                            list.remove(i + 1);
                            continue;
                        } else {
                            break;
                        }
                    }
                }
            }

            if is_safe {
                safe_reports_count += 1;
            }
        }
    }

    println!("Task 2 - Safe reports count: {}", safe_reports_count);
}

fn compare_ascending_decending(num1: u32, num2: u32, is_ascending: bool) -> bool {
    if is_ascending {
        num1 < num2
    } else {
        num2 < num1
    }
}
