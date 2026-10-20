use anyhow::{Result, anyhow};
use std::collections::HashMap;

pub fn solve(file_string: &str) -> Result<(String, String)> {
    Ok((
        part1(file_string)?.to_string(),
        part2(file_string)?.to_string(),
    ))
}

const REQUIRED_PASSPORT_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn has_required_fields(map: &HashMap<&str, &str>) -> bool {
    REQUIRED_PASSPORT_FIELDS.iter().all(|f| map.contains_key(f))
}

fn _has_valid_fields(map: &HashMap<&str, &str>) -> bool {
    true
}

fn part1(file_string: &str) -> Result<u64> {
    let passports: Vec<HashMap<&str, &str>> = file_string
        .split("\n\n")
        .map(|p| {
            p.split_whitespace()
                .map(|field| {
                    field
                        .split_once(':')
                        .ok_or_else(|| anyhow!("invalid passport field: {field}"))
                })
                .collect::<Result<HashMap<_, _>>>()
        })
        .collect::<Result<_>>()?;

    let mut invalid_passport_count = 0;

    for p in passports {
        println!("Passport: {p:?}");
        if has_required_fields(&p) {
            println!("Passport has required fields");
            invalid_passport_count += 1;
        }
        println!();
    }

    Ok(invalid_passport_count)
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
        assert_eq!(part1(EXAMPLE).unwrap(), 2);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE).unwrap(), 0);
    }
}
