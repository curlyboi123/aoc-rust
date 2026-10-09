use anyhow::Result;

pub fn solve(lines: &[String]) -> Result<(String, String)> {
    Ok((part1(lines)?.to_string(), part2(lines)?.to_string()))
}

#[derive(Debug)]
struct PasswordPolicy {
    min: usize,
    max: usize,
    character: char,
}

fn split_content(line: &str) -> Result<(PasswordPolicy, &str), anyhow::Error> {
    let (a, password) = line.split_once(": ").unwrap();
    let password = password.trim();
    let (amounts, character) = a.split_once(" ").unwrap();
    let character = character.chars().next().unwrap();

    let (min, max) = amounts.split_once("-").unwrap();
    let min: usize = min.parse()?;
    let max: usize = max.parse()?;
    Ok((
        PasswordPolicy {
            min,
            max,
            character,
        },
        password,
    ))
}

fn part1(lines: &[String]) -> Result<u64> {
    let mut valid_password_count = 0;
    for line in lines {
        let (policy, password) = split_content(line)?;

        let mut password_valid = true;
        let mut count = 0;
        for char in password.chars() {
            if char == policy.character {
                count += 1;
            }
            if count > policy.max {
                password_valid = false;
                break;
            }
        }
        if count < policy.min {
            password_valid = false;
        }

        valid_password_count += if password_valid { 1 } else { 0 };
    }
    println!("Valid password count: {valid_password_count}");
    Ok(valid_password_count)
}

fn part2(lines: &[String]) -> Result<u64> {
    let mut valid_password_count = 0;
    for line in lines {
        let (policy, password) = split_content(line)?;

        let is_valid_password = (password.chars().nth(policy.min - 1) == Some(policy.character))
            != (password.chars().nth(policy.max - 1) == Some(policy.character));
        valid_password_count += if is_valid_password { 1 } else { 0 };
    }
    Ok(valid_password_count)
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
        assert_eq!(part2(&lines).unwrap(), 1);
    }
}
