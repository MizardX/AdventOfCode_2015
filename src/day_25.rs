use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Expected substring is missing")]
    MissingSubstring,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[aoc_generator(day25)]
fn parse(input: &str) -> Result<(u64, u64), ParseError> {
    let rest = input
        .strip_prefix(
            "To continue, please consult the code grid in the manual.  Enter the code at row ",
        )
        .ok_or(ParseError::MissingSubstring)?;
    let (row, rest) = rest
        .split_once(", column ")
        .ok_or(ParseError::MissingSubstring)?;
    let col = rest.strip_suffix(".").ok_or(ParseError::MissingSubstring)?;
    Ok((row.parse()?, col.parse()?))
}

#[allow(clippy::manual_midpoint)]
const fn translate(row: u64, col: u64) -> u64 {
    let diag = row + col;
    diag * (diag - 1) / 2 - row + 1
}

#[aoc(day25, part1)]
const fn part_1(input: &(u64, u64)) -> u64 {
    let (row, col) = *input;
    let index = translate(row, col);
    calc_code(index)
}

const fn calc_code(index: u64) -> u64 {
    20_151_125 * mod_pow(252_533, index - 1, 33_554_393) % 33_554_393
}

const fn mod_pow(mut base: u64, mut power: u64, modulo: u64) -> u64 {
    let mut scale = 1;
    while power >= 1 {
        if power & 1 == 0 {
            base = base * base % modulo;
            power /= 2;
        } else {
            scale = scale * base % modulo;
            power -= 1;
        }
    }
    scale
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const EXAMPLE: &str = "\
        To continue, please consult the code grid in the manual.  Enter the code at row 3, column 4.\
    ";

    #[test]
    fn test_parse() {
        let (row, col) = parse(EXAMPLE).unwrap();
        assert_eq!(row, 3);
        assert_eq!(col, 4);
    }

    #[test_case(1, 1 => 20_151_125)]
    #[test_case(1, 2 => 18_749_137)]
    #[test_case(1, 3 => 17_289_845)]
    #[test_case(2, 1 => 31_916_031)]
    #[test_case(2, 2 => 21_629_792)]
    #[test_case(3, 1 => 16_080_970)]
    fn test_part_1(row: u64, col: u64) -> u64 {
        part_1(&(row, col))
    }

    #[test_case(1, 1, 1000 => 1)]
    #[test_case(1, 100, 1000 => 1)]
    #[test_case(2, 2, 1000 => 4)]
    #[test_case(2, 10, 1000 => 24)]
    fn test_mod_pow(base: u64, power: u64, modulo: u64) -> u64 {
        mod_pow(base, power, modulo)
    }

    #[test_case(1, 1 => 1)]
    #[test_case(1, 2 => 3)]
    #[test_case(1, 3 => 6)]
    #[test_case(1, 4 => 10)]
    #[test_case(1, 5 => 15)]
    #[test_case(1, 6 => 21)]
    #[test_case(2, 1 => 2)]
    #[test_case(2, 2 => 5)]
    #[test_case(2, 3 => 9)]
    #[test_case(2, 4 => 14)]
    #[test_case(2, 5 => 20)]
    #[test_case(3, 1 => 4)]
    #[test_case(3, 2 => 8)]
    #[test_case(3, 3 => 13)]
    #[test_case(3, 4 => 19)]
    #[test_case(4, 1 => 7)]
    #[test_case(4, 2 => 12)]
    #[test_case(4, 3 => 18)]
    #[test_case(5, 1 => 11)]
    #[test_case(5, 2 => 17)]
    #[test_case(6, 1 => 16)]
    fn test_translate(row: u64, col: u64) -> u64 {
        translate(row, col)
    }
}
