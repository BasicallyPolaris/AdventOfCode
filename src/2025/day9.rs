use std::env;
use std::fs;

const INPUT_BASE: &str = "src/2025/input/";
const CURR_DAY: u8 = 9;

type Coordinate = (u32, u32);

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
                [x, y] => (*x, *y),
                _ => panic!("Line did not contain exactly 2 numbers"),
            }
        })
        .collect();

    task_one(&coordinates);
    task_two(&coordinates);

    println!();
}

#[derive(Debug)]
struct DistanceSquare {
    c1: Coordinate,
    c2: Coordinate,
    area: u64,
}

fn task_one(coordinates: &[Coordinate]) {
    let mut distance_squares: Vec<DistanceSquare> = vec![];

    for (i, &c1) in coordinates.iter().enumerate() {
        for &c2 in &coordinates[i + 1..] {
            distance_squares.push(DistanceSquare {
                c1: c1,
                c2: c2,
                area: (c1.0.abs_diff(c2.0) as u64 + 1) * (c1.1.abs_diff(c2.1) as u64 + 1),
            });
        }
    }

    distance_squares.sort_by(|a, b| b.area.partial_cmp(&a.area).unwrap());

    let biggest_square = distance_squares
        .first()
        .expect("Need to have at least one rectangle.");
    println!(
        "Task 1 - The largest square is {} with tiles {:?}, {:?}",
        biggest_square.area, biggest_square.c1, biggest_square.c2
    )
}

fn task_two(coordinates: &[Coordinate]) {
    let mut distance_squares: Vec<DistanceSquare> = vec![];
    let mut edges: Vec<(Coordinate, Coordinate)> = vec![];

    let coordinate_count = coordinates.len();
    for i in 0..coordinate_count {
        edges.push((coordinates[i], coordinates[(i + 1) % coordinate_count]));
    }

    for (i, &c1) in coordinates.iter().enumerate() {
        for &c2 in &coordinates[i + 1..] {
            let x_min = std::cmp::min(c1.0, c2.0);
            let x_max = std::cmp::max(c1.0, c2.0);
            let y_min = std::cmp::min(c1.1, c2.1);
            let y_max = std::cmp::max(c1.1, c2.1);

            let top = ((x_min, y_max), (x_max, y_max));
            let bottom = ((x_min, y_min), (x_max, y_min));
            let left = ((x_min, y_min), (x_min, y_max));
            let right = ((x_max, y_min), (x_max, y_max));
            let square_sides = [top, bottom, left, right];

            let mut valid = true;

            // Check if boundaries cross
            for &(edge_p1, edge_p2) in edges.iter() {
                for side in square_sides {
                    if lines_intersect_strict(edge_p1, edge_p2, side.0, side.1) {
                        valid = false;
                        break;
                    }
                }
                if !valid {
                    break;
                }
            }
            if !valid {
                continue;
            }

            // Check that no node is within the rectangle itself
            for &coord in coordinates {
                if is_point_in_rect(coord, (x_min, y_min), (x_max, y_max)) {
                    valid = false;
                    break;
                }
            }
            if !valid {
                continue;
            }

            // Check that the rectangle itself actually is within the polygon
            let center_x = x_min as f64 + (x_max - x_min) as f64 / 2.0;
            let center_y = y_min as f64 + (y_max - y_min) as f64 / 2.0;

            if !is_point_in_polygon(center_x, center_y, &edges) {
                valid = false;
            }

            if valid {
                distance_squares.push(DistanceSquare {
                    c1,
                    c2,
                    area: (c1.0.abs_diff(c2.0) as u64 + 1) * (c1.1.abs_diff(c2.1) as u64 + 1),
                });
            }
        }
    }

    distance_squares.sort_by(|a, b| b.area.partial_cmp(&a.area).unwrap());

    if let Some(biggest_square) = distance_squares.first() {
        println!(
            "Task 2 - The largest square is {} with tiles {:?}, {:?}",
            biggest_square.area, biggest_square.c1, biggest_square.c2
        )
    } else {
        println!("Task 2 - No valid square found");
    }
}

fn is_point_in_polygon(x: f64, y: f64, edges: &Vec<(Coordinate, Coordinate)>) -> bool {
    let mut inside = false;
    for i in 0..edges.len() {
        let ((xi, yi), (xj, yj)) = (
            (edges[i].0 .0 as f64, edges[i].0 .1 as f64),
            (edges[i].1 .0 as f64, edges[i].1 .1 as f64),
        );

        let intersect = ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi);
        if intersect {
            inside = !inside;
        }
    }
    inside
}

// Check if a point is strictly inside a rectangle
fn is_point_in_rect(p: Coordinate, r_min: Coordinate, r_max: Coordinate) -> bool {
    p.0 > r_min.0 && p.0 < r_max.0 && p.1 > r_min.1 && p.1 < r_max.1
}

fn orientation(p: (u32, u32), q: (u32, u32), r: (u32, u32)) -> i8 {
    let val = (q.1 as i64 - p.1 as i64) * (r.0 as i64 - q.0 as i64)
        - (q.0 as i64 - p.0 as i64) * (r.1 as i64 - q.1 as i64);
    if val == 0 {
        return 0;
    }
    if val > 0 {
        return 1;
    }
    2
}

fn lines_intersect_strict(p1: (u32, u32), p2: (u32, u32), p3: (u32, u32), p4: (u32, u32)) -> bool {
    let o1 = orientation(p1, p2, p3);
    let o2 = orientation(p1, p2, p4);
    let o3 = orientation(p3, p4, p1);
    let o4 = orientation(p3, p4, p2);
    if o1 != o2 && o3 != o4 {
        // Need to be true intersections, not just touches
        if o1 == 0 || o2 == 0 || o3 == 0 || o4 == 0 {
            return false;
        }
        return true;
    }
    false
}
