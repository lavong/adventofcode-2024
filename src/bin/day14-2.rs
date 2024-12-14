use std::fs;
use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day14.txt")?;

    let mut robots: Vec<(i64, i64, i64, i64)> = input
        .split(|c: char| !c.is_ascii_digit() && c != '-')
        .filter_map(|w| w.parse::<i64>().ok())
        .tuples()
        .collect_vec();

    let mut seconds_til_robots_form_a_christmas_tree = 0;
    for t in 1.. {
        for (y, x, dy, dx) in &mut robots {
            *y = (*y + *dy).rem_euclid(101);
            *x = (*x + *dx).rem_euclid(103);
        }
        if robots.iter().map(|(y, x, _, _)| (y, x)).all_unique() {
            seconds_til_robots_form_a_christmas_tree = t;
            break;
        }
    }

    let robot_positions = robots.iter().map(|(y, x, _, _)| (y, x)).collect_vec();
    for x in 0..101 {
        for y in 0..103 {
            if robot_positions.contains(&(&y, &x)) {
                print!("#")
            } else {
                print!(" ")
            }
        }
        println!()
    }

    println!("solution part 2: {seconds_til_robots_form_a_christmas_tree}");
    Ok(())
}
