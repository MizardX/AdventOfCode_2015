use std::collections::HashMap;
use std::num::ParseIntError;

use thiserror::Error;

use crate::utils::Grid;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Missing delimiter")]
    MissingDelimiter,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[aoc_generator(day9)]
fn parse(input: &str) -> Result<Grid<u64>, ParseError> {
    let mut names = HashMap::new();
    let mut grid = Grid::new(10, 10);
    for line in input.lines() {
        let (name1, rest) = line
            .split_once(" to ")
            .ok_or(ParseError::MissingDelimiter)?;
        let (name2, rest) = rest.split_once(" = ").ok_or(ParseError::MissingDelimiter)?;
        let dist: u64 = rest.parse()?;
        let next_index = names.len();
        let index1 = *names.entry(name1).or_insert(next_index);
        let next_index = names.len();
        let index2 = *names.entry(name2).or_insert(next_index);
        grid[(index1, index2)] = dist;
        grid[(index2, index1)] = dist;
    }
    Ok(grid.resize(names.len(), names.len()))
}

#[aoc(day9, part1)]
fn part1(input: &Grid<u64>) -> u64 {
    fn walk(grid: &Grid<u64>, perm: &mut [usize], index: usize) -> u64 {
        if index == perm.len() {
            let mut dist = 0;
            let mut pos = perm[0];
            for &next in &perm[1..] {
                dist += grid[(pos, next)];
                pos = next;
            }
            dist
        } else {
            let mut min_dist = u64::MAX;
            for i in index..perm.len() {
                (perm[index], perm[i]) = (perm[i], perm[index]);
                min_dist = min_dist.min(walk(grid, perm, index + 1));
                (perm[index], perm[i]) = (perm[i], perm[index]);
            }
            min_dist
        }
    }
    let mut perm = (0..input.rows()).collect::<Vec<_>>();
    walk(input, &mut perm, 0)
}

#[aoc(day9, part2)]
fn part2(input: &Grid<u64>) -> u64 {
    fn walk(grid: &Grid<u64>, perm: &mut [usize], index: usize) -> u64 {
        if index == perm.len() {
            let mut dist = 0;
            let mut pos = perm[0];
            for &next in &perm[1..] {
                dist += grid[(pos, next)];
                pos = next;
            }
            dist
        } else {
            let mut max_dist = u64::MIN;
            for i in index..perm.len() {
                (perm[index], perm[i]) = (perm[i], perm[index]);
                max_dist = max_dist.max(walk(grid, perm, index + 1));
                (perm[index], perm[i]) = (perm[i], perm[index]);
            }
            max_dist
        }
    }
    let mut perm = (0..input.rows()).collect::<Vec<_>>();
    walk(input, &mut perm, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141";
        let result = parse(input).unwrap();
        assert_eq!(result.rows(), 3);
        assert_eq!(result.cols(), 3);
        assert_eq!(result[(0,1)], 464);
        assert_eq!(result[(0,2)], 518);
        assert_eq!(result[(1,0)], 464);
        assert_eq!(result[(1,2)], 141);
        assert_eq!(result[(2,0)], 518);
        assert_eq!(result[(2,1)], 141);
    }

    #[test]
    fn test_part_1() {
        let input = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141";
        let grid = parse(input).unwrap();
        let result = part1(&grid);
        assert_eq!(result, 605);
    }

    #[test]
    fn test_part_2() {
        let input = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141";
        let grid = parse(input).unwrap();
        let result = part2(&grid);
        assert_eq!(result, 982);
    }
}