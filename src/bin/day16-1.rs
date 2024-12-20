use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::u32;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day16.txt")?;
    let map = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    let start = find_char(&map, 'S');
    let end = find_char(&map, 'E');

    let cost = find_shortest_path(&map, start, end);

    println!("solution part 1: {cost}");
    Ok(())
}

fn find_shortest_path(map: &Vec<Vec<char>>, start: (i32, i32), end: (i32, i32)) -> u32 {
    let mut best = u32::MAX;
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut heap = BinaryHeap::from([S {
        y: start.0,
        x: start.1,
        dir: 1,
        cost: 0,
    }]);
    let mut seen = HashSet::from([]);
    let mut dist = HashMap::new();
    while let Some(S { y, x, dir, cost }) = heap.pop() {
        if !dist.contains_key(&(y, x, dir)) {
            *dist.entry((y, x, dir)).or_default() = cost;
        }
        if (y, x) == end && cost < best {
            best = cost
        }
        if seen.insert((y, x, dir)) {
            let (dy, dx) = dirs[dir as usize];
            let (y2, x2) = (y + dy, x + dx);
            if y2 >= 0 && y2 < map.len() as i32 && x2 >= 0 && x2 < map[0].len() as i32 {
                if char_at(map, y2, x2) != '#' {
                    heap.push(S {
                        y: y2,
                        x: x2,
                        dir: dir,
                        cost: cost + 1,
                    });
                }
                heap.push(S {
                    y: y,
                    x: x,
                    dir: (dir + 1) % 4,
                    cost: cost + 1000,
                });
                heap.push(S {
                    y: y,
                    x: x,
                    dir: (dir + 3) % 4,
                    cost: cost + 1000,
                });
            }
        }
    }
    best
}

fn find_char(map: &Vec<Vec<char>>, c: char) -> (i32, i32) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == c {
                return (y as i32, x as i32);
            }
        }
    }
    (0, 0)
}

fn char_at(chars: &Vec<Vec<char>>, y: i32, x: i32) -> char {
    *chars
        .get(y as usize)
        .and_then(|c| c.get(x as usize))
        .unwrap_or(&'#')
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct S {
    y: i32,
    x: i32,
    dir: i32,
    cost: u32,
}

// implement trait so that priority queue becomes min-heap
impl Ord for S {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.cost.cmp(&self.cost))
            .then_with(|| self.y.cmp(&other.y))
            .then_with(|| self.x.cmp(&other.x))
    }
}

impl PartialOrd for S {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
