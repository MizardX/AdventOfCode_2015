use std::num::ParseIntError;

#[aoc_generator(day20)]
fn parse(input: &str) -> Result<usize, ParseIntError> {
    input.parse()
}

#[aoc(day20, part1)]
#[allow(clippy::trivially_copy_pass_by_ref, reason = "aoc lib requires a reference")]
fn part_1(&target: &usize) -> usize {
    let mut dp = vec![0; 1_000_000];
    for elf in 1..dp.len() {
        for house in (elf..dp.len()).step_by(elf) {
            dp[house] += 10 * elf;
        }
        if dp[elf] >= target {
            return elf;
        }
    }
    0
}

#[aoc(day20, part2)]
#[allow(clippy::trivially_copy_pass_by_ref, reason = "aoc lib requires a reference")]
fn part_2(&target: &usize) -> usize {
    let mut dp = vec![0; 1_000_000];
    for elf in 1..dp.len() {
        for house in (elf..dp.len()).step_by(elf).take(50) {
            dp[house] += 11 * elf;
        }
        if dp[elf] >= target {
            return elf;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(10 => 1)]
    #[test_case(70 => 4)]
    fn test_part_1(target: usize) -> usize {
        part_1(&target)
    }
}
