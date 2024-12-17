use std::fs;
use std::io;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day17.txt")?;
    let (input_registers, input_program) = input.split_once("\n\n").unwrap();

    let (a, mut b, mut c) = input_registers
        .split(|c: char| !c.is_ascii_digit())
        .filter_map(|w| w.parse::<u64>().ok())
        .collect_tuple()
        .unwrap();
    let program = input_program[9..]
        .split(',')
        .filter_map(|n| n.trim().parse::<u64>().ok())
        .collect_vec();

    // brute force no good. we can haz clever solution? ¯\_(ツ)_/¯
    let arbitrary_offset: u64 = a + 47910000000000;
    let num_threads = 8;
    let solution = Arc::new(AtomicU64::new(0));
    let mut handles = vec![];
    for t in 1..=num_threads {
        let s = Arc::clone(&solution);
        let p = program.clone();
        let h = thread::Builder::new()
            .name(t.to_string())
            .spawn(move || {
                let mut i = arbitrary_offset + t;
                loop {
                    // if i % (1000000 + t) == t {
                    //     println!("t{}: {}", thread::current().name().unwrap(), i);
                    // }
                    let mut candidate = i;
                    let out = run(&mut candidate, &mut b, &mut c, &p);
                    if &p == &out {
                        let _ = &s.fetch_add(i, Ordering::Relaxed);
                        break;
                    }
                    i += num_threads;
                    if s.load(Ordering::Relaxed) > 0 {
                        break;
                    }
                }
                s.load(Ordering::Relaxed)
            })
            .unwrap();
        handles.push(h);
    }

    let a = handles
        .into_iter()
        .filter_map(|h| h.join().ok())
        .next()
        .unwrap();

    println!("solution part 2: {a}");
    Ok(())
}

fn run(a: &mut u64, b: &mut u64, c: &mut u64, program: &Vec<u64>) -> Vec<u64> {
    let mut ptr: u64 = 0;
    let mut out = vec![];
    loop {
        if ptr >= program.len() as u64 {
            break;
        }
        let opcode = program[ptr as usize];
        let operand = program[ptr as usize + 1];
        let combo = match operand {
            4 => *a,
            5 => *b,
            6 => *c,
            _ => operand,
        };
        ptr += 2;
        match opcode {
            0 => *a = *a / 2u64.pow(combo as u32),
            1 => *b ^= operand,
            2 => *b = combo % 8,
            3 if *a != 0 => ptr = operand,
            4 => *b ^= *c,
            5 => {
                let v = combo % 8;
                if v != *program.get(out.len()).unwrap_or(&8) {
                    return vec![];
                }
                out.push(v);
            }
            6 => *b = *a / 2u64.pow(combo as u32),
            7 => *c = *a / 2u64.pow(combo as u32),
            _ => {}
        }
    }
    out
}
