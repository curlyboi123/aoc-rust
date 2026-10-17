use anyhow::Result;
use log::{debug, info};

pub fn solve(lines: &[String]) -> Result<(String, String)> {
    Ok((part1(lines)?.to_string(), part2(lines)?.to_string()))
}

fn simulate_descent(lines: &[String], right: usize, down: usize) -> u32 {
    let mut position = (0, 0);
    let row_length = lines[0].len();
    let mut tree_count = 0;

    while position.0 < lines.len() {
        let value = lines[position.0].chars().nth(position.1).unwrap();
        debug!("Value: {}", value);

        if value == '#' {
            tree_count += 1;
        }

        position = (position.0 + down, (position.1 + right) % row_length);
    }

    info!("Tree count: {}", tree_count);
    return tree_count;
}

fn part1(lines: &[String]) -> Result<u64> {
    Ok(simulate_descent(lines, 3, 1) as u64)
}

fn part2(lines: &[String]) -> Result<u64> {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut result = 1;
    for (right, down) in slopes {
        result *= simulate_descent(lines, right, down) as u64;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::lines_from_str;

    const EXAMPLE: &str = include_str!("../../inputs/day03_sample.txt");

    #[test]
    fn example_part1() {
        let lines = lines_from_str(EXAMPLE);
        println!("{:?}", lines);
        assert_eq!(part1(&lines).unwrap(), 7);
    }

    #[test]
    fn example_part2() {
        let lines = lines_from_str(EXAMPLE);
        assert_eq!(part2(&lines).unwrap(), 336);
    }
}
