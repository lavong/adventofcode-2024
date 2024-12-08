use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day08.txt")?;
    let map = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] != '.' {
                antennas.entry(map[y][x]).or_default().push((y, x))
            }
        }
    }

    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            for antenna in antennas.values() {
                for (&(y1, x1), &(y2, x2)) in antenna.iter().tuple_combinations() {
                    if (y1, x1) == (y2, x2) {
                        continue;
                    }
                    let dy1 = row.wrapping_sub(y1) as i32;
                    let dy2 = row.wrapping_sub(y2) as i32;
                    let dx1 = col.wrapping_sub(x1) as i32;
                    let dx2 = col.wrapping_sub(x2) as i32;
                    if dy1 * dx2 == dx1 * dy2 {
                        antinodes.insert((row, col));
                    }
                }
            }
        }
    }

    println!("solution part 2: {}", antinodes.len());
    Ok(())
}
