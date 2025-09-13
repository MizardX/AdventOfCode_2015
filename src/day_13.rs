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
        let (name1, rest) = line
            .split_once(' ')
            .ok_or(ParseError::MissingDelimiter("first space"))?;
        let name_count = names.len();
        let name1_index = *names.entry(name1).or_insert(name_count);
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
        let name2 = rest
            .strip_suffix(".")
            .ok_or(ParseError::MissingDelimiter("Final period"))?;
        let name_count = names.len();
        let name2_index = *names.entry(name2).or_insert(name_count);
        grid[(name1_index, name2_index)] = sign * num;
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
mod test {
    use super::*;

    const EXAMPLE: &str = r"
Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.
";

    #[test]
    fn test_parse() {
        let grid = parse(EXAMPLE.trim()).unwrap();
        assert_eq!(grid.rows(), 4);
        assert_eq!(grid.cols(), 4);

        assert_eq!(grid[(0, 1)], 54);
        assert_eq!(grid[(0, 2)], -79);
        assert_eq!(grid[(0, 3)], -2);

        assert_eq!(grid[(1, 0)], 83);
        assert_eq!(grid[(1, 2)], -7);
        assert_eq!(grid[(1, 3)], -63);

        assert_eq!(grid[(2, 0)], -62);
        assert_eq!(grid[(2, 1)], 60);
        assert_eq!(grid[(2, 3)], 55);

        assert_eq!(grid[(3, 0)], 46);
        assert_eq!(grid[(3, 1)], -7);
        assert_eq!(grid[(3, 2)], 41);
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
