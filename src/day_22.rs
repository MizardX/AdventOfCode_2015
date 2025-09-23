#![allow(unused)]
use std::cell::{Ref, RefCell};
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
fn parse(input: &str) -> Result<(i64, i64), ParseError> {
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
    Ok((hit_points, damage))
}

#[aoc(day22, part1)]
fn part_1(state: &(i64, i64)) -> i64 {
    play((50, 500), *state, false)
        .iter()
        .map(|s| s.as_state().mana_spent)
        .min()
        .unwrap()
}

#[aoc(day22, part2)]
fn part_2(state: &(i64, i64)) -> i64 {
    play((50, 500), *state, true)
        .iter()
        .map(|s| s.as_state().mana_spent)
        .min()
        .unwrap()
}

fn play(player: (i64, i64), boss: (i64, i64), hard_mode: bool) -> Vec<SharedState> {
    let start_state = SharedState::new(State {
        last_action: "Start",
        player_hp: player.0,
        player_mana: player.1,
        boss_hp: boss.0,
        boss_damage: boss.1,
        hard_mode,
        ..Default::default()
    });

    #[expect(clippy::mutable_key_type, reason = "Cell not part of equality check")]
    let mut visited = HashSet::new();

    let mut pending = VecDeque::new();
    pending.push_front(start_state);

    let mut result = Vec::new();
    while let Some(state) = pending.pop_back() {
        if !visited.insert(state.clone()) {
            continue;
        }
        if state.as_state().player_win {
            result.push(state);
        } else if !state.as_state().boss_win {
            for child in state.moves() {
                pending.push_front(child);
            }
        }
    }
    result
}

#[derive(Debug, Default, Eq, Clone)]
struct State {
    round: i64,
    last_action: &'static str,
    player_hp: i64,
    player_mana: i64,
    boss_hp: i64,
    mana_spent: i64,

    boss_damage: i64,
    hard_mode: bool,

    armor_turns: i64,
    armor_bonus: i64,
    poison_turns: i64,
    poison_damage: i64,
    recharge_turns: i64,
    recharge_mana: i64,

    player_win: bool,
    boss_win: bool,
    came_from: Option<SharedState>,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.round == other.round
            && self.player_hp == other.player_hp
            && self.player_mana == other.player_mana
            && self.boss_hp == other.boss_hp
            && self.mana_spent == other.mana_spent
            && self.boss_damage == other.boss_damage
            && self.hard_mode == other.hard_mode
            && self.armor_turns == other.armor_turns
            && self.armor_bonus == other.armor_bonus
            && self.poison_turns == other.poison_turns
            && self.poison_damage == other.poison_damage
            && self.recharge_turns == other.recharge_turns
            && self.recharge_mana == other.recharge_mana
            && self.player_win == other.player_win
            && self.boss_win == other.boss_win
    }
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.round.hash(state);
        self.player_hp.hash(state);
        self.player_mana.hash(state);
        self.boss_hp.hash(state);
        self.mana_spent.hash(state);
        self.boss_damage.hash(state);
        self.hard_mode.hash(state);
        self.armor_turns.hash(state);
        self.armor_bonus.hash(state);
        self.poison_turns.hash(state);
        self.poison_damage.hash(state);
        self.recharge_turns.hash(state);
        self.recharge_mana.hash(state);
        self.player_win.hash(state);
        self.boss_win.hash(state);
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
    fn create_child(&self, action: &'static str) -> Self {
        let inner = self.as_state();
        Self(Rc::new(RefCell::new(State {
            last_action: action,
            round: inner.round + 1,
            came_from: Some(self.clone()),
            ..inner.clone()
        })))
    }

    #[expect(
        clippy::too_many_lines,
        clippy::cognitive_complexity,
        reason = "todo: refactor"
    )]
    fn moves(&self) -> Vec<Self> {
        let inner = self.as_state();
        if inner.player_win || inner.boss_win {
            return vec![];
        }
        let mut result: Vec<Self> = Vec::new();
        for action in SPELLS {
            let mut child = self.create_child(action.name);
            let mut child_inner = child.0.borrow_mut();

            // Apply effects (player turn)

            if child_inner.hard_mode {
                child_inner.player_hp -= 1;
                if child_inner.player_hp <= 0 {
                    child_inner.boss_win = true;
                    drop(child_inner);
                    result.push(child);
                    continue;
                }
            }
            if child_inner.armor_turns > 0 {
                // No direct effect
                child_inner.armor_turns -= 1;
                if child_inner.armor_turns == 0 {
                    child_inner.armor_bonus = 0;
                }
            }
            if child_inner.recharge_turns > 0 {
                child_inner.player_mana += child_inner.recharge_mana;
                child_inner.recharge_turns -= 1;
                if child_inner.recharge_turns <= 0 {
                    child_inner.recharge_mana = 0;
                }
            }
            if child_inner.poison_turns > 0 {
                child_inner.boss_hp -= child_inner.poison_damage;
                child_inner.poison_turns -= 1;
                if child_inner.poison_turns <= 0 {
                    child_inner.poison_damage = 0;
                }
            }
            if child_inner.boss_hp <= 0 {
                // Since boss dies before player makes a move, any move is unnessesary.
                // The action is "No action", since that is the first in the list.
                child_inner.player_win = true;
                drop(child_inner);
                result.push(child);
                break;
            }

            // Validate valid moves
            if child_inner.player_mana < action.cost {
                continue;
            }
            if action.effect_armor > 0 && child_inner.armor_turns > 0 {
                continue;
            }
            if action.effect_poison > 0 && child_inner.poison_turns > 0 {
                continue;
            }
            if action.effect_recharge > 0 && child_inner.recharge_turns > 0 {
                continue;
            }

            // Player turn

            child_inner.player_mana -= action.cost;
            child_inner.mana_spent += action.cost;
            child_inner.boss_hp -= action.damage;
            child_inner.player_hp += action.heal;

            if action.effect_armor > 0 {
                child_inner.armor_bonus = action.effect_armor;
                child_inner.armor_turns = action.effect_length;
            }
            if action.effect_poison > 0 {
                child_inner.poison_damage = action.effect_poison;
                child_inner.poison_turns = action.effect_length;
            }
            if action.effect_recharge > 0 {
                child_inner.recharge_mana = action.effect_recharge;
                child_inner.recharge_turns = action.effect_length;
            }
            if child_inner.boss_hp <= 0 {
                child_inner.player_win = true;
                drop(child_inner);
                result.push(child);
                continue;
            }

            // Apply effects (boss turn)

            if child_inner.armor_turns > 0 {
                // No direct effect
                child_inner.armor_turns -= 1;
                if child_inner.armor_turns == 0 {
                    child_inner.armor_bonus = 0;
                }
            }
            if child_inner.recharge_turns > 0 {
                child_inner.player_mana += child_inner.recharge_mana;
                child_inner.recharge_turns -= 1;
                if child_inner.recharge_turns <= 0 {
                    child_inner.recharge_mana = 0;
                }
            }
            if child_inner.poison_turns > 0 {
                child_inner.boss_hp -= child_inner.poison_damage;
                child_inner.poison_turns -= 1;
                if child_inner.poison_turns <= 0 {
                    child_inner.poison_damage = 0;
                }
            }
            if child_inner.boss_hp <= 0 {
                child_inner.player_win = true;
                drop(child_inner);
                result.push(child);
                continue;
            }

            // Boss turn

            child_inner.player_hp -= (child_inner.boss_damage - child_inner.armor_bonus).max(1);
            if child_inner.player_hp <= 0 {
                child_inner.boss_win = true;
                drop(child_inner);
                result.push(child);
                continue;
            }

            drop(child_inner);
            result.push(child);
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

struct Spell {
    name: &'static str,
    cost: i64,
    damage: i64,
    heal: i64,
    effect_armor: i64,
    effect_poison: i64,
    effect_recharge: i64,
    effect_length: i64,
}

impl Spell {
    #[expect(clippy::too_many_arguments, reason = "todo: convert to enum")]
    const fn new(
        name: &'static str,
        cost: i64,
        damage: i64,
        heal: i64,
        effect_armor: i64,
        effect_poison: i64,
        effect_recharge: i64,
        effect_length: i64,
    ) -> Self {
        Self {
            name,
            cost,
            damage,
            heal,
            effect_armor,
            effect_poison,
            effect_recharge,
            effect_length,
        }
    }
}

const SPELLS: &[Spell] = &[
    Spell::new("Magic Missile", 53, 4, 0, 0, 0, 0, 0),
    Spell::new("Drain", 73, 2, 2, 0, 0, 0, 0),
    Spell::new("Shield", 113, 0, 0, 7, 0, 0, 6),
    Spell::new("Poison", 173, 0, 0, 0, 3, 0, 6),
    Spell::new("Recharge", 229, 0, 0, 0, 0, 101, 5),
];
/*

static (string name, int cost, int damage, int heal, int effectArmor, int effectPoison, int effectRecharge, int effectLength)[] spells = new[]{
    //("No action",       0, 0, 0, 0, 0,   0, 0),
    ("Magic Missile",  53, 4, 0, 0, 0,   0, 0),
    ("Drain",          73, 2, 2, 0, 0,   0, 0),
    ("Shield",        113, 0, 0, 7, 0,   0, 6),
    ("Poison",        173, 0, 0, 0, 3,   0, 6),
    ("Recharge",      229, 0, 0, 0, 0, 101, 5)
};
 */

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;

    fn run_test(player: (i64, i64), boss: (i64, i64), hard_mode: bool) -> String {
        let mut minimal = Vec::new();
        let mut min_cost = i64::MAX;
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
                    player_hp,
                    player_mana,
                    boss_hp,
                    mana_spent,

                    armor_turns,
                    armor_bonus,
                    poison_turns,
                    poison_damage,
                    recharge_turns,
                    recharge_mana,
                    ..
                } = *inner;
                write!(&mut log, "[{round}] {last_action} - Player:{player_hp}hp/{player_mana}mp Boss:{boss_hp}hp");
                if armor_turns > 0 {
                    write!(&mut log, " Armor:{armor_bonus}hp({armor_turns})");
                }
                if poison_turns > 0 {
                    write!(&mut log, " Poison:{poison_damage}hp({poison_turns})");
                }
                if recharge_turns > 0 {
                    write!(&mut log, " Recharge:{recharge_mana}mp({recharge_turns})");
                }
                writeln!(&mut log, " - Mana spent:{mana_spent}");
            }
        }
        log
    }

    #[test]
    fn test_part_1_a() {
        let result = run_test((10, 250), (13, 8), false);
        
        assert_eq!(result, "\
        [0] Start - Player:10hp/250mp Boss:13hp - Mana spent:0\n\
        [1] Poison - Player:2hp/77mp Boss:10hp Poison:3hp(5) - Mana spent:173\n\
        [2] Magic Missile - Player:2hp/24mp Boss:0hp Poison:3hp(3) - Mana spent:226\n\
        ");
    }

    #[test]
    fn test_part_1_b() {
        let result = run_test((10, 250), (14, 8), false);
        
        assert_eq!(result, "\
        [0] Start - Player:10hp/250mp Boss:14hp - Mana spent:0\n\
        [1] Recharge - Player:2hp/122mp Boss:14hp Recharge:101mp(4) - Mana spent:229\n\
        [2] Shield - Player:1hp/211mp Boss:14hp Armor:7hp(5) Recharge:101mp(2) - Mana spent:342\n\
        [3] Drain - Player:2hp/340mp Boss:12hp Armor:7hp(3) - Mana spent:415\n\
        [4] Poison - Player:1hp/167mp Boss:9hp Armor:7hp(1) Poison:3hp(5) - Mana spent:588\n\
        [5] Magic Missile - Player:1hp/114mp Boss:-1hp Poison:3hp(3) - Mana spent:641\n\
        ");
    }
}
