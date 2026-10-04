use anyhow::{Error, Result};
use std::collections::HashMap;

pub fn solve(lines: &[String]) -> Result<(String, String)> {
    Ok((part1(lines)?.to_string(), part2(lines)?.to_string()))
}

fn part1(lines: &[String]) -> Result<u64> {
    let map: HashMap<u64, u64> = lines
        .into_iter()
        .filter_map(|n| n.parse::<u64>().ok())
        .map(|n| (n, n))
        .collect();
    // println!("Map: {map:?}");

    for (key, _value) in &map {
        let remainder = 2020 - key;
        if let Some(other) = map.get(&remainder) {
            println!("Found: {key} + {other} = 2020");
            return Ok(key * other);
        }
    }

    return Err(Error::msg("No pair found"));
}

fn part2(_lines: &[String]) -> Result<u64> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::lines_from_str;

    const EXAMPLE: &str = include_str!("../../inputs/day01_full.txt");

    #[test]
    fn example_part1() {
        let lines = lines_from_str(EXAMPLE);
        assert_eq!(part1(&lines).unwrap(), 0);
    }

    #[test]
    fn example_part2() {
        let lines = lines_from_str(EXAMPLE);
        assert_eq!(part2(&lines).unwrap(), 0);
    }
}
