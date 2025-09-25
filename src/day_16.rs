use std::num::ParseIntError;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Missing delimiter: {0}")]
    MissingDelimiter(&'static str),
    #[error("Invalid compound")]
    InvalidCompound,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[derive(Debug, Clone, Copy)]
#[repr(usize)]
enum Compound {
    Children,
    Cats,
    Samoyeds,
    Pomeranians,
    Akitas,
    Vizslas,
    Goldfish,
    Trees,
    Cars,
    Perfumes,
}

impl Compound {
    pub const fn all() -> [Self; 10] {
        [
            Self::Children,
            Self::Cats,
            Self::Samoyeds,
            Self::Pomeranians,
            Self::Akitas,
            Self::Vizslas,
            Self::Goldfish,
            Self::Trees,
            Self::Cars,
            Self::Perfumes,
        ]
    }
}

impl FromStr for Compound {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(match value {
            "children" => Self::Children,
            "cats" => Self::Cats,
            "samoyeds" => Self::Samoyeds,
            "pomeranians" => Self::Pomeranians,
            "akitas" => Self::Akitas,
            "vizslas" => Self::Vizslas,
            "goldfish" => Self::Goldfish,
            "trees" => Self::Trees,
            "cars" => Self::Cars,
            "perfumes" => Self::Perfumes,
            _ => Err(ParseError::InvalidCompound)?,
        })
    }
}

#[derive(Debug, Clone)]
struct AuntSue {
    number: usize,
    amounts: [Option<u8>; 10],
}

impl AuntSue {
    pub const fn new(number: usize) -> Self {
        Self {
            number,
            amounts: [None; 10],
        }
    }

    pub const fn with_amounts(amounts: [u8; 10]) -> Self {
        let mut new_amounts = [None; 10];
        let mut i = 0;
        while i < amounts.len() {
            new_amounts[i] = Some(amounts[i]);
            i += 1;
        }
        Self {
            number: 0,
            amounts: new_amounts,
        }
    }
}

impl Index<Compound> for AuntSue {
    type Output = Option<u8>;

    fn index(&self, index: Compound) -> &Self::Output {
        &self.amounts[index as usize]
    }
}

impl IndexMut<Compound> for AuntSue {
    fn index_mut(&mut self, index: Compound) -> &mut Self::Output {
        &mut self.amounts[index as usize]
    }
}

impl FromStr for AuntSue {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        // Sue 221: perfumes: 9, cars: 10, children: 10
        let rest = line
            .strip_prefix("Sue ")
            .ok_or(ParseError::MissingDelimiter("Sue"))?;
        let (number, rest) = rest
            .split_once(": ")
            .ok_or(ParseError::MissingDelimiter("colon"))?;
        let number = number.parse()?;

        let mut aunt = Self::new(number);

        let (first, rest) = rest
            .split_once(", ")
            .ok_or(ParseError::MissingDelimiter("comma"))?;
        let (compound, amount) = first
            .split_once(": ")
            .ok_or(ParseError::MissingDelimiter("colon"))?;
        let compound = compound.parse()?;
        let amount = amount.parse()?;

        aunt[compound] = Some(amount);

        let (second, third) = rest
            .split_once(", ")
            .ok_or(ParseError::MissingDelimiter("comma"))?;
        let (compound, amount) = second
            .split_once(": ")
            .ok_or(ParseError::MissingDelimiter("colon"))?;
        let compound = compound.parse()?;
        let amount = amount.parse()?;

        aunt[compound] = Some(amount);

        let (compound, amount) = third
            .split_once(": ")
            .ok_or(ParseError::MissingDelimiter("colon"))?;
        let compound = compound.parse()?;
        let amount = amount.parse()?;

        aunt[compound] = Some(amount);

        Ok(aunt)
    }
}

#[aoc_generator(day16)]
fn parse(input: &str) -> Result<Vec<AuntSue>, ParseError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day16, part1)]
fn part_1(aunts: &[AuntSue]) -> usize {
    run(aunts, |_, target, item| item == target)
}

#[aoc(day16, part2)]
fn part_2(aunts: &[AuntSue]) -> usize {
    run(aunts, |compound, target, item| match compound {
        Compound::Cats | Compound::Trees => item > target,
        Compound::Pomeranians | Compound::Goldfish => item < target,
        _ => item == target,
    })
}

fn run(aunts: &[AuntSue], matcher: fn(Compound, u8, u8) -> bool) -> usize {
    let target = AuntSue::with_amounts([3, 7, 2, 3, 0, 0, 5, 3, 2, 1]);
    aunts
        .iter()
        .find_map(|item| {
            Compound::all()
                .iter()
                .all(|&compound| {
                    let Some(item) = item[compound] else {
                        return true;
                    };
                    let target = target[compound].unwrap();
                    matcher(compound, target, item)
                })
                .then_some(item.number)
        })
        .unwrap()
}

// No test cases provided in problem statement
