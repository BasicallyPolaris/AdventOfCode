use std::collections::HashMap;
use std::env;
use std::fs;

const INPUT_BASE: &str = "src/2025/input/";
const CURR_DAY: u8 = 8;
type Coordinate = (u32, u32, u32);

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

    let coordinates: Vec<Coordinate> = file_content
        .lines()
        .map(|l| {
            let nums: Vec<u32> = l
                .split(',')
                .map(|n| n.trim().parse().expect("Parse error"))
                .collect();

            match nums.as_slice() {
                [x, y, z] => (*x, *y, *z),
                _ => panic!("Line did not contain exactly 3 numbers"),
            }
        })
        .collect();

    task_one(&coordinates);
    task_two(&coordinates);

    println!();
}

fn euclid_distance(p1: Coordinate, p2: Coordinate) -> f64 {
    let (x1, y1, z1) = p1;
    let (x2, y2, z2) = p2;

    let dx = x2 as f64 - x1 as f64;
    let dy = y2 as f64 - y1 as f64;
    let dz = z2 as f64 - z1 as f64;

    (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt()
}

fn task_one(coordinates: &[Coordinate]) {
    let mut distances_vector = Vec::new();

    for (i, &p1) in coordinates.iter().enumerate() {
        for &p2 in &coordinates[i + 1..] {
            let dist = euclid_distance(p1, p2);
            distances_vector.push((p1, p2, dist));
        }
    }

    distances_vector.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let mut groupings: HashMap<Coordinate, u32> = HashMap::new();
    let mut next_group_id = 1;
    let coordinate_count = coordinates.len();

    for (c1, c2, _) in distances_vector.into_iter().take(coordinate_count) {
        let g1 = groupings.get(&c1).copied();
        let g2 = groupings.get(&c2).copied();
        match (g1, g2) {
            // CASE A: Neither point is in a group yet.
            (None, None) => {
                groupings.insert(c1, next_group_id);
                groupings.insert(c2, next_group_id);
                next_group_id += 1;
            }

            // CASE B: One point is in a group, the other is not.
            (Some(id), None) => {
                groupings.insert(c2, id);
            }
            (None, Some(id)) => {
                groupings.insert(c1, id);
            }

            // CASE C: Both are already in the SAME group.
            (Some(id1), Some(id2)) if id1 == id2 => {
                continue;
            }

            // CASE D: They are in DIFFERENT groups - merge them
            (Some(id1), Some(id2)) => {
                for value in groupings.values_mut() {
                    if *value == id2 {
                        *value = id1;
                    }
                }
            }
        }
    }

    let mut group_sizes: HashMap<u32, usize> = HashMap::new();

    for &group_id in groupings.values() {
        *group_sizes.entry(group_id).or_default() += 1;
    }

    let mut sizes: Vec<usize> = group_sizes.into_values().collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    let top_3_mul: usize = sizes.iter().take(3).copied().fold(1, |acc, x| acc * x);
    println!(
        "Task 1 - The result of the three largest groupings: {}",
        top_3_mul
    );
}

fn task_two(coordinates: &[Coordinate]) {
    let mut distances_vector = Vec::new();

    for (i, &p1) in coordinates.iter().enumerate() {
        for &p2 in &coordinates[i + 1..] {
            let dist = euclid_distance(p1, p2);
            distances_vector.push((p1, p2, dist));
        }
    }

    distances_vector.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let mut groupings: HashMap<Coordinate, u32> = HashMap::new();
    let mut next_group_id = 1;

    let mut last_connected: Option<(Coordinate, Coordinate)> = None;

    for (c1, c2, _) in distances_vector.into_iter() {
        let g1 = groupings.get(&c1).copied();
        let g2 = groupings.get(&c2).copied();
        match (g1, g2) {
            // CASE A: Neither point is in a group yet.
            (None, None) => {
                groupings.insert(c1, next_group_id);
                groupings.insert(c2, next_group_id);
                next_group_id += 1;
            }

            // CASE B: One point is in a group, the other is not.
            (Some(id), None) => {
                groupings.insert(c2, id);
            }
            (None, Some(id)) => {
                groupings.insert(c1, id);
            }

            // CASE C: Both are already in the SAME group.
            (Some(id1), Some(id2)) if id1 == id2 => {
                continue;
            }

            // CASE D: They are in DIFFERENT groups - merge them
            (Some(id1), Some(id2)) => {
                for value in groupings.values_mut() {
                    if *value == id2 {
                        *value = id1;
                    }
                }

                last_connected = Some((c1, c2));
            }
        }
    }

    let wall_distance: u64 = if let Some((c1, c2)) = last_connected.take() {
        c1.0 as u64 * c2.0 as u64
    } else {
        panic!("Didn't find last match.");
    };

    println!("Task 2 - The distance from the wall is: {}", wall_distance);
}
