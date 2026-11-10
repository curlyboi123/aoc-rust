use std::collections::HashMap;

use anyhow::Result;

pub fn solve(file_string: &str) -> Result<(String, String)> {
    Ok((
        part1(file_string).to_string(),
        part2(file_string).to_string(),
    ))
}

fn part1(file_string: &str) -> u64 {
    let answers: Vec<Vec<&str>> = file_string
        .split("\n\n")
        .map(|group| group.split_whitespace().collect())
        .collect();
    // println!("Answers: {answers:?}");
    for (group_idx, group) in answers.iter().enumerate() {
        println!("Group {group_idx}");
        for (person_idx, &person_answers) in group.iter().enumerate() {
            println!("Person {person_idx}: {person_answers}");
            let mut answer_count: HashMap<char, usize> = HashMap::new();
            for answer in person_answers.as_bytes() {
                println!("Answer: {answer}");
            }


        }
        println!();
    }
    0
}

fn part2(_lines: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../inputs/day06_sample.txt");

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 11);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 0);
    }
}
