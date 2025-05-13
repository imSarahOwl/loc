use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let mut n_loc: i32 = 0;
    let args = Cli::parse();
    let file = File::open(&args.path)
        .with_context(|| format!("could not be read file {} :p", args.path.display()))?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if !line?.is_empty() {
            n_loc += 1;
        }
    }

    println!("{} locs", n_loc);
    Ok(())
}
