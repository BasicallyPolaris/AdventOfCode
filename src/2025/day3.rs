use std::env;
use std::fs;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let is_test_mode = args.iter().any(|arg| arg == "--test");
    const INPUT_BASE: &str = "src/2025/input/";

    let filename = if is_test_mode {
        format!("{}{}", INPUT_BASE, "day3_test.input")
    } else {
        format!("{}{}", INPUT_BASE, "day3.input")
    };

    println!("📆 Day 3 {}:", if is_test_mode { "(DEBUG)" } else { "" });
    println!("------------");
    let contents =
        fs::read_to_string(&filename).expect(&format!("Failed to read file: {}", filename));
    let battery_banks: Vec<&str> = contents.lines().collect();

    task_one(&battery_banks);
    task_two(&battery_banks);
    println!();
}

fn task_one(battery_banks: &Vec<&str>) {
    let mut joltages: Vec<u32> = Vec::new();
    for battery_bank in battery_banks {
        let mut idx_one = battery_bank.chars().nth(0).unwrap_or('0');
        let mut idx_two = battery_bank.chars().nth(1).unwrap_or('0');
        for current in battery_bank.chars().skip(2) {
            if idx_two > idx_one {
                (idx_one, idx_two) = (idx_two, current);
            } else if current > idx_two {
                idx_two = current;
            }
        }

        let joltage_str = format!("{}{}", idx_one, idx_two);
        let joltage_result = joltage_str.parse::<u32>();

        match joltage_result {
            Ok(joltage) => {
                joltages.push(joltage);
            }
            Err(e) => {
                eprintln!(
                    "Something went wrong while parsing '{}': {}",
                    joltage_str, e
                );
            }
        }
    }

    println!(
        "Task 1 - The total joltages are: {}",
        joltages.iter().sum::<u32>()
    )
}

struct JoltageCell {
    joltage: char,
}

fn task_two(battery_banks: &Vec<&str>) {
    let mut joltages: Vec<u64> = Vec::new();
    for battery_bank in battery_banks {
        let mut joltage_cells: Vec<JoltageCell> = (0..12)
            .map(|i| JoltageCell {
                joltage: battery_bank.chars().nth(i).unwrap_or('0'),
            })
            .collect();

        for current in battery_bank.chars().skip(12) {
            let mut took_current = false;
            for i in 0..11 {
                if joltage_cells[i].joltage < joltage_cells[i + 1].joltage {
                    joltage_cells.remove(i);
                    joltage_cells.push(JoltageCell { joltage: current });
                    took_current = true;
                    break;
                }
            }

            if !took_current {
                match joltage_cells.last() {
                    Some(last_cell) => {
                        if last_cell.joltage < current {
                            joltage_cells.pop();
                            joltage_cells.push(JoltageCell { joltage: current });
                        }
                    }
                    _ => {
                        eprintln!(
                            "Unexpected error: Did not find last joltage_cell in joltage_cells: {}",
                            joltage_cells
                                .iter()
                                .map(|cell| cell.joltage)
                                .collect::<String>()
                        );
                    }
                }
            }
        }

        let joltage_result = joltage_cells
            .iter()
            .map(|cell| cell.joltage)
            .collect::<String>()
            .parse::<u64>();

        match joltage_result {
            Ok(joltage) => {
                joltages.push(joltage);
            }
            Err(e) => {
                eprintln!("Something went wrong while parsing: {}", e);
            }
        }
    }

    println!(
        "Task 2 - The total joltages are: {}",
        joltages.iter().sum::<u64>()
    )
}
