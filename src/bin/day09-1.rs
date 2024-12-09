use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day09.txt")?;

    let mut disk = parse_filesystem(input);
    defragment(&mut disk);

    let mut checksum: u64 = 0;
    for i in 0..disk.len() {
        let id = disk[i];
        if id != u64::MAX {
            checksum += id * i as u64
        }
    }

    println!("solution part 1: {checksum}");
    Ok(())
}

fn parse_filesystem(input: String) -> Vec<u64> {
    let line = input.lines().next().unwrap();
    let mut disk = Vec::new();
    let mut next_index: u64 = 0;
    for i in 0..line.len() {
        let c = line.chars().nth(i).unwrap();
        let d = c.to_string().parse::<usize>().unwrap();
        if i % 2 == 0 {
            (0..d).for_each(|_| disk.push(next_index));
            next_index += 1;
        } else {
            (0..d).for_each(|_| disk.push(u64::MAX));
        }
    }
    disk
}

fn defragment(disk: &mut Vec<u64>) {
    let mut j = disk.len() - 1;
    while j > 0 {
        let id = disk[j];
        if id == u64::MAX {
            j -= 1;
            continue;
        }
        if let Some(i) = disk[0..j].iter().position(|&id| id == u64::MAX) {
            disk[i] = id;
            disk[j] = u64::MAX;
        }
        j -= 1;
    }
}
