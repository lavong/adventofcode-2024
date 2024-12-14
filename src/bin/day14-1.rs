use std::fs;
use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day14.txt")?;

    const X: i32 = 101;
    const Y: i32 = 103;
    let mut robots: Vec<(i32, i32, i32, i32)> = input
        .split(|c: char| !c.is_ascii_digit() && c != '-')
        .filter_map(|w| w.parse::<i32>().ok())
        .tuples()
        .collect_vec();

    for _ in 1..=100 {
        advance(&mut robots, X, Y);
    }

    println!("solution part 1: {}", safety_factor(&robots, X, Y));
    Ok(())
}

fn advance(robots: &mut Vec<(i32, i32, i32, i32)>, width: i32, height: i32) {
    for (x, y, dx, dy) in robots {
        *x = (*x + *dx).rem_euclid(width);
        *y = (*y + *dy).rem_euclid(height);
    }
}

fn safety_factor(robots: &Vec<(i32, i32, i32, i32)>, width: i32, height: i32) -> i32 {
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for &(x, y, _, _) in robots {
        if x < width / 2 && y < height / 2 {
            q1 += 1;
        } else if x > width / 2 && y < height / 2 {
            q2 += 1;
        } else if x < width / 2 && y > height / 2 {
            q3 += 1;
        } else if x > width / 2 && y > height / 2 {
            q4 += 1;
        }
    }
    q1 * q2 * q3 * q4
}
