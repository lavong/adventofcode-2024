use std::fs;
use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day15.txt")?;
    let (input_map, input_moves) = input.split_once("\n\n").unwrap();

    let mut warehouse = input_map
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();
    let moves = input_moves.chars().filter(|c| *c != '\n').collect_vec();

    simulate(&mut warehouse, &moves);

    let mut gps_coords_sum = 0;
    for y in 0..warehouse.len() {
        for x in 0..warehouse[y].len() {
            if warehouse[y][x] == 'O' {
                gps_coords_sum += 100 * y + x
            }
        }
    }

    println!("solution part 1: {gps_coords_sum}");
    Ok(())
}

fn simulate(map: &mut Vec<Vec<char>>, moves: &Vec<char>) {
    for c in moves {
        let (y, x) = find_robot(&map);
        let (dy, dx) = match c {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            _ => (0, 0),
        };
        attempt_move(&mut *map, y, x, dy, dx);
    }
}

fn attempt_move(map: &mut Vec<Vec<char>>, y: i32, x: i32, dy: i32, dx: i32) {
    let mut y2 = y + dy;
    let mut x2 = x + dx;
    match char_at(map, y2, x2) {
        '.' => swap_char(map, y, x, y2, x2),
        'O' => loop {
            y2 += dy;
            x2 += dx;
            match char_at(map, y2, x2) {
                'O' => continue,
                '.' => {
                    while (y2, x2) != (y, x) {
                        swap_char(map, y2, x2, y2 - dy, x2 - dx);
                        y2 -= dy;
                        x2 -= dx;
                    }
                    return;
                }
                _ => break,
            }
        },
        _ => {}
    }
}

fn find_robot(map: &Vec<Vec<char>>) -> (i32, i32) {
    let mut pos: (i32, i32) = (0, 0);
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '@' {
                pos = (y as i32, x as i32)
            }
        }
    }
    pos
}

fn swap_char(map: &mut Vec<Vec<char>>, y: i32, x: i32, y2: i32, x2: i32) {
    let c = char_at(map, y, x);
    let c2 = char_at(map, y2, x2);
    map[y as usize][x as usize] = c2;
    map[y2 as usize][x2 as usize] = c;
}

fn char_at(chars: &Vec<Vec<char>>, y: i32, x: i32) -> char {
    *chars
        .get(y as usize)
        .and_then(|c| c.get(x as usize))
        .unwrap_or(&' ')
}
