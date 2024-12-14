use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day12.txt")?;
    let garden = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    let mut total_price = 0;
    for y in 0..garden.len() {
        for x in 0..garden[y].len() {
            if seen.contains(&(y as i32, x as i32)) {
                continue;
            }
            let region = find_region(&garden, &mut seen, y as i32, x as i32);
            let perimeter: u32 = region.iter().map(|(n, m)| perimeter(&garden, *n, *m)).sum();
            total_price += region.len() as u32 * perimeter;
        }
    }

    println!("solution part 1: {total_price}");
    Ok(())
}

fn perimeter(garden: &Vec<Vec<char>>, y: i32, x: i32) -> u32 {
    let mut adjacent_region_count = 0;
    let plant = char_at(garden, y, x);
    for (n, m) in adjacent_coords() {
        if char_at(garden, y + n, x + m) != plant {
            adjacent_region_count += 1
        }
    }
    adjacent_region_count
}

fn find_region(
    garden: &Vec<Vec<char>>,
    seen: &mut HashSet<(i32, i32)>,
    y: i32,
    x: i32,
) -> HashSet<(i32, i32)> {
    let mut queue = VecDeque::from([(y, x)]);
    let mut region = HashSet::from([(y, x)]);
    let plant = char_at(garden, y, x);

    while let Some((y1, x1)) = queue.pop_front() {
        for (dy, dx) in adjacent_coords() {
            let y2 = y1 + dy;
            let x2 = x1 + dx;
            let plant2 = char_at(garden, y2, x2);
            if plant == plant2 && seen.insert((y2, x2)) {
                queue.push_back((y2, x2));
                region.insert((y2, x2));
            }
        }
    }
    region
}

fn adjacent_coords() -> [(i32, i32); 4] {
    [(-1, 0), (0, 1), (1, 0), (0, -1)]
}

fn char_at(chars: &Vec<Vec<char>>, y: i32, x: i32) -> char {
    *chars
        .get(y as usize)
        .and_then(|c| c.get(x as usize))
        .unwrap_or(&' ')
}
