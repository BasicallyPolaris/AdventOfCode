use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs;
use z3::ast::Int;
use z3::{Optimize, SatResult};

const ACTIVE_CHAR: char = '#';
const INACTIVE_CHAR: char = '.';
const DELIMITTER: char = ',';

const INPUT_BASE: &str = "src/2025/input/";
const CURR_DAY: u8 = 10;

struct Machine {
    goal_state: Vec<usize>,
    buttons: Vec<Vec<usize>>,
    joltage_requirements: Vec<u32>,
}

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

    let machines: Vec<Machine> = file_content
        .lines()
        .map(|line| {
            let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
            assert!(tokens.len() >= 3);

            let goal_state: Vec<usize> = tokens[0][1..tokens[0].len() - 1]
                .chars()
                .enumerate()
                .filter_map(|(i, state)| match state {
                    ACTIVE_CHAR => Some(i),
                    INACTIVE_CHAR => None,
                    _ => panic!("Unexpected char read when parsing state: {}", state),
                })
                .collect();

            let raw_cost = tokens[tokens.len() - 1];
            let joltage_requirements: Vec<u32> = raw_cost[1..raw_cost.len() - 1]
                .split(DELIMITTER)
                .map(|num| num.parse::<u32>().expect("Could not parse joltage cost"))
                .collect();

            let buttons: Vec<Vec<usize>> = tokens[1..tokens.len() - 1]
                .iter()
                .map(|s| {
                    assert!(s.len() >= 3, "Token too short: {}", s);

                    s[1..s.len() - 1]
                        .split(',')
                        .map(|num| {
                            num.trim()
                                .parse::<usize>()
                                .expect("Could not parse button number")
                        })
                        .collect()
                })
                .collect();

            Machine {
                goal_state,
                buttons,
                joltage_requirements,
            }
        })
        .collect();

    task_one(&machines);
    task_two(&machines);

    println!();
}

fn task_one(machines: &[Machine]) {
    let mut presses_needed = 0;

    for Machine {
        goal_state,
        buttons,
        joltage_requirements: _,
    } in machines
    {
        // println!("Goal state: {:?}, Buttons: {:?}", goal_state, buttons);
        if goal_state.is_empty() {
            continue;
        }

        let mut queue: VecDeque<(Vec<usize>, u32)> = VecDeque::new();
        let mut visited: HashSet<Vec<usize>> = HashSet::new();
        let mut found_goal = false;

        for button in buttons {
            if button == goal_state {
                presses_needed += 1;
                found_goal = true;
            }

            if visited.insert(button.clone()) {
                queue.push_back((button.clone(), 1));
            }
        }

        if found_goal {
            continue;
        }

        // Start the BFS Loop
        while let Some((current_state, current_cost)) = queue.pop_front() {
            let new_cost = current_cost + 1;
            for button in buttons {
                let mut next_state = current_state.clone();

                for &b_val in button {
                    if let Some(pos) = next_state.iter().position(|&x| x == b_val) {
                        next_state.swap_remove(pos);
                    } else {
                        next_state.push(b_val);
                    }
                }
                next_state.sort_unstable();

                if &next_state == goal_state {
                    presses_needed += new_cost;
                    found_goal = true;
                    break;
                }

                if visited.insert(next_state.clone()) {
                    queue.push_back((next_state, new_cost));
                }
            }
            if found_goal {
                break;
            }
        }
    }

    println!("Task 1 - Total presses needed: {}", presses_needed);
}

fn task_two(machines: &[Machine]) {
    let mut total_presses: u64 = 0;

    for machine in machines {
        if machine.joltage_requirements.iter().sum::<u32>() == 0 {
            continue;
        }

        if let Some(presses) = solve_machine_z3(machine) {
            total_presses += presses;
        }
    }

    println!("Task 2 - Total presses needed: {}", total_presses);
}

fn solve_machine_z3(machine: &Machine) -> Option<u64> {
    let opt = Optimize::new();

    let mut button_vars: Vec<Int> = Vec::new();
    for i in 0..machine.buttons.len() {
        let var = Int::new_const(format!("btn_{}", i));

        opt.assert(&var.ge(&Int::from_i64(0)));
        button_vars.push(var);
    }

    let offset = 0;

    let dim = machine.joltage_requirements.len();

    for d in 0..dim {
        let mut sum_expr = Int::from_i64(0);

        for (btn_idx, indices) in machine.buttons.iter().enumerate() {
            if indices.contains(&d) {
                sum_expr = sum_expr + &button_vars[btn_idx];
            }
        }

        let target_val = machine.joltage_requirements[d] as u64 + offset;
        let target_expr = Int::from_u64(target_val);

        opt.assert(&sum_expr.eq(&target_expr));
    }

    let mut total_cost = Int::from_i64(0);
    for var in &button_vars {
        total_cost = total_cost + var;
    }
    opt.minimize(&total_cost);

    if opt.check(&[]) == SatResult::Sat {
        let model = opt.get_model().unwrap();
        let result = model.eval(&total_cost, true).unwrap();
        return Some(result.as_u64().unwrap());
    }

    None
}
