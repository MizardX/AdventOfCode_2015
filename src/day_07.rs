use std::collections::HashMap;
use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Gate {
    Constant(u16),
    Copy(usize),
    And(usize, usize),
    Or(usize, usize),
    Not(usize),
    LShift(usize, u8),
    RShift(usize, u8),
}

impl Gate {
    pub fn evaluate(&self, values: &[Option<u16>]) -> Result<u16, usize> {
        Ok(match self {
            &Gate::Constant(x) => x,
            &Gate::Copy(a) => values[a].ok_or(a)?,
            &Gate::And(a, b) => values[a].ok_or(a)? & values[b].ok_or(b)?,
            &Gate::Or(a, b) => values[a].ok_or(a)? | values[b].ok_or(b)?,
            &Gate::Not(a) => !values[a].ok_or(a)?,
            &Gate::LShift(a, x) => values[a].ok_or(a)? << x,
            &Gate::RShift(a, x) => values[a].ok_or(a)? >> x,
        })
    }
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("Missing delimiter")]
    MissingDelimiter,
    #[error("Unknown name")]
    UnknownName,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[derive(Debug, PartialEq, Eq)]
struct Circuit {
    gates: Vec<Gate>,
    a_ix: Option<usize>,
    b_ix: Option<usize>,
}

impl Circuit {
    pub fn new(gates: Vec<Gate>, a_ix: Option<usize>, b_ix: Option<usize>) -> Self {
        Self { gates, a_ix, b_ix }
    }

    pub fn evaluate(&self, values: &mut [Option<u16>]) {
        let n = self.gates.len();
        assert!(values.len() >= n);
        let mut waiting_for = vec![vec![]; n];
        let mut pending: Vec<_> = (0..n).collect();
        while let Some(ix) = pending.pop() {
            if values[ix].is_some() {
                continue;
            }
            let gate = &self.gates[ix];
            match gate.evaluate(&values) {
                Ok(res) => {
                    values[ix] = Some(res);
                    pending.extend_from_slice(&waiting_for[ix]);
                    waiting_for[ix].clear();
                }
                Err(block) => waiting_for[block].push(ix),
            }
        }
    }
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Result<Circuit, ParseError> {
    let mut gates = Vec::new();
    let mut names = HashMap::new();
    for line in input.lines() {
        let (_, name) = line
            .split_once(" -> ")
            .ok_or(ParseError::MissingDelimiter)?;
        names.insert(name, gates.len());
        gates.push(Gate::Constant(0));
    }
    for (ix, line) in input.lines().enumerate() {
        let (expr, _) = line
            .split_once(" -> ")
            .ok_or(ParseError::MissingDelimiter)?;
        let gate = if let Some(a) = expr.strip_prefix("NOT ") {
            let a_ix = if let Ok(a_val) = a.parse() {
                let a_ix = gates.len();
                gates.push(Gate::Constant(a_val));
                a_ix
            } else {
                names.get(a).copied().ok_or(ParseError::UnknownName)?
            };
            Gate::Not(a_ix)
        } else if let Some((a, b)) = expr.split_once(" AND ") {
            let a_ix = if let Ok(a_val) = a.parse() {
                let a_ix = gates.len();
                gates.push(Gate::Constant(a_val));
                a_ix
            } else {
                names.get(a).copied().ok_or(ParseError::UnknownName)?
            };
            let b_ix = if let Ok(b_val) = b.parse() {
                let b_ix = gates.len();
                gates.push(Gate::Constant(b_val));
                b_ix
            } else {
                names.get(b).copied().ok_or(ParseError::UnknownName)?
            };
            Gate::And(a_ix, b_ix)
        } else if let Some((a, b)) = expr.split_once(" OR ") {
            let a_ix = if let Ok(a_val) = a.parse() {
                let a_ix = gates.len();
                gates.push(Gate::Constant(a_val));
                a_ix
            } else {
                names.get(a).copied().ok_or(ParseError::UnknownName)?
            };
            let b_ix = if let Ok(b_val) = b.parse() {
                let b_ix = gates.len();
                gates.push(Gate::Constant(b_val));
                b_ix
            } else {
                names.get(b).copied().ok_or(ParseError::UnknownName)?
            };
            Gate::Or(a_ix, b_ix)
        } else if let Some((a, x)) = expr.split_once(" LSHIFT ") {
            let a_ix = if let Ok(a_val) = a.parse() {
                let a_ix = gates.len();
                gates.push(Gate::Constant(a_val));
                a_ix
            } else {
                names.get(a).copied().ok_or(ParseError::UnknownName)?
            };
            Gate::LShift(a_ix, x.parse()?)
        } else if let Some((a, x)) = expr.split_once(" RSHIFT ") {
            let a_ix = if let Ok(a_val) = a.parse() {
                let a_ix = gates.len();
                gates.push(Gate::Constant(a_val));
                a_ix
            } else {
                names.get(a).copied().ok_or(ParseError::UnknownName)?
            };
            Gate::RShift(
                a_ix,
                x.parse()?,
            )
        } else if let Ok(x) = expr.parse() {
            Gate::Constant(x)
        } else {
            Gate::Copy(names.get(expr).copied().ok_or(ParseError::UnknownName)?)
        };
        gates[ix] = gate;
    }
    Ok(Circuit::new(
        gates,
        names.get("a").copied(),
        names.get("b").copied(),
    ))
}

#[aoc(day7, part1)]
fn part1(circuit: &Circuit) -> Option<u16> {
    let mut values = vec![None; circuit.gates.len()];
    let res = run(circuit, &mut values);
    res
}
fn run(circuit: &Circuit, values: &mut [Option<u16>]) -> Option<u16> {
    circuit.evaluate(values);
    values[circuit.a_ix?]
}

#[aoc(day7, part2)]
fn part2(circuit: &Circuit) -> Option<u16> {
    let mut values = vec![None; circuit.gates.len()];
    let a_value = run(circuit, &mut values);
    values.fill(None);
    values[circuit.b_ix?] = a_value;
    run(circuit, &mut values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i";
        let result = parse(input).unwrap();
        assert_eq!(
            result,
            Circuit::new(
                vec![
                    Gate::Constant(123),
                    Gate::Constant(456),
                    Gate::And(0, 1),
                    Gate::Or(0, 1),
                    Gate::LShift(0, 2),
                    Gate::RShift(1, 2),
                    Gate::Not(0),
                    Gate::Not(1)
                ],
                None,
                None
            )
        );
    }

    #[test]
    fn test_part_1() {
        let input = "123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i";
        let circuit = parse(input).unwrap();
        let mut values = vec![None; circuit.gates.len()];
        run(&circuit, &mut values);
        assert_eq!(
            &values,
            &[123, 456, 72, 507, 492, 114, 65412, 65079].map(Some)
        );
    }
}
