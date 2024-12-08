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

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for a in antennas.values() {
        for (y1, x1) in a.iter() {
            for (y2, x2) in a.iter() {
                if (y1, x1) == (y2, x2) {
                    continue;
                }
                let y = (y2 + y2).wrapping_sub(*y1) as i32;
                let x = (x2 + x2).wrapping_sub(*x1) as i32;
                if y >= 0 && y < map.len() as i32 && x >= 0 && x < map[0].len() as i32 {
                    antinodes.insert((y, x));
                }
            }
        }
    }

    println!("solution part 1: {}", antinodes.len());
    Ok(())
}
