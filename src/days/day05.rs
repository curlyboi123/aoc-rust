use anyhow::Result;

pub fn solve(file_string: &str) -> Result<(String, String)> {
    Ok((
        part1(file_string).to_string(),
        part2(file_string).to_string(),
    ))
}

fn part1(file_string: &str) -> u64 {
    for line in file_string.lines() {
        println!("{line:?}");
    }
    0
}

fn part2(_file_string: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../inputs/day05_sample.txt");

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 0);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 0);
    }
}
 
