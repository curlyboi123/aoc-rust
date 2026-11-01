use anyhow::Result;
use log::{debug, info};

pub fn solve(file_string: &str) -> Result<(String, String)> {
    Ok((
        part1(file_string).to_string(),
        part2(file_string).to_string(),
    ))
}

fn simulate_descent(grid: &[String], right: usize, down: usize) -> u64 {
    let mut position = (0, 0);
    let row_length = grid[0].len();
    let mut tree_count = 0;

    while position.0 < grid.len() {
        let value = grid[position.0].chars().nth(position.1).unwrap();
        debug!("Value: {value}");

        if value == '#' {
            tree_count += 1;
        }

        position = (position.0 + down, (position.1 + right) % row_length);
    }

    info!("Tree count: {tree_count}");
    tree_count
}

fn part1(file_string: &str) -> u64 {
    let grid: Vec<String> = file_string.lines().map(String::from).collect();
    simulate_descent(&grid, 3, 1)
}

fn part2(file_string: &str) -> u64 {
    let grid: Vec<String> = file_string.lines().map(String::from).collect();

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut result = 1;

    for (right, down) in slopes {
        result *= simulate_descent(&grid, right, down);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../inputs/day03_sample.txt");

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 7);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 336);
    }
}
