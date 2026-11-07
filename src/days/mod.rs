use anyhow::Result;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;

pub type Solver = fn(&str) -> Result<(String, String)>;

pub fn get(day: u8) -> Option<Solver> {
    match day {
        1 => Some(day01::solve),
        2 => Some(day02::solve),
        3 => Some(day03::solve),
        4 => Some(day04::solve),
        5 => Some(day05::solve),
        6 => Some(day06::solve),
        _ => None,
    }
}
