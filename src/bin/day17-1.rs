use std::fs;
use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day17.txt")?;
    let (input_registers, input_program) = input.split_once("\n\n").unwrap();

    let (mut a, mut b, mut c) = input_registers
        .split(|c: char| !c.is_ascii_digit())
        .filter_map(|w| w.parse::<u32>().ok())
        .collect_tuple()
        .unwrap();
    let program = input_program[9..]
        .split(',')
        .filter_map(|n| n.trim().parse::<u32>().ok())
        .collect_vec();

    let mut ptr: u32 = 0;
    let mut out = vec![];
    loop {
        if ptr >= program.len() as u32 {
            break;
        }
        let opcode = program[ptr as usize];
        let operand = program[ptr as usize + 1];
        let combo = match operand {
            4 => a,
            5 => b,
            6 => c,
            _ => operand,
        };
        match opcode {
            0 => a = a / 2u32.pow(combo),
            1 => b ^= operand,
            2 => b = combo % 8,
            4 => b ^= c,
            5 => out.push(combo % 8),
            6 => b = a / 2u32.pow(combo),
            7 => c = a / 2u32.pow(combo),
            _ => {}
        }
        if a != 0 && opcode == 3 {
            ptr = operand
        } else {
            ptr += 2
        }
    }
    let output = out.iter().join(",");

    println!("solution part 1: {output}");
    Ok(())
}
