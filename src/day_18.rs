use thiserror::Error;

use crate::utils::Grid;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Input is not rectangular")]
    ShapeError,
}

#[aoc_generator(day18)]
fn parse(input: &[u8]) -> Result<Grid<bool>, ParseError> {
    #[allow(clippy::naive_bytecount)]
    let rows = input.iter().filter(|&&ch| ch == b'\n').count() + 1;
    let cols = input
        .iter()
        .position(|&ch| ch == b'\n')
        .ok_or(ParseError::ShapeError)?;
    let mut grid = Grid::new(rows, cols);
    for (r, line) in input.split(|&ch| ch == b'\n').enumerate() {
        if line.len() != cols {
            return Err(ParseError::ShapeError);
        }
        for (c, &ch) in line.iter().enumerate() {
            grid[(r, c)] = ch == b'#';
        }
    }
    Ok(grid)
}

#[aoc(day18, part1)]
fn part_1(grid: &Grid<bool>) -> usize {
    run(grid, 100, false)
}

#[aoc(day18, part2)]
fn part_2(grid: &Grid<bool>) -> usize {
    run(grid, 100, true)
}

fn run(grid: &Grid<bool>, steps: usize, fixed_corners: bool) -> usize {
    let (rows, cols) = (grid.rows(), grid.cols());
    let mut read = grid.clone();
    let mut write = Grid::<bool>::new(rows, cols);
    let mut num_alive = 0;
    if fixed_corners {
        for (r, c) in [(0, 0), (rows - 1, 0), (rows - 1, cols - 1), (0, cols - 1)] {
            read[(r, c)] = true;
        }
    }
    for _ in 0..steps {
        num_alive = 0;
        for r in 0..rows {
            for c in 0..cols {
                let living = read[(r, c)];
                let mut alive = 0;
                for r1 in r.saturating_sub(1)..(r + 2).min(rows) {
                    for c1 in c.saturating_sub(1)..(c + 2).min(cols) {
                        if (r1, c1) != (r, c) && read[(r1, c1)] {
                            alive += 1;
                        }
                    }
                }
                let new_state = matches!((living, alive), (true, 2 | 3) | (false, 3));
                write[(r, c)] = new_state;
                num_alive += usize::from(new_state);
            }
        }
        if fixed_corners {
            for (r, c) in [(0, 0), (rows - 1, 0), (rows - 1, cols - 1), (0, cols - 1)] {
                if !write[(r, c)] {
                    write[(r, c)] = true;
                    num_alive += 1;
                }
            }
        }
        (read, write) = (write, read);
    }
    num_alive
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const EXAMPLE: &[u8] = b"\
        .#.#.#\n\
        ...##.\n\
        #....#\n\
        ..#...\n\
        #.#..#\n\
        ####..\
    ";

    #[test]
    fn test_parse() {
        let expected = [
            [0, 1, 0, 1, 0, 1],
            [0, 0, 0, 1, 1, 0],
            [1, 0, 0, 0, 0, 1],
            [0, 0, 1, 0, 0, 0],
            [1, 0, 1, 0, 0, 1],
            [1, 1, 1, 1, 0, 0],
        ];

        let result = parse(EXAMPLE).unwrap();
        assert_eq!(result.rows(), expected.len());
        assert_eq!(result.cols(), expected[0].len());
        for (r, row) in expected.iter().enumerate() {
            for (c, &cell) in row.iter().enumerate() {
                assert_eq!(result[(r, c)], cell == 1);
            }
        }
    }

    #[test_case(1 => 11)]
    #[test_case(2 => 8)]
    #[test_case(3 => 4)]
    #[test_case(4 => 4)]
    fn test_part_1(steps: usize) -> usize {
        let grid = parse(EXAMPLE).unwrap();
        run(&grid, steps, false)
    }

    #[test_case(1 => 18)]
    #[test_case(2 => 18)]
    #[test_case(3 => 18)]
    #[test_case(4 => 14)]
    #[test_case(5 => 17)]
    fn test_part_2(steps: usize) -> usize {
        let grid = parse(EXAMPLE).unwrap();
        run(&grid, steps, true)
    }
}
