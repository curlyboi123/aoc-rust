use anyhow::Result;

pub mod day01;

pub type Solver = fn(&[String]) -> Result<(String, String)>;

pub fn get(day: u8) -> Option<Solver> {
    match day {
        1 => Some(day01::solve),
        _ => None,
    }
}
