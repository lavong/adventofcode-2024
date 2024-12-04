use std::fs;
use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day04.txt")?;

    let chars = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let mut xmas_count = 0;
    for x in 0..chars.len() {
        for y in 0..chars[x].len() {
            if is_xmas_pattern(&chars, x as i32, y as i32) {
                xmas_count += 1
            }
        }
    }

    println!("solution part 2: {xmas_count}");
    Ok(())
}

fn is_xmas_pattern(chars: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    let c = char_at(chars, x, y);
    let tl = char_at(chars, x - 1, y - 1);
    let tr = char_at(chars, x + 1, y - 1);
    let bl = char_at(chars, x - 1, y + 1);
    let br = char_at(chars, x + 1, y + 1);

    [format!("{}{}{}", tl, c, br), format!("{}{}{}", bl, c, tr)]
        .iter()
        .all(|word| word == "SAM" || word == "MAS")
}

fn char_at(chars: &Vec<Vec<char>>, x: i32, y: i32) -> char {
    *chars
        .get(x as usize)
        .and_then(|c| c.get(y as usize))
        .unwrap_or(&'.')
}
