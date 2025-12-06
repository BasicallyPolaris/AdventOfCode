use regex::Regex;
use std::env;
use std::fs;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let is_test_mode = args.iter().any(|arg| arg == "--test");
    const INPUT_BASE: &str = "src/2024/input/";

    let filename = if is_test_mode {
        format!("{}{}", INPUT_BASE, "day3_test.input")
    } else {
        format!("{}{}", INPUT_BASE, "day3.input")
    };

    println!("📆 Day 3 {}:", if is_test_mode { "(DEBUG)" } else { "" });
    println!("------------");
    let contents = fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Failed to read file: {}", filename));

    task_one(&contents);
    task_two(&contents);
    println!();
}

fn task_one(input: &str) {
    let mul_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut mul_pairs = vec![];

    for (_, [mul_1, mul_2]) in mul_regex.captures_iter(&input).map(|mul| mul.extract()) {
        mul_pairs.push((
            mul_1.parse::<u32>().expect("Could not unwrap first number"),
            mul_2
                .parse::<u32>()
                .expect("Could not unwrap second number"),
        ));
    }

    let mut result = 0;
    for (x, y) in mul_pairs {
        result += x * y;
    }

    println!("Task 1 - The result is: {}", result);
}

fn task_two(input: &str) {
    let parts: Vec<&str> = input.split("don't()").collect();

    let mut valid_strings: Vec<&str> = vec![parts[0]];

    for part in &parts[1..] {
        valid_strings.extend(part.split("do()").skip(1));
    }

    let mul_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut mul_pairs = vec![];

    for (_, [mul_1, mul_2]) in mul_regex
        .captures_iter(&valid_strings.concat())
        .map(|mul| mul.extract())
    {
        mul_pairs.push((
            mul_1.parse::<u32>().expect("Could not unwrap first number"),
            mul_2
                .parse::<u32>()
                .expect("Could not unwrap second number"),
        ));
    }

    let mut result = 0;
    for (x, y) in mul_pairs {
        result += x * y;
    }

    println!("Task 1 - The result is: {}", result);
}
