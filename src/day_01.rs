#[aoc(day1, part1)]
fn solve_part_1(input: &[u8]) -> i32 {
    input
        .iter()
        .map(|&ch| match ch {
            b'(' => 1,
            b')' => -1,
            _ => 0,
        })
        .sum()
}

#[aoc(day1, part2)]
fn solve_part_2(input: &[u8]) -> usize {
    input
        .iter()
        .scan(1, |s, &ch| {
            *s += match ch {
                b'(' => 1,
                b')' => -1,
                _ => 0,
            };
            (*s > 0).then_some(())
        })
        .count()
        + 1 // +1 since we start counting at 0 instead of 1
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(b"(())" => 0; "example 1a")]
    #[test_case(b"()()" => 0; "example 1b")]
    #[test_case(b"(((" => 3; "example 2a")]
    #[test_case(b"(()(()(" => 3; "example 2b")]
    #[test_case(b"))(((((" => 3; "example 3")]
    #[test_case(b"())" => -1; "example 4a")]
    #[test_case(b"))(" => -1; "example 4b")]
    #[test_case(b")))" => -3; "example 5a")]
    #[test_case(b")())())" => -3; "example 5b")]
    fn test_part_1(input: &[u8]) -> i32 {
        solve_part_1(input)
    }

    #[test_case(b")" => 1; "example 6")]
    #[test_case(b"()())" => 5; "example 7")]
    fn test_part_2(input: &[u8]) -> usize {
        solve_part_2(input)
    }
}
