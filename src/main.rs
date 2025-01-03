use itertools::Itertools;
use std::{error::Error, fs, process::Command, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let days = fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src/bin/"))?
        .map(|p| p.ok().unwrap().path())
        .filter(|path| path.extension().unwrap() == "rs")
        .filter_map(|p| p.file_stem()?.to_str().map(str::to_string))
        .sorted()
        .collect::<Vec<_>>();

    for day in &days {
        let now = Instant::now();
        let cmd = Command::new("cargo")
            .args(["run", "--release", "--bin", day])
            .output()?;
        let output = String::from_utf8(cmd.stdout)?;
        let time = now.elapsed().as_millis();
        println!("# {} (took {} ms)\n{}", day, time, output);
    }

    Ok(())
}
