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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Flying,
    Recovering,
}

#[derive(Debug, Clone, Copy)]
struct Player {
    position: i64,
    score: i64,
    action: Action,
    until_time: i64,
    index: usize,
}

fn advanced_rules(input: &[Raindeer], time: i64) -> i64 {
    let n = input.len();
    let mut players = input
        .iter()
        .enumerate()
        .map(|(index, raindeer)| Player {
            position: 0,
            score: 0,
            action: Action::Flying,
            until_time: raindeer.active,
            index,
        })
        .collect::<Vec<_>>();
    for t in 0..=time {
        for p in &mut players {
            if p.until_time == t {
                match p.action {
                    Action::Flying => {
                        p.action = Action::Recovering;
                        p.until_time = t + input[p.index].recover;
                    }
                    Action::Recovering => {
                        p.action = Action::Flying;
                        p.until_time = t + input[p.index].active;
                    }
                }
            }
            if p.action == Action::Flying {
                p.position += input[p.index].velocity;
            }
        }
        players.sort_unstable_by_key(|p| (p.position, input[p.index].velocity));
        let max_position = players[n - 1].position;
        for p in &mut players {
            if p.position == max_position {
                p.score += 1;
            }
        }
    }
    players.iter().map(|p| p.score).max().unwrap()
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
