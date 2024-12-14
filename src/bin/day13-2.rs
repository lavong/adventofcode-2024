use std::fs;
use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day13.txt")?;

    let games = input
        .split(|c: char| !c.is_ascii_digit())
        .filter_map(|w| w.parse().ok())
        .tuples()
        .collect_vec();

    const PRIZE_OFFSET: i64 = 10000000000000;
    let min_total_cost: i64 = games
        .iter()
        .filter_map(|(x1, x2, y1, y2, p1, p2)| {
            cost(*x1, *x2, *y1, *y2, PRIZE_OFFSET + *p1, PRIZE_OFFSET + *p2)
        })
        .sum();

    println!("solution part 2: {min_total_cost}");
    Ok(())
}

fn cost(x1: i64, x2: i64, y1: i64, y2: i64, p1: i64, p2: i64) -> Option<i64> {
    let b = (p2 * x1 - p1 * x2) / (y2 * x1 - y1 * x2);
    let a = (p2 - y2 * b) / x2;
    if (x1 * a + y1 * b, x2 * a + y2 * b) == (p1, p2) {
        return Some(a * 3 + b);
    }
    return None;
}
