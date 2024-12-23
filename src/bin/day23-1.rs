use std::collections::HashMap;
use std::fs;
use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day23.txt")?;

    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    input
        .lines()
        .map(|l| l.split_once('-').unwrap())
        .for_each(|(a, b)| {
            graph.entry(a).or_default().push(b);
            graph.entry(b).or_default().push(a);
        });

    let ks = graph.keys().sorted().collect_vec();
    let mut triangle_sets = 0;
    for (i, a) in ks.iter().enumerate() {
        for j in (i + 1)..ks.len() {
            for k in (j + 1)..ks.len() {
                let b = ks[j];
                let c = ks[k];
                if graph[b].contains(a)
                    && graph[c].contains(a)
                    && graph[c].contains(b)
                    && (a.starts_with('t') || b.starts_with('t') || c.starts_with('t'))
                {
                    triangle_sets += 1;
                }
            }
        }
    }

    println!("solution part 1: {triangle_sets}");
    Ok(())
}
