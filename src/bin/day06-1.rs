use std::fs;
use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day06.txt")?;
    let mut map = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    simlate_guard_movement(&mut map);

    let guard_steps: usize = map
        .iter()
        .map(|l| l.iter().filter(|c| **c == 'X').count())
        .sum();

    println!("solution part 1: {guard_steps}");
    Ok(())
}

fn simlate_guard_movement(map: &mut Vec<Vec<char>>) {
    loop {
        let (y, x, (dy, dx)) = find_guard(map);
        let cur_pos_char = char_at(map, y, x);
        let new_pos_char = char_at(map, y + dy, x + dx);
        match new_pos_char {
            ' ' => {
                map[y as usize][x as usize] = 'X';
                break;
            }
            '#' => match cur_pos_char {
                '^' => map[y as usize][x as usize] = '>',
                '>' => map[y as usize][x as usize] = 'v',
                'v' => map[y as usize][x as usize] = '<',
                '<' => map[y as usize][x as usize] = '^',
                _ => {}
            },
            _ => {
                map[y as usize][x as usize] = 'X';
                map[(y + dy) as usize][(x + dx) as usize] = cur_pos_char;
            }
        }
    }
}

fn find_guard(map: &Vec<Vec<char>>) -> (i32, i32, (i32, i32)) {
    let mut guard_pos: (i32, i32, (i32, i32)) = (0, 0, (-1, -1));
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            match map[y][x] {
                '^' => guard_pos = (y as i32, x as i32, (-1, 0)),
                '>' => guard_pos = (y as i32, x as i32, (0, 1)),
                'v' => guard_pos = (y as i32, x as i32, (1, 0)),
                '<' => guard_pos = (y as i32, x as i32, (0, -1)),
                _ => {}
            }
        }
    }
    guard_pos
}

fn char_at(chars: &Vec<Vec<char>>, y: i32, x: i32) -> char {
    *chars
        .get(y as usize)
        .and_then(|c| c.get(x as usize))
        .unwrap_or(&' ')
}
