use std::num::ParseIntError;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Reg {
    A,
    B,
}

impl FromStr for Reg {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "a" => Self::A,
            "b" => Self::B,
            _ => return Err(ParseError::UnknownReg(s.to_string())),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Hlf(Reg),
    Tpl(Reg),
    Inc(Reg),
    Jmp(isize),
    Jie(Reg, isize),
    Jio(Reg, isize),
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("Unknown operation")]
    UnknownOp(String),
    #[error("Unknown register")]
    UnknownReg(String),
    #[error("Syntax error")]
    SyntaxError(&'static str),
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

impl FromStr for Op {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            match s
                .split_once(' ')
                .ok_or(ParseError::SyntaxError("no space"))?
            {
                ("hlf", arg) => Self::Hlf(arg.parse()?),
                ("tpl", arg) => Self::Tpl(arg.parse()?),
                ("inc", arg) => Self::Inc(arg.parse()?),
                ("jmp", arg) => Self::Jmp(arg.parse()?),
                ("jie", args) => {
                    let (reg, off) = args
                        .split_once(", ")
                        .ok_or(ParseError::SyntaxError("no comma"))?;
                    Self::Jie(reg.parse()?, off.parse()?)
                }
                ("jio", args) => {
                    let (reg, off) = args
                        .split_once(", ")
                        .ok_or(ParseError::SyntaxError("no comma"))?;
                    Self::Jio(reg.parse()?, off.parse()?)
                }
                _ => Err(ParseError::UnknownOp(s.to_string()))?,
            },
        )
    }
}

struct Machine {
    registers: [u64; 2],
    ip: usize,
    instructions: Vec<Op>,
}

impl Machine {
    const fn new(instructions: Vec<Op>) -> Self {
        Self {
            registers: [0; 2],
            ip: 0,
            instructions,
        }
    }

    fn run(&mut self) {
        while self.ip < self.instructions.len() {
            self.step();
        }
    }

    fn step(&mut self) {
        match self.instructions[self.ip] {
            Op::Hlf(reg) => self[reg] /= 2,
            Op::Tpl(reg) => self[reg] *= 3,
            Op::Inc(reg) => self[reg] += 1,
            Op::Jmp(delta) => {
                self.try_jump(delta);
                return;
            }
            Op::Jie(reg, delta) => {
                if self[reg] & 1 == 0 {
                    self.try_jump(delta);
                    return;
                }
            }
            Op::Jio(reg, delta) => {
                if self[reg] == 1 {
                    self.try_jump(delta);
                    return;
                }
            }
        }
        self.ip += 1;
    }

    const fn try_jump(&mut self, delta: isize) {
        if let Some(new_ip) = self.ip.checked_add_signed(delta)
            && new_ip < self.instructions.len()
        {
            self.ip = new_ip;
        } else {
            self.ip = self.instructions.len();
        }
    }
}

impl Index<Reg> for Machine {
    type Output = u64;

    fn index(&self, reg: Reg) -> &Self::Output {
        &self.registers[reg as usize]
    }
}

impl IndexMut<Reg> for Machine {
    fn index_mut(&mut self, reg: Reg) -> &mut Self::Output {
        &mut self.registers[reg as usize]
    }
}

#[aoc_generator(day23)]
fn parse(input: &str) -> Result<Vec<Op>, ParseError> {
    input.lines().map(str::parse).collect::<Result<_, _>>()
}

#[aoc(day23, part1)]
fn part_1(ops: &[Op]) -> u64 {
    let mut machine = Machine::new(ops.to_vec());
    machine.run();
    machine[Reg::B]
}

#[aoc(day23, part2)]
fn part_2(ops: &[Op]) -> u64 {
    let mut machine = Machine::new(ops.to_vec());
    machine[Reg::A] = 1;
    machine.run();
    machine[Reg::B]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
inc a
jio a, +2
tpl a
inc a
"
    .trim_ascii();

    #[test]
    fn test_parse() {
        let result = parse(EXAMPLE).unwrap();

        assert_eq!(
            result,
            &[
                Op::Inc(Reg::A),
                Op::Jio(Reg::A, 2),
                Op::Tpl(Reg::A),
                Op::Inc(Reg::A),
            ],
        );
    }

    #[test]
    fn test_part_1() {
        let ops = parse(EXAMPLE).unwrap();
        let mut machine = Machine::new(ops);
        machine.run();
        assert_eq!(machine[Reg::A], 2);
    }
}
