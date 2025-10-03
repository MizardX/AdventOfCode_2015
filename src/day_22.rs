use std::collections::{BinaryHeap, HashSet};
use std::hash::Hash;
use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

fn parse_line<T>(line: Option<&str>, prefix: &str) -> Result<T, ParseError>
where
    T: FromStr,
    ParseError: From<T::Err>,
{
    Ok(line
        .ok_or(ParseError::SyntaxError)?
        .strip_prefix(prefix)
        .ok_or(ParseError::SyntaxError)?
        .parse()?)
}

#[aoc_generator(day22)]
fn parse(input: &str) -> Result<Boss, ParseError> {
    let mut lines = input.lines();
    let hit_points = parse_line(lines.next(), "Hit Points: ")?;
    let damage = parse_line(lines.next(), "Damage: ")?;
    if lines.next().is_some() {
        return Err(ParseError::SyntaxError);
    }
    Ok(Boss::new(hit_points, damage))
}

#[aoc(day22, part1)]
fn part_1(boss: &Boss) -> u64 {
    Simulator::new(Player::new(50, 500), *boss, false)
        .map(|s| s.mana_spent)
        .min()
        .unwrap()
}

#[aoc(day22, part2)]
fn part_2(boss: &Boss) -> u64 {
    Simulator::new(Player::new(50, 500), *boss, true)
        .map(|s| s.mana_spent)
        .min()
        .unwrap()
}

#[derive(Debug, Clone)]
struct Simulator {
    visited: HashSet<State>,
    pending: BinaryHeap<State>,
    min_mana_spent: u64,
}

impl Simulator {
    fn new(player: Player, boss: Boss, hard_mode: bool) -> Self {
        let start_state = State {
            player,
            boss,
            hard_mode,
            ..Default::default()
        };

        Self {
            visited: HashSet::new(),
            pending: [start_state].into(),
            min_mana_spent: u64::MAX,
        }
    }
}

impl Iterator for Simulator {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(state) = self.pending.pop() {
            if state.mana_spent > self.min_mana_spent {
                continue;
            }
            if self.visited.contains(&state) {
                continue;
            }
            self.visited.insert(state.clone());
            if state.boss.is_dead() {
                self.min_mana_spent = state.mana_spent;
                return Some(state);
            } else if !state.player.is_dead() {
                for child in state.moves() {
                    self.pending.push(child);
                }
            }
        }
        None
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Player {
    hp: u64,
    mana: u64,
    armor: u64,
}

impl Player {
    const fn new(hp: u64, mana: u64) -> Self {
        Self { hp, mana, armor: 0 }
    }
    const fn take_damage(&mut self, damage: u64) {
        let damage = damage.saturating_sub(self.armor + 1) + 1;
        self.hp = self.hp.saturating_sub(damage);
    }
    const fn heal(&mut self, healing: u64) {
        self.hp += healing;
    }
    const fn is_dead(&self) -> bool {
        self.hp == 0
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Boss {
    hp: u64,
    damage: u64,
}

impl Boss {
    const fn new(hp: u64, damage: u64) -> Self {
        Self { hp, damage }
    }
    const fn take_damage(&mut self, damage: u64) {
        self.hp = self.hp.saturating_sub(damage);
    }
    const fn is_dead(&self) -> bool {
        self.hp == 0
    }
}

#[derive(Debug, Default, Eq, Clone)]
struct State {
    round: i64,
    player: Player,
    boss: Boss,

    mana_spent: u64,

    hard_mode: bool,

    effect_timers: [u64; Effect::all().len()],
}

impl State {
    const fn check_win(&self) -> bool {
        self.player.is_dead() || self.boss.is_dead()
    }

    const fn create_child(&self) -> Self {
        Self {
            round: self.round + 1,
            ..*self
        }
    }

    fn make_move(&self, spell: Spell) -> Option<Self> {
        let mut child = self.create_child();

        if child.hard_mode {
            child.player.take_damage(1);
        }

        for effect in Effect::all() {
            effect.apply(&mut child);
        }

        if !spell.apply(&mut child) {
            return None;
        }

        for effect in Effect::all() {
            effect.apply(&mut child);
        }

        let damage = child.boss.damage;
        child.player.take_damage(damage);

        Some(child)
    }

    fn moves(&self) -> Vec<Self> {
        let mut res = Vec::new();
        if self.check_win() {
            return res;
        }
        for spell in Spell::all() {
            if let Some(child) = self.make_move(spell) {
                res.push(child);
            }
        }
        res
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.mana_spent.cmp(&self.mana_spent)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.round == other.round
            && self.player == other.player
            && self.boss == other.boss
            && self.mana_spent == other.mana_spent
            && self.hard_mode == other.hard_mode
            && self.effect_timers == other.effect_timers
    }
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.round.hash(state);
        self.player.hash(state);
        self.boss.hash(state);
        self.mana_spent.hash(state);
        self.hard_mode.hash(state);
        self.effect_timers.hash(state);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spell {
    MagicMissile,
    Drain,
    Effect(Effect),
}

impl Spell {
    const fn all() -> [Self; 5] {
        [
            Self::MagicMissile,
            Self::Drain,
            Self::Effect(Effect::Shield),
            Self::Effect(Effect::Poison),
            Self::Effect(Effect::Recharge),
        ]
    }

    const fn mana_cost(self) -> u64 {
        match self {
            Self::MagicMissile => 53,
            Self::Drain => 73,
            Self::Effect(Effect::Shield) => 113,
            Self::Effect(Effect::Poison) => 173,
            Self::Effect(Effect::Recharge) => 229,
        }
    }

    const fn apply(self, state: &mut State) -> bool {
        let cost = self.mana_cost();
        if cost > state.player.mana {
            return false;
        }
        match self {
            Self::MagicMissile => {
                state.boss.take_damage(4);
            }
            Self::Drain => {
                state.boss.take_damage(2);
                state.player.heal(2);
            }
            Self::Effect(eff) => {
                if state.effect_timers[eff.index()] > 0 {
                    return false;
                }
                state.effect_timers[eff.index()] = eff.duration();
            }
        }
        state.player.mana -= cost;
        state.mana_spent += cost;
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Effect {
    Shield,
    Poison,
    Recharge,
}

impl Effect {
    const fn all() -> [Self; 3] {
        [Self::Shield, Self::Poison, Self::Recharge]
    }
    const fn index(self) -> usize {
        self as usize
    }
    const fn duration(self) -> u64 {
        match self {
            Self::Shield | Self::Poison => 6,
            Self::Recharge => 5,
        }
    }

    fn apply(self, state: &mut State) {
        let index = self.index();
        if state.effect_timers[index] > 0 {
            state.effect_timers[index] -= 1;
            match self {
                Self::Shield => state.player.armor = 7,
                Self::Poison => state.boss.hp = state.boss.hp.saturating_sub(3),
                Self::Recharge => state.player.mana += 101,
            }
        } else if self == Self::Shield {
            state.player.armor = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_a() {
        let boss = Boss::new(13, 8);
        let player = Player::new(10, 250);

        let result = Simulator::new(player, boss, false)
            .map(|s| s.mana_spent)
            .min()
            .unwrap();

        assert_eq!(result, 226);
    }

    #[test]
    fn test_part_1_b() {
        let boss = Boss::new(14, 8);
        let player = Player::new(10, 250);

        let result = Simulator::new(player, boss, false)
            .map(|s| s.mana_spent)
            .min()
            .unwrap();

        assert_eq!(result, 641);
    }
}
