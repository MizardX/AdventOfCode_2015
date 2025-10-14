use std::num::ParseIntError;

#[aoc_generator(day17)]
fn parse(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day17, part1)]
fn part_1(input: &[u32]) -> usize {
    run_part_1(150, input)
}

fn run_part_1(volume: u32, containers: &[u32]) -> usize {
    let volume = volume as usize;
    let mut dp = vec![0; volume + 1];
    dp[0] = 1;
    for &vol in containers {
        let vol = vol as usize;
        for i in (vol..=volume).rev() {
            dp[i] += dp[i - vol];
        }
    }
    dp[volume]
}

#[aoc(day17, part2)]
fn part_2(containers: &[u32]) -> usize {
    run_part_2(150, containers)
}

fn run_part_2(volume: u32, containers: &[u32]) -> usize {
    let volume = volume as usize;
    let mut dp = vec![0; containers.len() * (volume + 1)];
    macro_rules! dp {
        [$vol:expr, $count:expr] => { dp[($vol)*containers.len() + ($count)] }
    }
    dp![0, 0] = 1;
    for &vol in containers {
        let vol = vol as usize;
        for count in (1..containers.len()).rev() {
            for i in (vol..=volume).rev() {
                dp![i, count] += dp![i - vol, count - 1];
            }
        }
    }
    (1..containers.len())
        .map(|c| dp![volume, c])
        .find(|&x| x > 0)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        20\n\
        15\n\
        10\n\
        5\n\
        5\
    ";

    #[test]
    fn test_parse() {
        let result = parse(EXAMPLE).unwrap();
        assert_eq!(result, &[20, 15, 10, 5, 5]);
    }

    #[test]
    fn test_part_1() {
        let containers = parse(EXAMPLE).unwrap();
        let volume = 25;

        let result = run_part_1(volume, &containers);

        assert_eq!(result, 4);
    }

    #[test]
    fn test_part_2() {
        let containers = parse(EXAMPLE).unwrap();
        let volume = 25;

        let result = run_part_2(volume, &containers);

        assert_eq!(result, 3);
    }
}
