use anyhow::{Result, anyhow};
use std::cmp::Ordering;
use std::collections::HashSet;

pub fn solve(file_string: &str) -> Result<(String, String)> {
    Ok((
        part1(file_string)?.to_string(),
        part2(file_string)?.to_string(),
    ))
}

fn part1(file_string: &str) -> Result<u64> {
    let numbers = file_string
        .lines()
        .map(|l| l.trim().parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut seen = HashSet::new();

    for n in numbers {
        let remainder = 2020 - n;
        if seen.contains(&remainder) {
            println!("Found: {n} + {remainder} = 2020");
            return Ok(n * remainder);
        }
        seen.insert(n);
    }

    Err(anyhow::anyhow!("No solution found"))
}

fn part2(file_string: &str) -> Result<u64> {
    let mut numbers = file_string
        .lines()
        .map(|l| l.trim().parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;
    numbers.sort_unstable();

    for i in 0..numbers.len().saturating_sub(2) {
        let (mut lo, mut hi) = (i + 1, numbers.len() - 1);
        while lo < hi {
            let sum = numbers[i] + numbers[lo] + numbers[hi];
            match sum.cmp(&2020) {
                Ordering::Equal => return Ok(numbers[i] * numbers[lo] * numbers[hi]),
                Ordering::Less => lo += 1,
                Ordering::Greater => hi -= 1,
            }
        }
    }

    Err(anyhow!("no solution found"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../inputs/day01_sample.txt");

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE).unwrap(), 514579);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE).unwrap(), 241861950);
    }
}
