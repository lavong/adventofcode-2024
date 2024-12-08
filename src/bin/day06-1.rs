use std::fs;
use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day06.txt")?;
    let map = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    let visited = simulate(&map).len();

    println!("solution part 1: {visited}");
    Ok(())
}

fn simulate(map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut dir = 0;
    let (mut y, mut x) = find_guard(map);
    let mut seen = vec![vec![[false; 4]; map[0].len()]; map.len()];
    loop {
        seen[y as usize][x as usize][dir] = true;
        let (dy, dx) = dirs[dir];
        let next = char_at(map, y + dy, x + dx);
        match next {
            ' ' => {
                return (0..map.len())
                    .cartesian_product(0..map[0].len())
                    .filter(|&(y, x)| seen[y][x].iter().any(|&dir_seen| dir_seen))
                    .collect();
            }
            '#' => dir = (dir + 1) % 4,
            _ => {
                y += dy;
                x += dx;
            }
        }
    }
}

fn find_guard(map: &Vec<Vec<char>>) -> (i32, i32) {
    let mut pos: (i32, i32) = (0, 0);
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '^' {
                pos = (y as i32, x as i32)
            }
        }
    }
    pos
}

fn char_at(chars: &Vec<Vec<char>>, y: i32, x: i32) -> char {
    *chars
        .get(y as usize)
        .and_then(|c| c.get(x as usize))
        .unwrap_or(&' ')
}