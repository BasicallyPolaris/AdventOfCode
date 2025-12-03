#[path = "2024/mod.rs"]
mod y2024;
#[path = "2025/mod.rs"]
mod y2025;

fn main() {
    run_all_2024();
    // run_all_2025();
}

fn run_all_2024() {
    println!("Running advent of code for 2024 🎄🎁");
    println!();
    y2024::day1::run();
    y2024::day2::run();
}

fn run_all_2025() {
    println!("Running advent of code for 2025 🎄🎁");
    println!();
    y2025::day1::run();
    y2025::day2::run();
    y2025::day3::run();
}
