use anyhow::Result;

pub fn solve(lines: &[String]) -> Result<(String, String)> {
    Ok((part1(lines)?.to_string(), part2(lines)?.to_string()))
}

fn part1(_lines: &[String]) -> Result<u64> {
    Ok(0)
}

fn part2(_lines: &[String]) -> Result<u64> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::lines_from_str;

    const EXAMPLE: &str = include_str!("../../inputs/day02_sample.txt");

    #[test]
    fn example_part1() {
        let lines = lines_from_str(EXAMPLE);
        println!("{:?}", lines);
        assert_eq!(part1(&lines).unwrap(), 2);
    }

    #[test]
    fn example_part2() {
        let lines = lines_from_str(EXAMPLE);
        assert_eq!(part2(&lines).unwrap(), 0);
    }
}
