use std::env;

#[path = "2024/mod.rs"]
mod y2024;
#[path = "2025/mod.rs"]
mod y2025;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut has_flags = false;

    if args.iter().any(|arg| arg == "--all") {
        run_all();
        has_flags = true;
    } else {
        if args.iter().any(|arg| arg == "--all-2024") {
            run_all_2024();
            has_flags = true;
        }
        if args.iter().any(|arg| arg == "--all-2025") {
            run_all_2025();
            has_flags = true;
        }
    }

    if !has_flags {
        run_today();
    }
}

fn run_all() {
    run_all_2024();
    println!();
    run_all_2025();
}

fn run_all_2024() {
    println!("🎄 Running Advent of Code 2024 🎁");
    println!("--------------------------------");
    y2024::day1::run();
    y2024::day2::run();
    y2024::day3::run();
}

fn run_all_2025() {
    println!("🎄 Running Advent of Code 2025 🎁");
    println!("--------------------------------");
    y2025::day1::run();
    y2025::day2::run();
    y2025::day3::run();
    y2025::day4::run();
    y2025::day5::run();
}

fn run_today() {
    y2025::day5::run();
}
