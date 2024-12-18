use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::u32;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day18.txt")?;
    let falling_bytes: Vec<(i32, i32)> = input
        .split(|c: char| !c.is_ascii_digit())
        .filter_map(|w| w.parse::<i32>().ok())
        .tuples()
        .collect_vec();

    const X: usize = 71;
    const I: usize = 1024;
    let mut map = vec![vec!['.'; X]; X];
    for (x, y) in falling_bytes.iter().take(I) {
        map[*y as usize][*x as usize] = '#';
    }

    const S: (i32, i32) = (0, 0);
    const E: (i32, i32) = (X as i32 - 1, X as i32 - 1);
    let mut c = (0, 0);
    for (x, y) in falling_bytes.iter().dropping(I) {
        map[*y as usize][*x as usize] = '#';
        if find_shortest_path(&map, S, E) == u32::MAX {
            c = (*x, *y);
            break;
        }
    }
    let blocking_byte_coord = format!("{},{}", c.0, c.1);

    println!("solution part 2: {blocking_byte_coord}");
    Ok(())
}

fn find_shortest_path(map: &Vec<Vec<char>>, start: (i32, i32), end: (i32, i32)) -> u32 {
    let mut best = u32::MAX;
    let mut heap = BinaryHeap::from([S {
        y: start.0,
        x: start.1,
        cost: 0,
    }]);
    let mut seen = HashSet::from([]);
    let mut dist = HashMap::new();
    while let Some(S { y, x, cost }) = heap.pop() {
        if !dist.contains_key(&(y, x)) {
            *dist.entry((y, x)).or_default() = cost;
        }
        if (y, x) == end && cost < best {
            best = cost
        }
        if seen.insert((y, x)) {
            for (dy, dx) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
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
