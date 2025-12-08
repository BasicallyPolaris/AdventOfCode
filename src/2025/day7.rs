use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time;

const TACHYON_START: char = 'S';
const TACHYON_SPLITTER: char = '^';

const INPUT_BASE: &str = "src/2025/input/";
const CURR_DAY: u8 = 7;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let is_test_mode = args.iter().any(|arg| arg == "--test");

    let filename = if is_test_mode {
        format!("{}{}{}{}", INPUT_BASE, "day", CURR_DAY, "_test.input")
    } else {
        format!("{}{}{}{}", INPUT_BASE, "day", CURR_DAY, ".input")
    };

    println!(
        "📆 Day {}{}:",
        CURR_DAY,
        if is_test_mode { " (DEBUG)" } else { "" }
    );
    println!("------------");
    let file_content = fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Failed to read file: {}", filename));

    let lines: Vec<&str> = file_content.lines().collect();

    let (start, board) = lines.split_at(1);
    let mut tachyon_index = 0;

    for i in 0..start[0].len() {
        if start[0]
            .chars()
            .nth(i)
            .unwrap_or_else(|| panic!("Reached end of first line before finding tachyon start."))
            == TACHYON_START
        {
            tachyon_index = i;
            break;
        }
    }

    task_one(tachyon_index, board);
    task_two(tachyon_index, board);

    println!();
}

fn task_one(start: usize, board: &[&str]) {
    let mut active_indices = HashSet::from([start]);
    let mut times_split: u64 = 0;

    for line in board.iter() {
        let mut next_indices = vec![];
        for &index in &active_indices {
            match line
                .chars()
                .nth(index)
                .expect("Couldn't visit active index")
            {
                TACHYON_SPLITTER => {
                    next_indices.push(index - 1);
                    next_indices.push(index + 1);
                    times_split += 1;
                }
                _ => {
                    next_indices.push(index);
                }
            }
        }
        active_indices.clear();
        active_indices.extend(next_indices);
    }

    println!(
        "Task 1 - Amount of times tachyons were split: {}",
        times_split
    );
}

fn task_two(start: usize, board: &[&str]) {
    let mut active_indices: HashMap<usize, u64> = HashMap::new();
    active_indices.insert(start, 1);
    let mut timelines: u64 = 1;

    for line in board.iter() {
        let mut new_indices = HashMap::new();
        for (&index, &count) in active_indices.iter() {
            match line
                .chars()
                .nth(index)
                .expect("Couldn't visit active index")
            {
                TACHYON_SPLITTER => {
                    new_indices
                        .entry(index - 1)
                        .and_modify(|timelines| *timelines += count)
                        .or_insert(count);
                    new_indices
                        .entry(index + 1)
                        .and_modify(|timelines| *timelines += count)
                        .or_insert(count);
                    timelines += count;
                }
                _ => {
                    new_indices
                        .entry(index)
                        .and_modify(|timelines| *timelines += count)
                        .or_insert(count);
                }
            }
        }
        active_indices = new_indices;
    }

    println!(
        "Task 2 - Amount of times tachyons split the time: {}",
        timelines
    );
}
