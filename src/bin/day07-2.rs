use std::fs;
use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day07.txt")?;
    let equations = parse(&input);

    let calibration_result: f64 = equations
        .iter()
        .filter(|(v, nums)| check(*v, nums.clone()))
        .map(|eq| eq.0)
        .sum();

    println!("solution part 2: {calibration_result}");
    Ok(())
}

fn parse(input: &String) -> Vec<(f64, Vec<f64>)> {
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

fn check(target: f64, nums: Vec<f64>) -> bool {
    if nums.len() == 1 {
        return target == nums[0];
    }
    let n = nums[0];
    let m = nums[1];
    let mut nm = nums.clone().drain(2..).collect_vec();
    nm.insert(0, n * m);
    let mut na = nums.clone().drain(2..).collect_vec();
    na.insert(0, n + m);
    let mut nc = nums.clone().drain(2..).collect_vec();
    nc.insert(0, format!("{}{}", n, m).parse().unwrap());
    return check(target, nm) || check(target, na) || check(target, nc);
}
