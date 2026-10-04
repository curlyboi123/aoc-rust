use anyhow::{Context, Result};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines(path: impl AsRef<Path>) -> Result<Vec<String>> {
    let path = path.as_ref();
    let file = File::open(path).with_context(|| format!("opening {}", path.display()))?;
    io::BufReader::new(file)
        .lines()
        .collect::<io::Result<Vec<_>>>()
        .with_context(|| format!("reading {}", path.display()))
}

pub fn lines_from_str(s: &str) -> Vec<String> {
    s.lines().map(String::from).collect()
}
