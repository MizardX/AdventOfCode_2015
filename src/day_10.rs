#[aoc(day10, part1)]
fn part_1(input: &[u8]) -> usize {
    run(input, 40, 1 << 19)
}

#[aoc(day10, part2)]
fn part_2(input: &[u8]) -> usize {
    run(input, 50, 1 << 23)
}

fn run(input: &[u8], turns: usize, capacity: usize) -> usize {
    let mut curr = Vec::with_capacity(capacity);
    curr.extend_from_slice(input);
    for ch in &mut curr {
        *ch -= b'0';
    }
    let mut next = Vec::with_capacity(capacity);
    for _ in 0..turns {
        for chunk in curr.chunk_by(u8::eq) {
            let dig = chunk[0];
            next.push(u8::try_from(chunk.len()).expect("too many repeats"));
            next.push(dig);
        }
        (next, curr) = (curr, next);
        next.clear();
    }
    curr.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(b"1", 1 => 2)]
    #[test_case(b"1", 2 => 2)]
    #[test_case(b"1", 3 => 4)]
    #[test_case(b"1", 4 => 6)]
    #[test_case(b"1", 5 => 6)]
    fn test_run(input: &[u8], turns: usize) -> usize {
        run(input, turns, 64)
    }
}
