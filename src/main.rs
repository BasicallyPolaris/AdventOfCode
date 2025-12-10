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
    y2025::day6::run();
    y2025::day7::run();
    y2025::day8::run();
    y2025::day9::run();
    y2025::day10::run();
}

fn run_today() {
    y2025::day10::run();
}

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

fn test_point() -> io::Result<()> {
    // 1. Define the Rectangle
    // We normalize coordinates so we have explicit min/max bounds
    let p1 = Point { x: 84201, y: 15908 };
    let p2 = Point { x: 16590, y: 83979 };

    let rect_x_min = p1.x.min(p2.x);
    let rect_x_max = p1.x.max(p2.x);
    let rect_y_min = p1.y.min(p2.y);
    let rect_y_max = p1.y.max(p2.y);

    println!(
        "Checking Rectangle: X[{}..{}] Y[{}..{}]",
        rect_x_min, rect_x_max, rect_y_min, rect_y_max
    );

    // 2. Load the Polygon (Outline) from file
    // Replace "polygon.txt" with your actual file path
    let polygon = load_polygon("src/2025/input/day9.input")?;
    println!("Loaded polygon with {} vertices.", polygon.len());

    // 3. CHECK A: Wall Collision
    // Ensure no polygon vertex exists strictly inside the rectangle.
    // If a wall is inside, the rectangle hits an obstacle.
    for p in &polygon {
        if p.x > rect_x_min && p.x < rect_x_max && p.y > rect_y_min && p.y < rect_y_max {
            println!(
                "❌ FAILED: Polygon vertex ({}, {}) is inside the rectangle (Collision).",
                p.x, p.y
            );
            return Ok(());
        }
    }

    // 4. CHECK B: Containment (Ray Casting)
    // We must ensure the rectangle isn't floating in "void" space (the U-shape issue).
    // We check the Center and all 4 Corners.

    // Check Center
    let center = Point {
        x: (rect_x_min + rect_x_max) / 2,
        y: (rect_y_min + rect_y_max) / 2,
    };

    // Check Corners
    let corners = vec![
        Point {
            x: rect_x_min,
            y: rect_y_min,
        },
        Point {
            x: rect_x_max,
            y: rect_y_min,
        },
        Point {
            x: rect_x_max,
            y: rect_y_max,
        },
        Point {
            x: rect_x_min,
            y: rect_y_max,
        },
        center, // Check center as well to be safe against U-shapes bridging gaps
    ];

    for (i, pt) in corners.iter().enumerate() {
        if !is_point_in_polygon(*pt, &polygon) {
            let label = if i == 4 { "Center" } else { "Corner" };
            println!(
                "❌ FAILED: Rectangle {} ({}, {}) is outside the polygon shape.",
                label, pt.x, pt.y
            );
            return Ok(());
        }
    }

    println!("✅ SUCCESS: The rectangle is fully valid and inside the shape.");
    Ok(())
}

/// Ray Casting Algorithm
/// Casts a ray from `p` to the right (positive X).
/// Counts how many polygon edges it crosses.
/// Odd = Inside, Even = Outside.
fn is_point_in_polygon(p: Point, polygon: &[Point]) -> bool {
    let mut inside = false;
    let n = polygon.len();
    let mut j = n - 1; // The previous vertex

    for i in 0..n {
        let pi = polygon[i];
        let pj = polygon[j];

        // Check if the ray crosses the edge (pi, pj)
        // 1. The edge must straddle the point's Y coordinate
        // 2. The intersection point on the X axis must be to the right of p.x
        let intersects_y = (pi.y > p.y) != (pj.y > p.y);

        if intersects_y {
            // Calculate intersection X coordinate
            // Formula: x = x1 + (y - y1) * (x2 - x1) / (y2 - y1)
            // We use f64 for precision during division
            let intersect_x =
                pi.x as f64 + (p.y - pi.y) as f64 * (pj.x - pi.x) as f64 / (pj.y - pi.y) as f64;

            if (p.x as f64) < intersect_x {
                inside = !inside;
            }
        }
        j = i;
    }

    inside
}

fn load_polygon<P: AsRef<Path>>(filename: P) -> io::Result<Vec<Point>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut points = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.trim().split(',').collect();
        if parts.len() == 2 {
            let x = parts[0].parse::<i64>().unwrap_or(0);
            let y = parts[1].parse::<i64>().unwrap_or(0);
            points.push(Point { x, y });
        }
    }
    Ok(points)
}
