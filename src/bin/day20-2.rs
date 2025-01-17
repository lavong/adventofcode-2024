use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day20.txt")?;
    let map = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    let start = find_char(&map, 'S');
    let end = find_char(&map, 'E');

    let mut dists_from_start = HashMap::new();
    let regular_cost = find_shortest_path(&map, &mut dists_from_start, start, end);

    let mut dists_from_end = HashMap::new();
    let _ = find_shortest_path(&map, &mut dists_from_end, end, start);

    let mut cheats_saving_100_ps = 0;
    for (y, x) in dists_from_start.keys() {
        for (y2, x2) in dists_from_end.keys() {
            let dist_cheat = y.abs_diff(*y2) + x.abs_diff(*x2);
            if dist_cheat <= 20 {
                let dist = dists_from_start[&(*y, *x)] + dist_cheat + dists_from_end[&(*y2, *x2)];
                if dist <= regular_cost - 100 {
                    cheats_saving_100_ps += 1;
                }
            }
        }
    }

    println!("solution part 2: {cheats_saving_100_ps}");
    Ok(())
}

fn find_shortest_path(
    map: &Vec<Vec<char>>,
    dist: &mut HashMap<(i32, i32), u32>,
    start: (i32, i32),
    end: (i32, i32),
) -> u32 {
    let mut best = u32::MAX;
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut heap = BinaryHeap::from([S {
        y: start.0,
        x: start.1,
        cost: 0,
    }]);
    let mut seen = HashSet::from([]);
    while let Some(S { y, x, cost }) = heap.pop() {
        if !dist.contains_key(&(y, x)) {
            *dist.entry((y, x)).or_default() = cost;
        }
        if (y, x) == end && cost < best {
            best = cost
        }
        if seen.insert((y, x)) {
            for dir in 0..dirs.len() {
                let (dy, dx) = dirs[dir as usize];
                let (y2, x2) = (y + dy, x + dx);
                if char_at(map, y2, x2) != '#' {
                    heap.push(S {
                        y: y2,
                        x: x2,
                        cost: cost + 1,
                    });
                }
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
