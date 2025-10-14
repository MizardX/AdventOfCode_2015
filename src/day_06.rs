use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Rect {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Rect {
    pub const fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Self {
        Self { x1, y1, x2, y2 }
    }
}

#[derive(Debug, Error, PartialEq)]
enum ParseError {
    #[error("Invalid prefix")]
    InvalidPrefix,
    #[error("Missing delimiter")]
    MissingDelimiter,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

impl TryFrom<&[u8]> for Rect {
    type Error = ParseError;

    fn try_from(text: &[u8]) -> Result<Self, Self::Error> {
        const SPLIT: &[u8] = b" through ";
        let ix = text
            .windows(SPLIT.len())
            .position(|w| w == SPLIT)
            .ok_or(ParseError::MissingDelimiter)?;
        let left = &text[..ix];
        let left_comma = left
            .iter()
            .position(|&ch| ch == b',')
            .ok_or(ParseError::MissingDelimiter)?;
        let right = &text[ix + SPLIT.len()..];
        let right_comma = right
            .iter()
            .position(|&ch| ch == b',')
            .ok_or(ParseError::MissingDelimiter)?;
        let x1 = parse_num(&left[..left_comma])?;
        let y1 = parse_num(&left[left_comma + 1..])?;
        let x2 = parse_num(&right[..right_comma])?;
        let y2 = parse_num(&right[right_comma + 1..])?;
        Ok(Self::new(x1, y1, x2, y2))
    }
}

fn parse_num(num: &[u8]) -> Result<usize, ParseIntError> {
    let s = unsafe { std::str::from_utf8_unchecked(num) };
    s.parse()
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    TurnOn(Rect),
    Toggle(Rect),
    TurnOff(Rect),
}

impl TryFrom<&[u8]> for Instruction {
    type Error = ParseError;

    fn try_from(line: &[u8]) -> Result<Self, Self::Error> {
        if let Some(rest) = line.strip_prefix(b"turn on ") {
            Ok(Self::TurnOn(rest.try_into()?))
        } else if let Some(rest) = line.strip_prefix(b"turn off ") {
            Ok(Self::TurnOff(rest.try_into()?))
        } else if let Some(rest) = line.strip_prefix(b"toggle ") {
            Ok(Self::Toggle(rest.try_into()?))
        } else {
            Err(ParseError::InvalidPrefix)
        }
    }
}

#[aoc_generator(day6)]
fn parse(input: &[u8]) -> Result<Vec<Instruction>, ParseError> {
    input
        .split(|&ch| ch == b'\n')
        .map(std::convert::TryInto::try_into)
        .collect()
}

#[aoc(day6, part1)]
fn part_1(instructions: &[Instruction]) -> usize {
    let mut grid = vec![[false; 1000]; 1000].into_boxed_slice();
    for instr in instructions {
        match instr {
            Instruction::TurnOn(rect) => {
                for row in &mut grid[rect.y1..=rect.y2] {
                    for cell in &mut row[rect.x1..=rect.x2] {
                        *cell = true;
                    }
                }
            }
            Instruction::TurnOff(rect) => {
                for row in &mut grid[rect.y1..=rect.y2] {
                    for cell in &mut row[rect.x1..=rect.x2] {
                        *cell = false;
                    }
                }
            }
            Instruction::Toggle(rect) => {
                for row in &mut grid[rect.y1..=rect.y2] {
                    for cell in &mut row[rect.x1..=rect.x2] {
                        *cell = !*cell;
                    }
                }
            }
        }
    }
    grid.iter().flatten().map(|&v| usize::from(v)).sum()
}

#[aoc(day6, part2)]
fn part_2(instructions: &[Instruction]) -> usize {
    let mut grid = vec![[0_u8; 1000]; 1000].into_boxed_slice();
    for instr in instructions {
        match instr {
            Instruction::TurnOn(rect) => {
                for row in &mut grid[rect.y1..=rect.y2] {
                    for cell in &mut row[rect.x1..=rect.x2] {
                        *cell += 1;
                    }
                }
            }
            Instruction::TurnOff(rect) => {
                for row in &mut grid[rect.y1..=rect.y2] {
                    for cell in &mut row[rect.x1..=rect.x2] {
                        *cell = (*cell).saturating_sub(1);
                    }
                }
            }
            Instruction::Toggle(rect) => {
                for row in &mut grid[rect.y1..=rect.y2] {
                    for cell in &mut row[rect.x1..=rect.x2] {
                        *cell += 2;
                    }
                }
            }
        }
    }
    grid.iter().flatten().map(|&v| v as usize).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(b"turn on 0,0 through 999,999" => Ok(vec![Instruction::TurnOn(Rect::new(0, 0, 999, 999))]))]
    #[test_case(b"toggle 0,0 through 999,0" => Ok(vec![Instruction::Toggle(Rect::new(0, 0, 999, 0))]))]
    #[test_case(b"turn off 499,499 through 500,500" => Ok(vec![Instruction::TurnOff(Rect::new(499, 499, 500, 500))]))]
    fn test_parse(input: &[u8]) -> Result<Vec<Instruction>, ParseError> {
        parse(input)
    }

    #[test_case(&[Instruction::TurnOn(Rect::new(0,0,999,999))] => 1_000_000)]
    #[test_case(&[Instruction::Toggle(Rect::new(0,0,999,0))] => 1_000)]
    #[test_case(&[Instruction::TurnOn(Rect::new(0,0,999,999)), Instruction::TurnOff(Rect::new(499,499,500,500))] => 999_996)]
    fn test_part_1(instructions: &[Instruction]) -> usize {
        part_1(instructions)
    }

    #[test_case(&[Instruction::TurnOn(Rect::new(0,0,0,0))] => 1)]
    #[test_case(&[Instruction::Toggle(Rect::new(0,0,999,999))] => 2_000_000)]
    fn test_part_2(instructions: &[Instruction]) -> usize {
        part_2(instructions)
    }
}
