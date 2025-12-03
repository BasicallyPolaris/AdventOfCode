use std::env;
use std::fs;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let is_test_mode = args.iter().any(|arg| arg == "--test");
    const INPUT_BASE: &str = "src/2025/input/";

    let filename = if is_test_mode {
        format!("{}{}", INPUT_BASE, "day2_test.input")
    } else {
        format!("{}{}", INPUT_BASE, "day2.input")
    };

    println!("📆 Day 2 {}:", if is_test_mode { "(DEBUG)" } else { "" });
    println!("------------");
    let contents =
        fs::read_to_string(&filename).expect(&format!("Failed to read file: {}", filename));
    let ranges: Vec<&str> = contents.split(|char| char == ',').collect();

    task_one(&ranges);
    task_two(&ranges);
    println!();
}

fn task_one(ranges: &Vec<&str>) {
    let mut invalid_id_count: u64 = 0;
    for range in ranges {
        if let Some((range_start, range_end)) = range.split_once('-') {
            let range_start: u64 = range_start.parse().expect("Failed to parse range start");
            let range_end: u64 = range_end.parse().expect("Failed to parse range end");

            for id in range_start..=range_end {
                let id_string = id.to_string();
                let (first_half, second_half) = id_string.split_at(id_string.len() / 2);
                if first_half == second_half {
                    invalid_id_count += id;
                }
            }
        } else {
            eprintln!("Unexpected range format.");
            return;
        }
    }

    println!(
        "Task 1 - The added up invalid ids are: {}",
        invalid_id_count
    );
}

fn task_two(ranges: &Vec<&str>) {
    let mut invalid_id_count: u64 = 0;
    for range in ranges {
        if let Some((range_start, range_end)) = range.split_once('-') {
            let range_start: u64 = range_start.parse().expect("Failed to parse range start");
            let range_end: u64 = range_end.parse().expect("Failed to parse range end");

            for id in range_start..=range_end {
                let id_string = id.to_string();
                let length = id_string.len();

                let valid_substring_lengths: Vec<usize> = (1..length)
                    .filter(|substr_length| length % substr_length == 0)
                    .collect();

                for cut in valid_substring_lengths {
                    let number = &id_string[..cut];
                    let mut are_equal = true;
                    for i in 1..(length / cut) {
                        if number != &id_string[(i * cut)..(i + 1) * cut] {
                            are_equal = false;
                            break;
                        }
                    }
                    if are_equal {
                        invalid_id_count += id;
                        break;
                    }
                }
            }
        } else {
            eprintln!("Unexpected range format.");
            return;
        }
    }

    println!(
        "Task 2 - The added up invalid ids are: {}",
        invalid_id_count
    );
}
