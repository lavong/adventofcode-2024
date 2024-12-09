use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/day09.txt")?;

    let mut disk = parse_filesystem(input);
    defragment(&mut disk);

    let checksum: u64 = disk
        .iter()
        .flat_map(|&(id, size)| (0..size).map(move |_| id))
        .enumerate()
        .map(|(i, id)| if id != u64::MAX { id * i as u64 } else { 0 })
        .sum();

    println!("solution part 2: {checksum}");
    Ok(())
}

fn parse_filesystem(input: String) -> Vec<(u64, usize)> {
    let line = input.lines().next().unwrap();
    let mut disk = Vec::new();
    let mut next_index: u64 = 0;
    for i in 0..line.len() {
        let c = line.chars().nth(i).unwrap();
        let d = c.to_string().parse::<usize>().unwrap();
        if i % 2 == 0 {
            disk.push((next_index, d));
            next_index += 1;
        } else {
            disk.push((u64::MAX, d));
        }
    }
    disk
}

fn defragment(disk: &mut Vec<(u64, usize)>) {
    let mut j = disk.len() - 1;
    while j > 0 {
        let (id, size_j) = disk[j];
        if id == u64::MAX {
            j -= 1;
            continue;
        }
        if let Some(i) = disk[0..j]
            .iter()
            .position(|&(id, size_i)| id == u64::MAX && size_j <= size_i)
        {
            let size_i = disk[i].1;
            disk[i] = (id, size_j);
            disk[j] = (u64::MAX, size_j);
            if size_j < size_i {
                disk.insert(i + 1, (u64::MAX, size_i - size_j));
            }
        }
        j -= 1;
    }
}
