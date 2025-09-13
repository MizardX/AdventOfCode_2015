use std::collections::HashSet;

#[aoc(day3, part1)]
fn solve_part_1(input: &[u8]) -> usize {
    let mut seen = HashSet::with_capacity(input.len());
    let mut x = 0;
    let mut y = 0;
    seen.insert((x, y));
    for &ch in input {
        match ch {
            b'<' => x -= 1,
            b'>' => x += 1,
            b'^' => y -= 1,
            b'v' => y += 1,
            _ => continue,
        }
        seen.insert((x, y));
    }
    seen.len()
}

#[aoc(day3, part2)]
fn solve_part_2(input: &[u8]) -> usize {
    let mut seen = HashSet::with_capacity(input.len());
    let mut x1 = 0;
    let mut y1 = 0;
    let mut x2 = 0;
    let mut y2 = 0;
    seen.insert((x1, y1));
    for &ch in input {
        match ch {
            b'<' => x1 -= 1,
            b'>' => x1 += 1,
            b'^' => y1 -= 1,
            b'v' => y1 += 1,
            _ => continue,
        }
        seen.insert((x1, y1));
        (x1, y1, x2, y2) = (x2, y2, x1, y1);
    }
    seen.len()
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(b">" => 2)]
    #[test_case(b"^>v<" => 4)]
    #[test_case(b"^v^v^v^v^v" => 2)]
    fn test_part_1(input: &[u8]) -> usize {
        solve_part_1(input)
    }

    #[test_case(b"^v" => 3; "up down")]
    #[test_case(b"^>v<" => 3)]
    #[test_case(b"^v^v^v^v^v" => 11)]
    fn test_part_2(input: &[u8]) -> usize {
        solve_part_2(input)
    }
}
