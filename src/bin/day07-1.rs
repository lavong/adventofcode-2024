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

    println!("solution part 1: {calibration_result}");
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
    let n = nums.last().unwrap();
    return check(target / n, nums.clone().drain(..(nums.len() - 1)).collect())
        || check(target - n, nums.clone().drain(..(nums.len() - 1)).collect());
}
