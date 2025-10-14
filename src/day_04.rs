#[aoc(day4, part1)]
fn part_1(input: &[u8]) -> u32 {
    let mut root = md5::Context::new();
    root.consume(input.trim_ascii());
    for x in 1_u32.. {
        let mut ctx = root.clone();
        ctx.consume(format!("{x}"));
        let hash = ctx.finalize().0;
        if matches!(hash, [0, 0, b, ..] if b >> 4 == 0) {
            return x;
        }
    }
    0
}

#[aoc(day4, part2)]
fn part_2(input: &[u8]) -> u32 {
    let mut root = md5::Context::new();
    root.consume(input.trim_ascii());
    for x in 1_u32.. {
        let mut ctx = root.clone();
        ctx.consume(format!("{x}"));
        let hash = ctx.finalize().0;
        if matches!(hash, [0, 0, 0, ..]) {
            return x;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(b"abcdef" => 609_043)]
    #[test_case(b"pqrstuv" => 1_048_970)]
    fn test_part_1(input: &[u8]) -> u32 {
        part_1(input)
    }

    #[test_case(b"abcdef" => 6_742_839)]
    #[test_case(b"pqrstuv" => 5_714_438)]
    fn test_part_2(input: &[u8]) -> u32 {
        part_2(input)
    }
}
