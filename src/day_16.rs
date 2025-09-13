use std::num::ParseIntError;
use std::ops::{Bound, Index, IndexMut, RangeBounds};
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
}

trait SueMatcher {
    fn matches(target: &AuntSue, item: &AuntSue) -> bool;
}

struct Exact;
impl SueMatcher for Exact {
    fn matches(target: &AuntSue, item: &AuntSue) -> bool {
        for compound in Compound::all() {
            let Some(a) = target[compound] else { continue };
            let Some(b) = item[compound] else { continue };
            if a != b {
                return false;
            }
        }
        true
    }
}

struct Ranged;
impl SueMatcher for Ranged {
    fn matches(target: &AuntSue, item: &AuntSue) -> bool {
        for compound in Compound::all() {
            let Some(a) = target[compound] else { continue };
            let Some(b) = item[compound] else { continue };
            let range = match compound {
                Compound::Cats | Compound::Trees => (Bound::Excluded(a), Bound::Unbounded),
                Compound::Pomeranians | Compound::Goldfish => (Bound::Unbounded, Bound::Excluded(a)),
                _ => (Bound::Included(a), Bound::Included(a)),
            };
            if !range.contains(&b) {
                return false;
            }
        }
        true
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
    run::<Exact>(aunts)
}

#[aoc(day16, part2)]
fn part_2(aunts: &[AuntSue]) -> usize {
    run::<Ranged>(aunts)
}

fn run<M: SueMatcher>(aunts: &[AuntSue]) -> usize {
    let mut target = AuntSue::new(0);
    for (compound, amout) in Compound::all()
        .into_iter()
        .zip([3, 7, 2, 3, 0, 0, 5, 3, 2, 1])
    {
        target[compound] = Some(amout);
    }
    let matching_aunt = aunts.iter().find(|&a| M::matches(&target, a)).unwrap();
    matching_aunt.number
}

// No test cases provided in problem statement
