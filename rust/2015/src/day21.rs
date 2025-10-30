use chumsky::prelude::*;
use itertools::Itertools;

static WEAPONS: [(i32, i32); 5] = [(8, 4), (10, 5), (25, 6), (40, 7), (74, 8)];

static ARMORS: [(i32, i32); 6] = [(0, 0), (13, 1), (31, 2), (53, 3), (75, 4), (102, 5)];

static RINGS: [(i32, i32, i32); 8] = [
    (0, 0, 0),
    (0, 0, 0),
    (25, 1, 0),
    (50, 2, 0),
    (100, 3, 0),
    (20, 0, 1),
    (40, 0, 2),
    (80, 0, 3),
];

fn parser<'a>() -> impl Parser<'a, &'a str, (i32, i32, i32), extra::Err<Rich<'a, char>>> {
    group((
        just("Hit Points: ")
            .ignore_then(text::int(10).from_str::<i32>().unwrapped())
            .then_ignore(text::newline()),
        just("Damage: ")
            .ignore_then(text::int(10).from_str::<i32>().unwrapped())
            .then_ignore(text::newline()),
        just("Armor: ")
            .ignore_then(text::int(10).from_str::<i32>().unwrapped())
            .then_ignore(text::newline()),
    ))
}

fn turns_to_defeat(hp: i32, armor: i32, damage: i32) -> i32 {
    let damager_per_turn = std::cmp::max(1, damage - armor);
    (hp + damager_per_turn - 1) / damager_per_turn
}

fn can_win(
    hp: i32,
    damage: i32,
    armor: i32,
    boss_hp: i32,
    boss_damage: i32,
    boss_armor: i32,
) -> bool {
    turns_to_defeat(boss_hp, boss_armor, damage) <= turns_to_defeat(hp, armor, boss_damage)
}

pub fn solve(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Failed to read file.");
    let (boss_hp, boss_damage, boss_armor) = parser()
        .parse(&input)
        .into_result()
        .expect("Failed to parse input");
    let mut min_winning_cost = i32::MAX;
    let mut max_losing_cost = 0;
    for (weapon_cost, damage) in WEAPONS.iter() {
        for (armor_cost, armor) in ARMORS.iter() {
            for rings in RINGS.iter().combinations(2) {
                let (ring_cost1, ring_damage1, ring_armor1) = rings[0];
                let (ring_cost2, ring_damage2, ring_armor2) = rings[1];
                let cost = weapon_cost + armor_cost + ring_cost1 + ring_cost2;
                if can_win(
                    100,
                    damage + ring_damage1 + ring_damage2,
                    armor + ring_armor1 + ring_armor2,
                    boss_hp,
                    boss_damage,
                    boss_armor,
                ) {
                    min_winning_cost = std::cmp::min(cost, min_winning_cost);
                } else {
                    max_losing_cost = std::cmp::max(cost, max_losing_cost);
                }
            }
        }
    }
    assert!(min_winning_cost == 111);
    assert!(max_losing_cost == 188);
}
