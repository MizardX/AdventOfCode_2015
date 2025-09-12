use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, PartialEq)]
struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    fn new(length: u32, width: u32, height: u32) -> Self {
        Self {
            length,
            width,
            height,
        }
    }
}

#[derive(Debug, Error, PartialEq)]
enum ParseError {
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
    #[error("Missing delimiter")]
    MissingDelimiter,
}

#[aoc_generator(day2)]
fn parse_input(input: &[u8]) -> Result<Vec<Present>, ParseError> {
    input
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut dims = line.split(|&b| b == b'x').map(|num| {
                let s = unsafe { std::str::from_utf8_unchecked(num) };
                s.parse()
            });
            Ok(Present::new(
                dims.next().ok_or(ParseError::MissingDelimiter)??,
                dims.next().ok_or(ParseError::MissingDelimiter)??,
                dims.next().ok_or(ParseError::MissingDelimiter)??,
            ))
        })
        .collect()
}

#[aoc(day2, part1)]
fn solve_part_1(input: &[Present]) -> u32 {
    input
        .iter()
        .map(|b| {
            let lw = b.length * b.width;
            let wh = b.width * b.height;
            let hl = b.height * b.length;
            2 * (lw + wh + hl) + lw.min(wh).min(hl)
        })
        .sum()
}

#[aoc(day2, part2)]
fn solve_part_2(input: &[Present]) -> u32 {
    input
        .iter()
        .map(|b| {
            let max_dim = b.length.max(b.width).max(b.height);
            2 * (b.length + b.width + b.height - max_dim) + b.length * b.width * b.height
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_parse() {
        let input = b"2x3x4\n1x1x10";
        let result = parse_input(input);
        assert_eq!(
            result,
            Ok(vec![Present::new(2, 3, 4), Present::new(1, 1, 10)])
        );
    }

    #[test_case(Present::new(2, 3, 4) => 58)]
    #[test_case(Present::new(1, 1, 10) => 43)]
    fn test_part_1(present: Present) -> u32 {
        solve_part_1(&[present])
    }

    #[test_case(Present::new(2, 3, 4) => 34)]
    #[test_case(Present::new(1, 1, 10) => 14)]
    fn test_part_2(present: Present) -> u32 {
        solve_part_2(&[present])
    }
}
