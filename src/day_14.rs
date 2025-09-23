use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Raindeer {
    velocity: i64,
    active: i64,
    recover: i64,
}

impl Raindeer {
    pub const fn project(&self, time: i64) -> i64 {
        let whole_cycles = time / (self.active + self.recover);
        let remaining = time % (self.active + self.recover);
        if remaining < self.active {
            (whole_cycles * self.active + remaining) * self.velocity
        } else {
            (whole_cycles + 1) * self.active * self.velocity
        }
    }
}

impl Raindeer {
    pub const fn new(velocity: i64, duration: i64, sleep: i64) -> Self {
        Self {
            velocity,
            active: duration,
            recover: sleep,
        }
    }
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("Missing delimiter: {0}")]
    MissingDelimiter(&'static str),
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[aoc_generator(day14)]
fn parse(input: &str) -> Result<Vec<Raindeer>, ParseError> {
    let mut result = Vec::new();
    for line in input.lines() {
        let (_name, rest) = line
            .split_once(" can fly ")
            .ok_or(ParseError::MissingDelimiter("can fly"))?;
        let (vel, rest) = rest
            .split_once(" km/s for ")
            .ok_or(ParseError::MissingDelimiter("km/s for"))?;
        let velocity = vel.parse()?;
        let (dur, rest) = rest.split_once(" seconds, but then must rest for ").ok_or(
            ParseError::MissingDelimiter("seconds, but then must rest for"),
        )?;
        let duration = dur.parse()?;
        let slp = rest
            .strip_suffix(" seconds.")
            .ok_or(ParseError::MissingDelimiter("seconds"))?;
        let sleep = slp.parse()?;
        result.push(Raindeer::new(velocity, duration, sleep));
    }
    Ok(result)
}

#[aoc(day14, part1)]
fn part_1(input: &[Raindeer]) -> i64 {
    simple_rules(input, 2503)
}

fn simple_rules(input: &[Raindeer], time: i64) -> i64 {
    input.iter().map(|r| r.project(time)).max().unwrap()
}

#[aoc(day14, part2)]
fn part_2(input: &[Raindeer]) -> i64 {
    advanced_rules(input, 2503)
}

fn advanced_rules(input: &[Raindeer], time: i64) -> i64 {
    let n = input.len();
    let mut scores = vec![0; n];
    let mut in_lead = vec![];
    for t in 1..=time {
        let mut lead_pos = i64::MIN;
        for (index, raindeer) in input.iter().enumerate() {
            let position = raindeer.project(t);
            if position > lead_pos {
                in_lead.clear();
                lead_pos = position;
            }
            if position == lead_pos {
                in_lead.push(index);
            }
        }
        for &i in &in_lead {
            scores[i] += 1;
        }
    }
    scores.iter().copied().max().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "
Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
"
    .trim_ascii();

    #[test]
    fn test_parse() {
        let result = parse(EXAMPLE).unwrap();

        assert_eq!(
            result,
            &[
                Raindeer {
                    velocity: 14,
                    active: 10,
                    recover: 127
                },
                Raindeer {
                    velocity: 16,
                    active: 11,
                    recover: 162
                }
            ]
        );
    }

    #[test]
    fn test_part_1() {
        let input = parse(EXAMPLE).unwrap();
        let result = simple_rules(&input, 1000);
        assert_eq!(result, 1120);
    }

    #[test]
    fn test_part_2() {
        let input = parse(EXAMPLE).unwrap();
        let result = advanced_rules(&input, 1000);
        assert_eq!(result, 689);
    }
}
