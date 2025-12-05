use std::env;
use std::fs;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let is_test_mode = args.iter().any(|arg| arg == "--test");
    const INPUT_BASE: &str = "src/2025/input/";

    let filename = if is_test_mode {
        format!("{}{}", INPUT_BASE, "day5_test.input")
    } else {
        format!("{}{}", INPUT_BASE, "day5.input")
    };

    println!("📆 Day 5 {}:", if is_test_mode { "(DEBUG)" } else { "" });
    println!("------------");
    let contents = fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Failed to read file: {}", filename));

    let mut at_ids_section = false;
    let mut ranges = Vec::new();
    let mut ids = Vec::new();

    for line in contents.lines() {
        if at_ids_section {
            ids.push(
                line.parse::<u64>()
                    .expect("Invalid Id could not be parsed."),
            );
            continue;
        }

        if line == "" {
            at_ids_section = true;
            continue;
        }

        ranges.push({
            let (start, end) = line
                .split_once("-")
                .expect("Invalid range format (missing '-')");

            (
                start.parse().expect("Invalid range start number"),
                end.parse().expect("Invalid range end number"),
            )
        });
    }

    task_one((&ranges, &ids));
    task_two(ranges);
    println!();
}

fn task_one((ranges, ids): (&[(u64, u64)], &[u64])) {
    let mut fresh_ids_count = 0;

    for id in ids {
        for (a, b) in ranges {
            if (*a..=*b).contains(id) {
                fresh_ids_count += 1;
                break;
            }
        }
    }

    println!("Task 1 - Number of fresh ids: {}", fresh_ids_count);
}

fn task_two(mut ranges: Vec<(u64, u64)>) {
    if ranges.is_empty() {
        println!("Task 2 - Number of fresh ids: 0");
        return;
    }

    ranges.sort_by_key(|r| r.0);

    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();

    let (mut current_start, mut current_end) = ranges[0];

    for &(next_start, next_end) in ranges.iter().skip(1) {
        if next_start <= current_end {
            current_end = current_end.max(next_end);
        } else {
            merged_ranges.push((current_start, current_end));
            current_start = next_start;
            current_end = next_end;
        }
    }

    merged_ranges.push((current_start, current_end));

    let mut total_count: u64 = 0;
    merged_ranges
        .iter()
        .for_each(|(s, e)| total_count += e - s + 1);

    println!("Task 2 - Number of all valid fresh ids: {}", total_count);
}
