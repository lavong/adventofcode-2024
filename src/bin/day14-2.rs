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

    let mut seconds_til_robots_form_a_christmas_tree = 0;
    for t in 1.. {
        advance(&mut robots, X, Y);

        if robots.iter().map(|(x, y, _, _)| (y, x)).all_unique() {
            seconds_til_robots_form_a_christmas_tree = t;
            break;
        }
    }

    print_robot_positions(&robots, X, Y);

    println!("solution part 2: {seconds_til_robots_form_a_christmas_tree}");
    Ok(())
}

fn advance(robots: &mut Vec<(i32, i32, i32, i32)>, width: i32, height: i32) {
    for (x, y, dx, dy) in robots {
        *x = (*x + *dx).rem_euclid(width);
        *y = (*y + *dy).rem_euclid(height);
    }
}

fn print_robot_positions(robots: &Vec<(i32, i32, i32, i32)>, width: i32, height: i32) {
    let robot_positions = robots.iter().map(|(y, x, _, _)| (y, x)).collect_vec();
    for x in 0..width {
        for y in 0..height {
            if robot_positions.contains(&(&y, &x)) {
                print!("#")
            } else {
                print!(" ")
            }
        }
        println!()
    }
}
