use anyhow::{Result, anyhow};
use regex::Regex;
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

fn validate_year(field: &str, expected_length: usize, min_val: i32, max_val: i32) -> bool {
    if field.len() != expected_length {
        return false;
    };
    let field = field.parse::<i32>().unwrap();
    if field <= min_val || field >= max_val {
        return false;
    }
    true
}

fn has_valid_fields(map: &HashMap<&str, &str>) -> Result<bool> {
    if !has_required_fields(map) {
        return Ok(false);
    }

    // Birth year
    let birth_year = map.get("byr").unwrap();
    if !validate_year(*birth_year, 4, 1920, 2002) {
        return Ok(false);
    }
    // Issue year
    let issue_year = map.get("iyr").unwrap();
    if !validate_year(*issue_year, 4, 2010, 2020) {
        return Ok(false);
    }

    // Expiration year
    let expiration_year = map.get("eyr").unwrap();
    if !validate_year(*expiration_year, 4, 2020, 2030) {
        return Ok(false);
    }

    // Height
    let height = map.get("hgt").unwrap();
    let re = Regex::new(r"(\d+)(cm|in)").unwrap();
    let mut fields: Vec<(i32, &str)> = vec![];
    for (_, [f1, f2]) in re.captures_iter(height).map(|caps| caps.extract()) {
        fields.push((f1.parse::<i32>()?, f2))
    }
    println!("Height regex results: {fields:?}");

    Ok(true)
}

fn construct_passports(file_string: &str) -> Result<Vec<HashMap<&str, &str>>> {
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
    return Ok(passports);
}

fn part1(file_string: &str) -> Result<u64> {
    let mut invalid_passport_count = 0;

    let passports = construct_passports(file_string).unwrap();

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

fn part2(file_string: &str) -> Result<u64> {
    let passports = construct_passports(file_string).unwrap();

    let mut valid_passport_count = 0;

    for p in passports {
        if !has_valid_fields(&p).unwrap() {
            valid_passport_count += 1;
        }
    }

    Ok(valid_passport_count)
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
