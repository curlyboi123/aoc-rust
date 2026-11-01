use anyhow::{Result, anyhow};
use regex::Regex;
use std::collections::HashMap;

pub fn solve(file_string: &str) -> Result<(String, String)> {
    Ok((
        part1(file_string).to_string(),
        part2(file_string).to_string(),
    ))
}

const REQUIRED_PASSPORT_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn has_required_fields(map: &HashMap<&str, &str>) -> bool {
    REQUIRED_PASSPORT_FIELDS.iter().all(|f| map.contains_key(f))
}

fn validate_year(field: &str, expected_length: usize, min_val: i32, max_val: i32) -> bool {
    if field.len() != expected_length {
        return false;
    }
    let field = field.parse::<i32>().unwrap();
    if field < min_val || field > max_val { return false }
    true
}

fn has_valid_fields(map: &HashMap<&str, &str>) -> Result<bool> {
    if !has_required_fields(map) { return Ok(false) }

    // Birth year
    let Some(&birth_year) = map.get("byr") else { return Ok(false) };
    if !validate_year(birth_year, 4, 1920, 2002) {
        return Ok(false);
    }
    // Issue year
    let Some(&issue_year) = map.get("iyr") else { return Ok(false) };
    if !validate_year(issue_year, 4, 2010, 2020) {
        return Ok(false);
    }

    // Expiration year
    let Some(&expiration_year) = map.get("eyr") else { return Ok(false) };
    if !validate_year(expiration_year, 4, 2020, 2030) {
        return Ok(false);
    }

    // Height
    let Some(&height) = map.get("hgt") else { return Ok(false) };
    let re = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    let mut fields: Vec<(i32, &str)> = vec![];
    for (_, [val, unit]) in re.captures_iter(height).map(|caps| caps.extract()) {
        fields.push((val.parse::<i32>()?, unit));
    }
    let Some(&(val, unit)) = fields.first() else { return Ok(false) };
    let valid_height = match unit {
        "cm" => (150..=193).contains(&val),
        "in" => (59..=76).contains(&val),
        _ => false,
    };
    if !valid_height { return Ok(false) }

    // Hair color
    let Some(&hair_color) = map.get("hcl") else { return Ok(false) };
    let hair_color_re = Regex::new(r"^#(\d|[a-f]){6}$").unwrap();
    let has_valid_hair_color = hair_color_re.is_match(hair_color);
    if !has_valid_hair_color { return Ok(false) }

    // Eye color
    let Some(&eye_color) = map.get("ecl") else { return Ok(false)};
    let allowed_eye_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    if !allowed_eye_colors.contains(&eye_color) { return Ok(false) }

    // Passport ID
    let Some(&passport_id) = map.get("pid") else { return Ok(false) };
    let passport_re = Regex::new(r"^\d{9}$").unwrap();
    let has_valid_passport_id = passport_re.is_match(passport_id);
    if !has_valid_passport_id { return Ok(false) }
 
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
    Ok(passports)
}

fn part1(file_string: &str) -> u64 {
    let mut invalid_passport_count = 0;

    let passports = construct_passports(file_string).unwrap();

    for p in passports {
        if has_required_fields(&p) {
            invalid_passport_count += 1;
        }
    }

    invalid_passport_count
}

fn part2(file_string: &str) -> u64 {
    let passports = construct_passports(file_string).unwrap();

    let mut valid_passport_count = 0;

    for p in passports {
        if has_valid_fields(&p).unwrap() {
            valid_passport_count += 1;
        }
    }

    valid_passport_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../inputs/day04_sample.txt");

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 2);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 0);
    }
}
