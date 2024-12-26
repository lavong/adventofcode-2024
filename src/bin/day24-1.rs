use std::collections::HashMap;
use std::fs;
use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day24.txt")?;
    let (mut wires, gates) = parse_input(&input);

    for (out, _) in &gates {
        dfs(&mut wires, &gates, out);
    }

    let num_bin = wires
        .iter()
        .filter(|(k, _v)| k.starts_with('z'))
        .sorted_by(|a, b| b.0.cmp(a.0))
        .map(|(_k, v)| v)
        .join("");
    let num_dec = u64::from_str_radix(num_bin.as_str(), 2).unwrap();

    println!("solution part 1: {num_dec}");
    Ok(())
}

fn parse_input(input: &String) -> (HashMap<&str, u32>, HashMap<&str, (&str, &str, &str)>) {
    let (in_wires, in_gates) = input.split_once("\n\n").unwrap();

    let wires: HashMap<&str, u32> = in_wires
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(w, c)| (w, c.parse().unwrap()))
        .collect();

    let gates = in_gates
        .lines()
        .map(|l| {
            let (s, out) = l.split_once(" -> ").unwrap();
            let (a, op, b) = s.split_whitespace().next_tuple().unwrap();
            (out, (a, op, b))
        })
        .collect();

    (wires, gates)
}

fn dfs<'a>(
    wires: &mut HashMap<&'a str, u32>,
    gates: &HashMap<&'a str, (&'a str, &str, &'a str)>,
    out: &'a str,
) -> u32 {
    if wires.contains_key(out) {
        return wires[out];
    }
    let (a, op, b) = gates[out];
    let left = dfs(wires, gates, a);
    let right = dfs(wires, gates, b);
    let v = match op {
        "AND" => left & right,
        "OR" => left | right,
        "XOR" => left ^ right,
        _ => 0,
    };
    wires.insert(out, v);
    v
}
