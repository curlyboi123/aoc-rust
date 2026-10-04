use anyhow::{Result, anyhow};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    day: u8,
    /// Use the sample input instead of the full one
    #[arg(short, long)]
    sample: bool,
    /// Explicit input path (overrides --sample)
    input: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let kind = if args.sample { "sample" } else { "full" };
    let path = args.input.unwrap_or_else(|| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join(format!("day{:02}_{kind}.txt", args.day))
    });
    let solver =
        aoc::days::get(args.day).ok_or_else(|| anyhow!("day {} not implemented", args.day))?;

    let lines = aoc::util::read_lines(&path)?;
    let (p1, p2) = solver(&lines)?;
    println!("Part 1: {p1}\nPart 2: {p2}");
    Ok(())
}
