use std::collections::HashSet;

use anyhow::Result;

pub fn solve(file_string: &str) -> Result<(String, String)> {
    Ok((
        part1(file_string).to_string(),
        part2(file_string).to_string(),
    ))
}

fn get_seat_id(line: &str) -> u32 {
    // println!("{line:?}");

    let mut lo: u32 = 0;
    let mut hi: u32 = 127;

    for b in line.bytes().take(7) {
        match b {
            b'F' => hi = lo.midpoint(hi),
            b'B' => lo = (lo + hi).div_ceil(2),
            _ => unreachable!(),
        }
    }
    let row = lo; // Make logic better by ensuring lo and hi are equal

    let mut lo_col: u32 = 0;
    let mut hi_col: u32 = 7;

    for b in line.bytes().rev().take(3).rev() {
        match b {
            b'L' => hi_col = lo_col.midpoint(hi_col),
            b'R' => lo_col = (lo_col + hi_col).div_ceil(2),
            _ => unreachable!(),
        }
    }

    let col = lo_col;

    (row * 8) + col
}

fn part1(file_string: &str) -> u32 {
    let mut highest_seat_id = u32::MIN;

    for line in file_string.lines() {
        let seat_id = get_seat_id(line);
        highest_seat_id = u32::max(highest_seat_id, seat_id);
    }

    highest_seat_id
}

fn part2(file_string: &str) -> u32 {
    let mut filled_seat_ids: HashSet<u32> = HashSet::new();
    let mut min_seat_id = u32::MAX;
    let mut max_seat_id = u32::MIN;


    for line in file_string.lines() {
        let seat_id = get_seat_id(line);
        filled_seat_ids.insert(seat_id);
        min_seat_id = u32::min(min_seat_id, seat_id);
        max_seat_id = u32::max(max_seat_id, seat_id);
    }

    // println!("Fille seats: {filled_seat_ids:?}");
    // println!("Min seat id: {min_seat_id}");
    // println!("Max seat id: {max_seat_id}");
    
    for i in min_seat_id..=max_seat_id {
        // println!("Seat_id: {i}");
        if !filled_seat_ids.contains(&i) { return i }
    }
    0 // TODO: Do proper result here
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../inputs/day05_sample.txt");

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 820);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 0);
    }
}
