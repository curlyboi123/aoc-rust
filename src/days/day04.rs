use anyhow::Result;

pub fn solve(file_string: &str) -> Result<(String, String)> {
    Ok((
        part1(file_string)?.to_string(),
        part2(file_string)?.to_string(),
    ))
}

fn part1(file_string: &str) -> Result<u64> {
    let passports: Vec<Vec<&str>> = file_string
        .split("\n\n")
        .map(|p| p.split_whitespace().collect())
        .collect();

    for p in passports {
        println!("Passport: {p:?}");
    }

    Ok(0)
}

fn part2(_file_string: &str) -> Result<u64> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../inputs/day04_sample.txt");

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE).unwrap(), 0);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE).unwrap(), 0);
    }
}
