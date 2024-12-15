use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day15.txt")?;
    let (input_map, input_moves) = input.split_once("\n\n").unwrap();

    let mut scaled_up_warehouse = input_map
        .lines()
        .map(|l| {
            l.chars()
                .flat_map(|c| match c {
                    '#' => vec!['#', '#'],
                    'O' => vec!['[', ']'],
                    '.' => vec!['.', '.'],
                    '@' => vec!['@', '.'],
                    _ => vec![c],
                })
                .collect_vec()
        })
        .collect_vec();
    let moves = input_moves.chars().filter(|c| *c != '\n').collect_vec();

    play(&mut scaled_up_warehouse, &moves);

    let mut gps_coords_sum = 0;
    for y in 0..scaled_up_warehouse.len() {
        for x in 0..scaled_up_warehouse[y].len() {
            if scaled_up_warehouse[y][x] == '[' {
                gps_coords_sum += 100 * y + x
            }
        }
    }

    println!("solution part 2: {gps_coords_sum}");
    Ok(())
}

fn play(map: &mut Vec<Vec<char>>, moves: &Vec<char>) {
    for c in moves {
        let (y, x) = find_robot(&map);
        let (dy, dx) = match c {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            _ => (0, 0),
        };
        attempt_multipush(&mut *map, y, x, dy, dx);
    }
}

fn attempt_multipush(map: &mut Vec<Vec<char>>, y: i32, x: i32, dy: i32, dx: i32) {
    match char_at(map, y + dy, x + dx) {
        '.' => swap_char(map, y, x, y + dy, x + dx),
        '[' | ']' => {
            let mut queue = VecDeque::from([(y, x)]);
            let mut seen: HashSet<(i32, i32)> = HashSet::from([]);
            while let Some((y, x)) = queue.pop_front() {
                if seen.insert((y, x)) {
                    let y2 = y + dy;
                    let x2 = x + dx;
                    match char_at(map, y2, x2) {
                        '#' => return,
                        '[' => queue.extend([(y2, x2), (y2, x2 + 1)]),
                        ']' => queue.extend([(y2, x2), (y2, x2 - 1)]),
                        _ => continue,
                    }
                }
            }
            let seen_sorted = match (dy, dx) {
                (-1, 0) => seen.iter().sorted_by(|a, b| Ord::cmp(&a.0, &b.0)),
                (0, 1) => seen.iter().sorted_by(|a, b| Ord::cmp(&b.1, &a.1)),
                (1, 0) => seen.iter().sorted_by(|a, b| Ord::cmp(&b.0, &a.0)),
                _ => seen.iter().sorted_by(|a, b| Ord::cmp(&a.1, &b.1)),
            };
            for (y, x) in seen_sorted {
                swap_char(map, y + dy, x + dx, *y, *x);
            }
        }
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
