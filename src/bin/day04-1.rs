use std::fs;
use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day04.txt")?;

    let chars = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let mut xmas_count = 0;
    for x in 0..chars.len() {
        for y in 0..chars[x].len() {
            xmas_count += count_xmas(&chars, x, y);
        }
    }

    println!("solution part 1: {xmas_count}");
    Ok(())
}

fn adjacent_coords() -> [(i32, i32); 8] {
    [
        (-1, -1),
        (0, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 0),
        (1, 1),
        (1, -1),
    ]
}

fn count_xmas(chars: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let mut count: usize = 0;
    for (dx, dy) in adjacent_coords() {
        let mut word: Vec<char> = vec!['.'; 4];
        for i in 0..4 {
            word[i as usize] = *chars
                .get(x.wrapping_add((dx * i) as usize))
                .and_then(|c| c.get(y.wrapping_add((dy * i) as usize)))
                .unwrap_or(&'.');
        }
        if word.iter().join("") == "XMAS" {
            count += 1
        }
    }
    count
}
