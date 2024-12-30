#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    health: usize,
    mana: usize,
    boss_health: usize,
    boss_damage: usize,
    shield: usize,
    poison: usize,
    recharge: usize,
    cost: usize,
}

fn process_effects(state: &mut State) -> usize {
    let mut armor = 0;
    if state.poison > 0 {
        state.boss_health -= 3;
        state.poison -= 1;
    }
    if state.recharge > 0 {
        state.mana += 101;
        state.recharge -= 1;
    }
    if state.shield > 0 {
        armor = 7;
        state.shield -= 1;
    }
    armor
}

fn do_round(state: &mut State) {
    if state.boss_health == 0 {
        return;
    }

    let armor = process_effects(state);
    if state.health == 0 || state.boss_health == 0 {
        return;
    }

    state.health = state
        .health
        .saturating_sub(state.boss_damage.saturating_sub(armor).max(1));
    if state.health == 0 {
        return;
    }

    process_effects(state);
}

fn part1(input: &str) -> impl Debug {
    let i = integers_unsigned(input);
    let start = State {
        health: 50,
        mana: 500,
        boss_health: i[0],
        boss_damage: i[1],
        shield: 0,
        poison: 0,
        recharge: 0,
        cost: 0,
    };

    let mut q = VecDeque::new();
    q.push_back(start);
    let mut out = usize::MAX;
    while let Some(state) = q.pop_front() {
        if state.health == 0 {
            continue;
        }
        if state.boss_health == 0 {
            out = out.min(state.cost);
        }
        if state.cost > out {
            continue;
        }
        // Magic Missile
        if state.mana >= 53 {
            let mut new = state;
            new.mana -= 53;
            new.cost += 53;
            new.boss_health = new.boss_health.saturating_sub(4);
            do_round(&mut new);
            q.push_back(new);
        }
        // Drain
        if state.mana >= 73 {
            let mut new = state;
            new.mana -= 73;
            new.cost += 73;
            new.boss_health = new.boss_health.saturating_sub(2);
            new.health += 2;
            do_round(&mut new);
            q.push_back(new);
        }
        // Shield
        if state.mana >= 113 {
            let mut new = state;
            new.mana -= 113;
            new.cost += 113;
            new.shield = 6;
            do_round(&mut new);
            q.push_back(new);
        }
        // Poison
        if state.mana >= 173 {
            let mut new = state;
            new.mana -= 173;
            new.cost += 173;
            new.poison = 6;
            do_round(&mut new);
            q.push_back(new);
        }
        // Recharge
        if state.mana >= 229 {
            let mut new = state;
            new.mana -= 229;
            new.cost += 229;
            new.recharge = 5;
            do_round(&mut new);
            q.push_back(new);
        }
    }

    out
}

fn do_round2(state: &mut State) {
    if state.boss_health == 0 {
        return;
    }

    let armor = process_effects(state);
    if state.health == 0 || state.boss_health == 0 {
        return;
    }

    state.health = state
        .health
        .saturating_sub(state.boss_damage.saturating_sub(armor).max(1) + 1);

    if state.health == 0 {
        return;
    }
    process_effects(state);
}

fn part2(input: &str) -> impl Debug {
    let i = integers_unsigned(input);
    let start = State {
        health: 49,
        mana: 500,
        boss_health: i[0],
        boss_damage: i[1],
        shield: 0,
        poison: 0,
        recharge: 0,
        cost: 0,
    };

    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    q.push_back(start);
    let mut out = usize::MAX;
    while let Some(state) = q.pop_front() {
        if seen.contains(&state) {
            continue;
        }
        seen.insert(state);
        if state.health == 0 {
            continue;
        }
        if state.boss_health == 0 {
            out = out.min(state.cost);
            continue;
        }
        if state.cost > out {
            continue;
        }
        // Magic Missile
        if state.mana >= 53 {
            let mut new = state;
            new.mana -= 53;
            new.cost += 53;
            new.boss_health = new.boss_health.saturating_sub(4);
            do_round2(&mut new);
            q.push_back(new);
        }
        // Drain
        if state.mana >= 73 {
            let mut new = state;
            new.mana -= 73;
            new.cost += 73;
            new.boss_health = new.boss_health.saturating_sub(2);
            new.health += 2;
            do_round2(&mut new);
            q.push_back(new);
        }
        // Shield
        if state.mana >= 113 {
            let mut new = state;
            new.mana -= 113;
            new.cost += 113;
            new.shield = 6;
            do_round2(&mut new);
            q.push_back(new);
        }
        // Poison
        if state.mana >= 173 {
            let mut new = state;
            new.mana -= 173;
            new.cost += 173;
            new.poison = 6;
            do_round2(&mut new);
            q.push_back(new);
        }
        // Recharge
        if state.mana >= 229 {
            let mut new = state;
            new.mana -= 229;
            new.cost += 229;
            new.recharge = 5;
            do_round2(&mut new);
            q.push_back(new);
        }
    }

    out
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/22")));
    println!("{:?}", part2(include_str!("../../input/22")));
}
