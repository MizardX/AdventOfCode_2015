use std::num::ParseIntError;
use std::ops::AddAssign;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Clone, Copy)]
struct CharInfo {
    hit_points: u64,
    damage: u64,
    armor: u64,
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

impl FromStr for CharInfo {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
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
        let armor = lines
            .next()
            .ok_or(ParseError::SyntaxError)?
            .strip_prefix("Armor: ")
            .ok_or(ParseError::SyntaxError)?
            .parse()?;
        if lines.next().is_some() {
            return Err(ParseError::SyntaxError);
        }
        Ok(Self {
            hit_points,
            damage,
            armor,
        })
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Item {
    cost: u64,
    damage: u64,
    armor: u64,
}

impl Item {
    pub const fn new(cost: u64, damage: u64, armor: u64) -> Self {
        Self {
            cost,
            damage,
            armor,
        }
    }
}

impl AddAssign for Item {
    fn add_assign(&mut self, rhs: Self) {
        self.cost += rhs.cost;
        self.damage += rhs.damage;
        self.armor += rhs.armor;
    }
}

const WEAPONS: &[Item] = &[
    Item::new(8, 4, 0),
    Item::new(10, 5, 0),
    Item::new(25, 6, 0),
    Item::new(40, 7, 0),
    Item::new(74, 8, 0),
];
const ARMOR: &[Item] = &[
    Item::new(13, 0, 1),
    Item::new(31, 0, 2),
    Item::new(53, 0, 3),
    Item::new(75, 0, 4),
    Item::new(102, 0, 5),
];
const RINGS: &[Item] = &[
    Item::new(25, 1, 0),
    Item::new(50, 2, 0),
    Item::new(100, 3, 0),
    Item::new(20, 0, 1),
    Item::new(40, 0, 2),
    Item::new(80, 0, 3),
];

#[aoc_generator(day21)]
fn parse(input: &str) -> Result<CharInfo, ParseError> {
    input.parse()
}

#[aoc(day21, part1)]
fn part_1(boss: &CharInfo) -> u64 {
    let mut win_min_cost = u64::MAX;
    for config in 0..(5 * 6) << 6 {
        let equipment = build_equipment(config);
        let player = CharInfo {
            hit_points: 100,
            damage: equipment.damage,
            armor: equipment.armor,
        };
        if simulate(player, *boss) {
            win_min_cost = win_min_cost.min(equipment.cost);
        }
    }
    win_min_cost
}

#[aoc(day21, part2)]
fn part_2(boss: &CharInfo) -> u64 {
    let mut loss_max_cost = u64::MIN;
    for config in 0..(5 * 6) << 6 {
        let equipment = build_equipment(config);
        let player = CharInfo {
            hit_points: 100,
            damage: equipment.damage,
            armor: equipment.armor,
        };
        if !simulate(player, *boss) {
            loss_max_cost = loss_max_cost.max(equipment.cost);
        }
    }
    loss_max_cost
}

fn build_equipment(mut config: usize) -> Item {
    let mut equipment = Item::default();

    let weapon_index = config % 5;
    equipment += WEAPONS[weapon_index];
    config /= 5;

    let armor_index = config % 6;
    if armor_index != 0 {
        equipment += ARMOR[armor_index - 1];
    }
    config /= 6;

    for (ring_index, ring) in RINGS.iter().enumerate() {
        if (config >> ring_index) & 1 != 0 {
            equipment += *ring;
        }
    }

    equipment
}

fn simulate(mut player: CharInfo, mut boss: CharInfo) -> bool {
    loop {
        boss.hit_points = boss
            .hit_points
            .saturating_sub(player.damage.saturating_sub(boss.armor).max(1));
        if boss.hit_points == 0 {
            return true;
        }
        player.hit_points = player
            .hit_points
            .saturating_sub(boss.damage.saturating_sub(player.armor).max(1));
        if player.hit_points == 0 {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
Hit Points: 12
Damage: 7
Armor: 2
"
    .trim_ascii();

    #[test]
    fn test_parse() {
        let result = parse(EXAMPLE).unwrap();
        assert_eq!(result.hit_points, 12);
        assert_eq!(result.damage, 7);
        assert_eq!(result.armor, 2);
    }

    #[test]
    fn test_simulate() {
        let mut player = CharInfo {
            hit_points: 8,
            damage: 5,
            armor: 5,
        };
        let boss = CharInfo {
            hit_points: 12,
            damage: 7,
            armor: 2,
        };
        assert!(simulate(player, boss), "player wins");
        player.hit_points -= 1;
        assert!(simulate(player, boss), "player wins");
        player.hit_points -= 1;
        assert!(!simulate(player, boss), "boss wins");
    }
}
