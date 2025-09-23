#![allow(unused)]
use std::cell::{Ref, RefCell, RefMut};
use std::collections::{HashSet, VecDeque};
use std::hash::Hash;
use std::num::ParseIntError;
use std::ops::Deref;
use std::rc::Rc;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[aoc_generator(day22)]
fn parse(input: &str) -> Result<Boss, ParseError> {
    let mut lines = input.lines();
    let hit_points = lines
        .next()
        .ok_or(ParseError::SyntaxError)?
        .strip_prefix("Hit Points: ")
        .ok_or(ParseError::SyntaxError)?
        .parse()?;
    let damage = lines
        .next()
        .ok_or(ParseError::SyntaxError)?
        .strip_prefix("Damage: ")
        .ok_or(ParseError::SyntaxError)?
        .parse()?;
    if lines.next().is_some() {
        return Err(ParseError::SyntaxError);
    }
    Ok(Boss::new(hit_points, damage))
}

#[aoc(day22, part1)]
fn part_1(boss: &Boss) -> u64 {
    play(Player::new(50, 500), *boss, false)
        .iter()
        .map(|s| s.as_state().mana_spent)
        .min()
        .unwrap()
}

#[aoc(day22, part2)]
fn part_2(boss: &Boss) -> u64 {
    play(Player::new(50, 500), *boss, true)
        .iter()
        .map(|s| s.as_state().mana_spent)
        .min()
        .unwrap()
}

fn play(player: Player, boss: Boss, hard_mode: bool) -> Vec<SharedState> {
    let start_state = SharedState::new(State {
        last_action: "Start",
        player,
        boss,
        hard_mode,
        ..Default::default()
    });

    #[expect(
        clippy::mutable_key_type,
        reason = "Inner struct is frozen, and came_from is not part of equality"
    )]
    let mut visited = HashSet::new();

    let mut pending = VecDeque::new();
    pending.push_front(start_state);

    let mut result = Vec::new();
    while let Some(state) = pending.pop_back() {
        if visited.contains(&state) {
            continue;
        }
        state.freeze();
        visited.insert(state.clone());
        if state.as_state().boss.is_dead() {
            result.push(state);
        } else if !state.as_state().player.is_dead() {
            for child in state.moves() {
                pending.push_front(child);
            }
        }
    }
    result
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
    last_action: &'static str,
    player: Player,
    boss: Boss,

    mana_spent: u64,

    hard_mode: bool,
    frozen: bool,

    effect_timers: [u64; Effect::all().len()],

    came_from: Option<SharedState>,
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

#[derive(Debug, Clone)]
struct SharedState(Rc<RefCell<State>>);
impl SharedState {
    fn new(state: State) -> Self {
        Self(Rc::new(RefCell::new(state)))
    }
}
impl PartialEq for SharedState {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().deref() == other.0.borrow().deref()
    }
}
impl Eq for SharedState {}
impl Hash for SharedState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.borrow().hash(state);
    }
}

impl SharedState {
    fn as_state(&self) -> Ref<'_, State> {
        self.0.borrow()
    }

    fn as_state_mut(&self) -> RefMut<'_, State> {
        let res = self.0.borrow_mut();
        assert!(!res.frozen, "Tried to modify frozen state");
        res
    }

    fn freeze(&self) {
        self.as_state_mut().frozen = true;
    }

    fn create_child(&self, action: &'static str) -> Self {
        let inner = self.as_state();
        Self(Rc::new(RefCell::new(State {
            last_action: action,
            round: inner.round + 1,
            came_from: Some(self.clone()),
            frozen: false,
            ..inner.clone()
        })))
    }

    fn check_win(&self) -> bool {
        let mut inner = self.as_state();
        inner.player.is_dead() || inner.boss.is_dead()
    }

    fn make_move(&self, spell: Spell) -> Option<Self> {
        let mut child = self.create_child(spell.name());

        for step in [
            Step::HardMode,
            Step::Effects,
            Step::Spell(spell),
            Step::Effects,
            Step::BossAction,
        ] {
            if !step.apply(&child) {
                return None;
            }
            if child.check_win() {
                return Some(child);
            }
        }

        Some(child)
    }

    fn moves(&self) -> Vec<Self> {
        if self.check_win() {
            return vec![];
        }
        let inner = self.as_state();
        let mut result: Vec<Self> = Vec::new();
        for spell in Spell::all() {
            if let Some(child) = self.make_move(spell) {
                result.push(child);
            }
        }
        result
    }

    fn get_history(&self) -> Vec<Self> {
        let inner = self.as_state();
        let mut result = Vec::new();
        if let Some(rc) = &inner.came_from {
            result = rc.get_history();
        }
        result.push(self.clone());
        result
    }
}

#[derive(Debug, Clone, Copy)]
enum Step {
    HardMode,
    Effects,
    Spell(Spell),
    BossAction,
}

impl Step {
    fn apply(self, state: &SharedState) -> bool {
        match self {
            Self::HardMode => {
                let mut inner = state.as_state_mut();
                if inner.hard_mode {
                    inner.player.take_damage(1);
                }
            }
            Self::Effects => {
                for effect in Effect::all() {
                    effect.apply(state);
                }
            }
            Self::Spell(spell) => return spell.apply(state),
            Self::BossAction => {
                let mut inner = state.as_state_mut();
                let damage = inner.boss.damage;
                inner.player.take_damage(damage);
            }
        }
        true
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

    const fn name(self) -> &'static str {
        match self {
            Self::MagicMissile => "Magic Missile",
            Self::Drain => "Drain",
            Self::Effect(Effect::Poison) => "Poison",
            Self::Effect(Effect::Shield) => "Shield",
            Self::Effect(Effect::Recharge) => "Recharge",
        }
    }

    fn apply(self, state: &SharedState) -> bool {
        let mut inner = state.as_state_mut();
        let cost = self.mana_cost();
        if cost > inner.player.mana {
            return false;
        }
        match self {
            Self::MagicMissile => {
                inner.boss.take_damage(4);
            }
            Self::Drain => {
                inner.boss.take_damage(2);
                inner.player.heal(2);
            }
            Self::Effect(eff) => {
                if inner.effect_timers[eff.index()] > 0 {
                    return false;
                }
                inner.effect_timers[eff.index()] = eff.duration();
            }
        }
        inner.player.mana -= cost;
        inner.mana_spent += cost;
        drop(inner);
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

    fn apply(self, state: &SharedState) {
        let index = self.index();
        let mut inner = state.0.borrow_mut();
        if inner.effect_timers[index] > 0 {
            inner.effect_timers[index] -= 1;
            match self {
                Self::Shield => inner.player.armor = 7,
                Self::Poison => inner.boss.hp = inner.boss.hp.saturating_sub(3),
                Self::Recharge => inner.player.mana += 101,
            }
        } else if self == Self::Shield {
            inner.player.armor = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;

    fn run_test_play(player: Player, boss: Boss, hard_mode: bool) -> String {
        let mut minimal = Vec::new();
        let mut min_cost = u64::MAX;
        for solution in play(player, boss, hard_mode) {
            let inner = solution.as_state();
            if inner.mana_spent < min_cost {
                minimal.clear();
                min_cost = inner.mana_spent;
            }
            if inner.mana_spent == min_cost {
                drop(inner);
                minimal.push(solution);
            }
        }
        let mut log = String::new();
        for state in minimal {
            for history in state.get_history() {
                let inner = history.as_state();
                let State {
                    round,
                    last_action,
                    player:
                        Player {
                            hp: player_hp,
                            mana: player_mana,
                            ..
                        },
                    boss: Boss { hp: boss_hp, .. },
                    mana_spent,

                    effect_timers,
                    ..
                } = *inner;
                write!(
                    &mut log,
                    "[{round}] {last_action} - Player:{player_hp}hp/{player_mana}mp Boss:{boss_hp}hp"
                );
                for effect in Effect::all() {
                    let name = Spell::Effect(effect).name();
                    let duration = effect_timers[effect.index()];
                    if duration > 0 {
                        write!(&mut log, " {name}({duration})");
                    }
                }
                writeln!(&mut log, " - Mana spent:{mana_spent}");
            }
        }
        log
    }

    #[test]
    fn test_play_a() {
        let result = run_test_play(Player::new(10, 250), Boss::new(13, 8), false);
        let expected = "\
            [0] Start - Player:10hp/250mp Boss:13hp - Mana spent:0\n\
            [1] Poison - Player:2hp/77mp Boss:10hp Poison(5) - Mana spent:173\n\
            [2] Magic Missile - Player:2hp/24mp Boss:0hp Poison(3) - Mana spent:226\n\
            ";
        assert_eq!(
            result, expected,
            "\nexpanded left:\n{result}\nexpanded right:\n{expected}\n"
        );
    }

    #[test]
    fn test_play_a_hard() {
        let result = run_test_play(Player::new(11, 250), Boss::new(13, 8), true);
        let expected = "\
            [0] Start - Player:11hp/250mp Boss:13hp - Mana spent:0\n\
            [1] Poison - Player:2hp/77mp Boss:10hp Poison(5) - Mana spent:173\n\
            [2] Magic Missile - Player:1hp/24mp Boss:0hp Poison(3) - Mana spent:226\n\
            ";
        assert_eq!(
            result, expected,
            "\nexpanded left:\n{result}\nexpanded right:\n{expected}\n"
        );
    }

    #[test]
    fn test_play_b() {
        let result = run_test_play(Player::new(10, 250), Boss::new(14, 8), false);
        let expected = "\
            [0] Start - Player:10hp/250mp Boss:14hp - Mana spent:0\n\
            [1] Recharge - Player:2hp/122mp Boss:14hp Recharge(4) - Mana spent:229\n\
            [2] Shield - Player:1hp/211mp Boss:14hp Shield(5) Recharge(2) - Mana spent:342\n\
            [3] Drain - Player:2hp/340mp Boss:12hp Shield(3) - Mana spent:415\n\
            [4] Poison - Player:1hp/167mp Boss:9hp Shield(1) Poison(5) - Mana spent:588\n\
            [5] Magic Missile - Player:1hp/114mp Boss:0hp Poison(3) - Mana spent:641\n\
            ";
        assert_eq!(
            result, expected,
            "\nexpanded left:\n{result}\nexpanded right:\n{expected}\n"
        );
    }

    #[test]
    fn test_play_b_hard() {
        let result = run_test_play(Player::new(15, 250), Boss::new(14, 8), true);
        let expected = "\
            [0] Start - Player:15hp/250mp Boss:14hp - Mana spent:0\n\
            [1] Recharge - Player:6hp/122mp Boss:14hp Recharge(4) - Mana spent:229\n\
            [2] Shield - Player:4hp/211mp Boss:14hp Shield(5) Recharge(2) - Mana spent:342\n\
            [3] Poison - Player:2hp/240mp Boss:11hp Shield(3) Poison(5) - Mana spent:515\n\
            [4] Drain - Player:2hp/167mp Boss:3hp Shield(1) Poison(3) - Mana spent:588\n\
            [5] Magic Missile - Player:1hp/167mp Boss:0hp Poison(2) - Mana spent:588\n\
            ";
        assert_eq!(
            result, expected,
            "\nexpanded left:\n{result}\nexpanded right:\n{expected}\n"
        );
    }

    fn run_test_make_move(player: Player, boss: Boss, hard_mode: bool, spells: &[Spell]) -> String {
        let mut state = SharedState::new(State {
            last_action: "Start",
            player,
            boss,
            hard_mode,
            ..Default::default()
        });

        let mut result = String::new();
        for &spell in spells {
            if state.as_state().player.is_dead() {
                writeln!(&mut result, "Unexpected boss win!");
                break;
            }
            if state.as_state().boss.is_dead() {
                writeln!(&mut result, "Unexpected player win!");
            }
            if let Some(next) = state.make_move(spell) {
                state = next;
            } else {
                let name = spell.name();
                writeln!(&mut result, "Failed to cast {name}!");
                break;
            }
        }
        for history in state.get_history() {
            let inner = history.as_state();
            let State {
                round,
                last_action,
                player:
                    Player {
                        hp: player_hp,
                        mana: player_mana,
                        ..
                    },
                boss: Boss { hp: boss_hp, .. },
                mana_spent,

                effect_timers,
                ..
            } = *inner;
            write!(
                &mut result,
                "[{round}] {last_action} - Player:{player_hp}hp/{player_mana}mp Boss:{boss_hp}hp"
            );
            for effect in Effect::all() {
                let name = Spell::Effect(effect).name();
                let duration = effect_timers[effect.index()];
                if duration > 0 {
                    write!(&mut result, " {name}({duration})");
                }
            }
            writeln!(&mut result, " - Mana spent:{mana_spent}");
        }
        result
    }

    #[test]
    fn test_make_move_a() {
        let result = run_test_make_move(
            Player::new(10, 250),
            Boss::new(14, 8),
            false,
            &[
                Spell::Effect(Effect::Recharge),
                Spell::Effect(Effect::Shield),
                Spell::Drain,
                Spell::Effect(Effect::Poison),
                Spell::MagicMissile,
            ],
        );
        let expected = "\
            [0] Start - Player:10hp/250mp Boss:14hp - Mana spent:0\n\
            [1] Recharge - Player:2hp/122mp Boss:14hp Recharge(4) - Mana spent:229\n\
            [2] Shield - Player:1hp/211mp Boss:14hp Shield(5) Recharge(2) - Mana spent:342\n\
            [3] Drain - Player:2hp/340mp Boss:12hp Shield(3) - Mana spent:415\n\
            [4] Poison - Player:1hp/167mp Boss:9hp Shield(1) Poison(5) - Mana spent:588\n\
            [5] Magic Missile - Player:1hp/114mp Boss:0hp Poison(3) - Mana spent:641\n\
            ";
        assert_eq!(
            result, expected,
            "\nexpanded left:\n{result}\nexpanded right:\n{expected}\n"
        );
    }
}
