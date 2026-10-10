use anyhow::Result;

pub mod day01;
pub mod day02;
pub mod day03;

pub type Solver = fn(&[String]) -> Result<(String, String)>;

pub fn get(day: u8) -> Option<Solver> {
    match day {
        1 => Some(day01::solve),
        2 => Some(day02::solve),
        3 => Some(day03::solve),
        _ => None,
    }
}
