use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day10.txt")?;
    let map = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    let total_rating: usize = trailheads(&map)
        .iter()
        .map(|(y, x)| rating(&map, (*y, *x)))
        .sum();

    println!("solution part 2: {total_rating}");
    Ok(())
}

fn trailheads(map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut trailheads = Vec::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '0' {
                trailheads.push((y, x));
            }
        }
    }
    trailheads
}

fn adjacent_coords() -> [(i32, i32); 4] {
    [(-1, 0), (0, 1), (1, 0), (0, -1)]
}

fn rating(map: &Vec<Vec<char>>, head: (usize, usize)) -> usize {
    let mut score = 0;
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back(head);
    while let Some((y, x)) = queue.pop_front() {
        if seen.contains(&(y, x)) {
            continue;
        }
        seen.insert((y, x));
        if map[y][x] == '9' {
            return 1;
        }
        for (dy, dx) in adjacent_coords() {
            let y2 = y as i32 + dy;
            let x2 = x as i32 + dx;
            let n = char_at(map, y as i32, x as i32).to_digit(10).unwrap();
            let m = char_at(map, y2, x2).to_digit(10).unwrap();
            if n + 1 == m {
                score += rating(map, (y2 as usize, x2 as usize));
            }
        }
    }
    score
}

fn char_at(chars: &Vec<Vec<char>>, y: i32, x: i32) -> char {
    *chars
        .get(y as usize)
        .and_then(|c| c.get(x as usize))
        .unwrap_or(&'0')
}
