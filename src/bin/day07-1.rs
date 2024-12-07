use std::fs;
use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day07.txt")?;
    let equations = parse(&input);

    let calibration_result: f64 = equations
        .iter()
        .filter(|(v, nums)| check((*v, nums.clone())))
        .map(|eq| eq.0)
        .sum();

    println!("solution part 1: {calibration_result}");
    Ok(())
}

type Equation = (f64, Vec<f64>);

fn parse(input: &String) -> Vec<Equation> {
    input
        .lines()
        .filter_map(|line| line.split_once(':'))
        .map(|(n, m)| {
            (
                n.parse::<f64>().unwrap(),
                m.trim()
                    .split_whitespace()
                    .map(|x| x.parse::<f64>().unwrap())
                    .collect_vec(),
            )
        })
        .collect_vec()
}

fn check(eq: Equation) -> bool {
    if eq.1.len() == 1 {
        return eq.0 == eq.1[0];
    }
    let n = eq.1.last().unwrap();
    return check((eq.0 / n, eq.1.clone().drain(..(eq.1.len() - 1)).collect()))
        || check((eq.0 - n, eq.1.clone().drain(..(eq.1.len() - 1)).collect()));
}
