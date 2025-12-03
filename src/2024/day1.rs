use std::collections::HashMap;
use std::env;
use std::fs;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let is_test_mode = args.iter().any(|arg| arg == "--test");
    const INPUT_BASE: &str = "src/2024/input/";

    let filename = if is_test_mode {
        format!("{}{}", INPUT_BASE, "day1_test.input")
    } else {
        format!("{}{}", INPUT_BASE, "day1.input")
    };

    println!("📆 Day 1 {}:", if is_test_mode { "(DEBUG)" } else { "" });
    println!("------------");
    let contents =
        fs::read_to_string(&filename).expect(&format!("Failed to read file: {}", filename));
    let lines: Vec<&str> = contents.lines().collect();

    task_one(&lines);
    task_two(&lines);
    println!();
}

fn task_one(lines: &[&str]) {
    let (mut list_one, mut list_two): (Vec<u32>, Vec<u32>) = lines
        .iter()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();

            let a = parts.next()?.parse::<u32>().ok()?;
            let b = parts.next()?.parse::<u32>().ok()?;
            Some((a, b))
        })
        .unzip();

    list_one.sort_unstable();
    list_two.sort_unstable();

    let difference: u32 = list_one
        .into_iter()
        .zip(list_two.into_iter())
        .map(|(a, b)| a.abs_diff(b))
        .sum();

    println!("Task 1 - The list difference is: {}", difference);
}

fn task_two(lines: &[&str]) {
    let (list_one, list_two): (Vec<u32>, Vec<u32>) = lines
        .iter()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();

            let a = parts.next()?.parse::<u32>().ok()?;
            let b = parts.next()?.parse::<u32>().ok()?;
            Some((a, b))
        })
        .unzip();

    let mut number_counts = HashMap::new();

    for entry in list_one {
        if let Some(old_count) = number_counts.get(&entry) {
            number_counts.insert(entry, old_count + 1);
        } else {
            number_counts.insert(entry, 1);
        }
    }

    let similarity_score: u32 = list_two
        .iter()
        .filter_map(|entry| Some(entry * number_counts.get(&entry)?))
        .sum();

    println!("Task 2 - Similarity Score: {}", similarity_score);
}
