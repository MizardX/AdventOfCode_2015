use std::iter::Sum;
use std::num::ParseIntError;
use std::ops::{Add, Mul};

use thiserror::Error;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Recipe {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Recipe {
    pub const fn all_nonnegative(&self) -> bool {
        self.capacity >= 0 && self.durability >= 0 && self.flavor >= 0 && self.texture >= 0
    }

    pub const fn score(&self) -> i32 {
        self.capacity * self.durability * self.flavor * self.texture
    }
}

impl Mul<i32> for Recipe {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        Self {
            capacity: self.capacity * rhs,
            durability: self.durability * rhs,
            flavor: self.flavor * rhs,
            texture: self.texture * rhs,
            calories: self.calories * rhs,
        }
    }
}

impl Add for Recipe {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            capacity: self.capacity + rhs.capacity,
            durability: self.durability + rhs.durability,
            flavor: self.flavor + rhs.flavor,
            texture: self.texture + rhs.texture,
            calories: self.calories + rhs.calories,
        }
    }
}

impl Sum for Recipe {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |s, r| s + r)
    }
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("Missing delimiter: {0}")]
    MissingDelimiter(&'static str),
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[aoc_generator(day15)]
fn parse(input: &str) -> Result<Vec<Recipe>, ParseError> {
    let mut result = Vec::new();
    for line in input.lines() {
        let (_name, rest) = line
            .split_once(": capacity ")
            .ok_or(ParseError::MissingDelimiter("capacity"))?;
        let (capacity, rest) = rest
            .split_once(", durability ")
            .ok_or(ParseError::MissingDelimiter("durability"))?;
        let capacity = capacity.parse()?;
        let (durability, rest) = rest
            .split_once(", flavor ")
            .ok_or(ParseError::MissingDelimiter("flavor"))?;
        let durability = durability.parse()?;
        let (flavor, rest) = rest
            .split_once(", texture ")
            .ok_or(ParseError::MissingDelimiter("texture"))?;
        let flavor = flavor.parse()?;
        let (texture, calories) = rest
            .split_once(", calories ")
            .ok_or(ParseError::MissingDelimiter("calories"))?;
        let texture = texture.parse()?;
        let calories = calories.parse()?;
        result.push(Recipe {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        });
    }
    Ok(result)
}

#[aoc(day15, part1)]
fn part_1(input: &[Recipe]) -> i32 {
    run(input, false)
}

#[aoc(day15, part2)]
fn part_2(input: &[Recipe]) -> i32 {
    run(input, true)
}

fn run(input: &[Recipe], restrict_calories: bool) -> i32 {
    fn walk(
        recipies: &[Recipe],
        counts: &mut [i32],
        remaining: i32,
        index: usize,
        restrict_calories: bool,
    ) -> i32 {
        if index == recipies.len() - 1 {
            counts[index] = remaining;
            let combined: Recipe = recipies.iter().zip(counts).map(|(&r, &mut c)| r * c).sum();
            if combined.all_nonnegative() && (!restrict_calories || combined.calories == 500) {
                combined.score()
            } else {
                i32::MIN
            }
        } else {
            let mut max = i32::MIN;
            for amount in 0..=remaining {
                counts[index] = amount;
                max = max.max(walk(
                    recipies,
                    counts,
                    remaining - amount,
                    index + 1,
                    restrict_calories,
                ));
            }
            max
        }
    }
    let mut counts = vec![0; input.len()];
    walk(input, &mut counts, 100, 0, restrict_calories)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
"
    .trim_ascii();

    #[test]
    fn test_parse() {
        let result = parse(EXAMPLE).unwrap();
        macro_rules! recipe {
            ($cap:expr, $dur:expr, $flav:expr, $tex:expr, $cal:expr $(,)?) => {
                Recipe {
                    capacity: $cap,
                    durability: $dur,
                    flavor: $flav,
                    texture: $tex,
                    calories: $cal,
                }
            };
        }
        assert_eq!(
            result,
            &[recipe!(-1, -2, 6, 3, 8), recipe!(2, 3, -2, -1, 3)]
        );
    }

    #[test]
    fn test_part_1() {
        let recipies = parse(EXAMPLE).unwrap();
        let result = part_1(&recipies);
        assert_eq!(result, 62_842_880);
    }

    #[test]
    fn test_part_2() {
        let recipies = parse(EXAMPLE).unwrap();
        let result = part_2(&recipies);
        assert_eq!(result, 57_600_000);
    }
}
