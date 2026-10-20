use anyhow::{Context, Result};
use std::fs::{self, File};
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

pub fn read_lines_as_string_vec(filename: impl AsRef<Path>) -> Result<Vec<String>> {
    let message = fs::read_to_string(filename)?;
    Ok(message.lines().map(String::from).collect())
}

pub fn read_lines_as_string(filename: impl AsRef<Path>) -> Result<String> {
    let message = fs::read_to_string(filename)?;
    return Ok(message);
}
