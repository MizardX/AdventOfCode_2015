use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, PartialEq)]
struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    const fn new(length: u32, width: u32, height: u32) -> Self {
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
fn parse(input: &[u8]) -> Result<Vec<Present>, ParseError> {
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
fn part_1(input: &[Present]) -> u32 {
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
fn part_2(input: &[Present]) -> u32 {
    input
        .iter()
        .map(|b| {
            let max_dim = b.length.max(b.width).max(b.height);
            2 * (b.length + b.width + b.height - max_dim) + b.length * b.width * b.height
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const EXAMPLE1: &[u8] = b"2x3x4";
    const EXAMPLE2: &[u8] = b"1x1x10";

    #[test_case(EXAMPLE1 => &[Present::new(2, 3, 4)][..])]
    #[test_case(EXAMPLE2 => &[Present::new(1, 1, 10)][..])]
    fn test_parse(input: &[u8]) -> Vec<Present> {
        parse(input).unwrap()
    }

    #[test_case(EXAMPLE1 => 58)]
    #[test_case(EXAMPLE2 => 43)]
    fn test_part_1(input: &[u8]) -> u32 {
        let presents = parse(input).unwrap();
        part_1(&presents)
    }


    #[test_case(EXAMPLE1 => 34)]
    #[test_case(EXAMPLE2 => 14)]
    fn test_part_2(input: &[u8]) -> u32 {
        let presents = parse(input).unwrap();
        part_2(&presents)
    }
}
