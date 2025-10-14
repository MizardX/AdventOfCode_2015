use std::collections::HashSet;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Not recognized as an atom")]
    InvalidAtom,
    #[error("Missing delimiter")]
    MissingDelimiter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Atom {
    Empty,
    Single(u8),
    Double(u8, u8),
}

impl TryFrom<&[u8]> for Atom {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(match value {
            [b'e'] => Self::Empty,
            [a @ b'A'..=b'Z'] => Self::Single(*a),
            [a @ b'A'..=b'Z', b @ b'a'..=b'z'] => Self::Double(*a, *b),
            _ => Err(ParseError::InvalidAtom)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rule {
    source: Atom,
    target: Vec<Atom>,
}

impl TryFrom<&[u8]> for Rule {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let arrow = (0..value.len())
            .find(|&i| value[i..].starts_with(b" => "))
            .ok_or(ParseError::MissingDelimiter)?;
        let source: Atom = value[..arrow].try_into()?;
        let target = value[arrow + 4..]
            .chunk_by(|_, b| u8::is_ascii_lowercase(b))
            .map(Atom::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { source, target })
    }
}

#[derive(Debug)]
struct Schema {
    rules: Vec<Rule>,
    target: Vec<Atom>,
}

impl TryFrom<&[u8]> for Schema {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut lines = value.split(|&ch| ch == b'\n');
        let mut rules = Vec::new();
        for line in &mut lines {
            if line.is_empty() {
                break;
            }
            rules.push(line.try_into()?);
        }
        let last_line = lines.next().ok_or(ParseError::MissingDelimiter)?;
        let target = last_line
            .chunk_by(|_, b| u8::is_ascii_lowercase(b))
            .map(Atom::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { rules, target })
    }
}

#[aoc_generator(day19)]
fn parse(input: &[u8]) -> Result<Schema, ParseError> {
    input.try_into()
}

#[aoc(day19, part1)]
fn part_1(input: &Schema) -> usize {
    let mut seen = HashSet::new();
    for start in 0..input.target.len() {
        for rule in &input.rules {
            if input.target[start..].starts_with(&[rule.source]) {
                let mut constructed =
                    Vec::with_capacity(input.target.len() + rule.target.len() - 1);
                constructed.extend_from_slice(&input.target[..start]);
                constructed.extend_from_slice(&rule.target);
                constructed.extend_from_slice(&input.target[start + 1..]);
                seen.insert(constructed);
            }
        }
    }
    seen.len()
}

#[aoc(day19, part2)]
fn part_2(input: &Schema) -> usize {
    // There are two kinds of rules:
    // 1) X => XX,
    // 2) X => X(X) | X(X,X) | X(X,X,X)
    // To figure out number of rule applications, you just need to count number of open parens or commas.
    // Except, we need to figure out which atoms act like open paren and comma.
    let (open, separator) = input
        .rules
        .iter()
        .find_map(|r| {
            if let &[_, op, _, sep, _, ..] = r.target.as_slice() {
                Some((op, sep))
            } else {
                None
            }
        })
        // The examples don't have those, so just default to Empty.
        .unwrap_or((Atom::Empty, Atom::Empty));

    // If there is a rule `e => X` (not `e => XX`), we don't have to subtract one.
    let empty_to_singles = input
        .rules
        .iter()
        .any(|r| r.source == Atom::Empty && r.target.len() == 1);

    let mut count = 0;
    for &atom in &input.target {
        if atom == open || atom == separator {
            count -= 1;
        } else {
            count += 1;
        }
    }
    if !empty_to_singles {
        count -= 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const EXAMPLE1: &[u8] = b"\
        e => H\n\
        e => O\n\
        H => HO\n\
        H => OH\n\
        O => HH\n\
        \n\
        HOH\
    ";

    const EXAMPLE2: &[u8] = b"\
        e => H\n\
        e => O\n\
        H => HO\n\
        H => OH\n\
        O => HH\n\
        \n\
        HOHOHO\
    ";

    #[test]
    fn test_parse() {
        let result = parse(EXAMPLE1).unwrap();

        macro_rules! rule {
            ($src:expr => $($target:expr),* $(,)?) => {
                Rule { source: $src, target: vec![$($target),*] }
            };
        }

        assert_eq!(
            result.rules,
            &[
                rule!(Atom::Empty => Atom::Single(b'H')),
                rule!(Atom::Empty => Atom::Single(b'O')),
                rule!(Atom::Single(b'H') => Atom::Single(b'H'), Atom::Single(b'O')),
                rule!(Atom::Single(b'H') => Atom::Single(b'O'), Atom::Single(b'H')),
                rule!(Atom::Single(b'O') => Atom::Single(b'H'), Atom::Single(b'H')),
            ]
        );
        assert_eq!(
            result.target,
            &[Atom::Single(b'H'), Atom::Single(b'O'), Atom::Single(b'H')]
        );
    }

    #[test_case(EXAMPLE1 => 4)]
    fn test_part_1(input: &[u8]) -> usize {
        let schema = parse(input).unwrap();
        part_1(&schema)
    }

    #[test_case(EXAMPLE1 => 3)]
    #[test_case(EXAMPLE2 => 6)]
    fn test_part_2(input: &[u8]) -> usize {
        let schema = parse(input).unwrap();
        part_2(&schema)
    }
}
