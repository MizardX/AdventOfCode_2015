use std::collections::HashMap;
use std::num::ParseIntError;

use thiserror::Error;

use crate::utils::Grid;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Missing delimiter: {0}")]
    MissingDelimiter(&'static str),
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[aoc_generator(day13)]
fn parse(input: &str) -> Result<Grid<i64>, ParseError> {
    let mut names = HashMap::<&str, usize>::new();
    let mut grid = Grid::new(10, 10);
    for line in input.lines() {
        //(name) would (gain|loose) (num) happiness units by sitting next to (name).
        let (subject, rest) = line
            .split_once(' ')
            .ok_or(ParseError::MissingDelimiter("first space"))?;
        let name_count = names.len();
        let subject_index = *names.entry(subject).or_insert(name_count);
        let rest = rest
            .strip_prefix("would ")
            .ok_or(ParseError::MissingDelimiter("would"))?;
        let (sign, rest) = if let Some(rest) = rest.strip_prefix("gain ") {
            (1, rest)
        } else if let Some(rest) = rest.strip_prefix("lose ") {
            (-1, rest)
        } else {
            return Err(ParseError::MissingDelimiter("gain|lose"));
        };
        let (num, rest) = rest
            .split_once(' ')
            .ok_or(ParseError::MissingDelimiter("space after num"))?;
        let num = num.parse::<i64>()?;
        let rest = rest
            .strip_prefix("happiness units by sitting next to ")
            .ok_or(ParseError::MissingDelimiter("final phrase"))?;
        let object = rest
            .strip_suffix(".")
            .ok_or(ParseError::MissingDelimiter("Final period"))?;
        let name_count = names.len();
        let object_index = *names.entry(object).or_insert(name_count);
        grid[(subject_index, object_index)] = sign * num;
    }
    Ok(grid.resize(names.len(), names.len()))
}

#[aoc(day13, part1)]
fn part_1(input: &Grid<i64>) -> i64 {
    fn find_permutation(perm: &mut [usize], index: usize, grid: &Grid<i64>) -> i64 {
        if index == perm.len() {
            perm.iter()
                .zip(perm.iter().cycle().skip(1))
                .map(|(&i1, &i2)| grid[(i1, i2)] + grid[(i2, i1)])
                .sum()
        } else {
            let mut max = i64::MIN;
            for i in index..perm.len() {
                perm.swap(index, i);
                max = max.max(find_permutation(perm, index + 1, grid));
                perm.swap(index, i);
            }
            max
        }
    }
    let mut perm = (0..input.rows()).collect::<Vec<_>>();
    find_permutation(&mut perm, 0, input)
}

#[aoc(day13, part2)]
fn part_2(input: &Grid<i64>) -> i64 {
    fn find_permutation(perm: &mut [usize], index: usize, grid: &Grid<i64>) -> i64 {
        if index == perm.len() {
            perm.iter()
                .zip(&perm[1..])
                .map(|(&i1, &i2)| grid[(i1, i2)] + grid[(i2, i1)])
                .sum()
        } else {
            let mut max = i64::MIN;
            for i in index..perm.len() {
                perm.swap(index, i);
                max = max.max(find_permutation(perm, index + 1, grid));
                perm.swap(index, i);
            }
            max
        }
    }
    let mut perm = (0..input.rows()).collect::<Vec<_>>();
    find_permutation(&mut perm, 0, input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        Alice would gain 54 happiness units by sitting next to Bob.\n\
        Alice would lose 79 happiness units by sitting next to Carol.\n\
        Alice would lose 2 happiness units by sitting next to David.\n\
        Bob would gain 83 happiness units by sitting next to Alice.\n\
        Bob would lose 7 happiness units by sitting next to Carol.\n\
        Bob would lose 63 happiness units by sitting next to David.\n\
        Carol would lose 62 happiness units by sitting next to Alice.\n\
        Carol would gain 60 happiness units by sitting next to Bob.\n\
        Carol would gain 55 happiness units by sitting next to David.\n\
        David would gain 46 happiness units by sitting next to Alice.\n\
        David would lose 7 happiness units by sitting next to Bob.\n\
        David would gain 41 happiness units by sitting next to Carol.\
    ";

    #[test]
    fn test_parse() {
        let grid = parse(EXAMPLE.trim()).unwrap();
        assert_eq!(grid.rows(), 4);
        assert_eq!(grid.cols(), 4);

        let expected = [
            [0, 54, -79, -2],
            [83, 0, -7, -63],
            [-62, 60, 0, 55],
            [46, -7, 41, 0],
        ];

        for (r, row) in expected.into_iter().enumerate() {
            for (c, value) in row.into_iter().enumerate() {
                assert_eq!(grid[(r, c)], value);
            }
        }
    }

    #[test]
    fn test_part_1() {
        let grid = parse(EXAMPLE.trim()).unwrap();

        let result = part_1(&grid);

        assert_eq!(result, 330);
    }

    #[test]
    fn test_part_2() {
        let grid = parse(EXAMPLE.trim()).unwrap();

        let result = part_2(&grid);

        assert_eq!(result, 330 - (46 + (-2)));
    }
}
