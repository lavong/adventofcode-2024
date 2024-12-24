use std::collections::HashMap;
use std::fs;
use std::io;

use itertools::Itertools;
use rand::seq::SliceRandom;

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

    let highest_degree = graph.values().map(|v| v.len()).max().unwrap();
    let mut max_clique = Vec::new();
    while max_clique.len() < highest_degree {
        let mut ks = graph.keys().collect_vec();
        ks.shuffle(&mut rand::thread_rng());

        let mut clique: Vec<&str> = Vec::new();
        for k in ks {
            if clique.iter().all(|c| graph[c].contains(k)) {
                let _ = &clique.push(k);
            }
        }

        if clique.len() > max_clique.len() {
            max_clique = clique;
        }
    }
    let password = max_clique.iter().sorted().join(",");

    println!("solution part 2: {password}");
    Ok(())
}
