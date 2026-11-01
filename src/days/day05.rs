use anyhow::Result;

pub fn solve(file_string: &str) -> Result<(String, String)> {
    Ok((
        part1(file_string).to_string(),
        part2(file_string).to_string(),
    ))
}

fn part1(file_string: &str) -> u64 {
    let mut highest_seat_id = u64::MIN;

    for line in file_string.lines() {
        println!("{line:?}");

        let mut lo: u32= 0;
        let mut hi: u32 = 127;

        for b in line.bytes().take(7) {
            match b {
                b'F' => { hi = lo.midpoint(hi) },
                b'B' => { lo = (lo + hi).div_ceil(2) },
                _ => unreachable!(),
            }
        }
        let row = lo; // Make logic better by ensuring lo and hi are equal

        let mut lo_col: u32 = 0;
        let mut hi_col: u32 = 7;

        for b in line.bytes().rev().take(3).rev() {
            match b {
                b'L' => { hi_col = lo_col.midpoint(hi_col) },
                b'R' => { lo_col = (lo_col + hi_col).div_ceil(2) },
                _ => unreachable!(),
            }
        }

        let col = lo_col;

        highest_seat_id = u64::max(highest_seat_id,u64::from((row * 8) + col));
    }

    highest_seat_id
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
