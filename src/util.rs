use anyhow::{Context, Result};
use std::fs::File;
use std::io::{self, BufRead};
use std::{fs, path::Path};

pub fn read_input(path: impl AsRef<Path>) -> Result<String> {
    let path = path.as_ref();
    fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))
}

pub fn parse_lines<T: std::str::FromStr>(input: &str) -> Result<Vec<T>>
where
    T::Err: std::error::Error + Send + Sync + 'static,
{
    input.lines().map(|l| Ok(l.trim().parse()?)).collect()
}

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
