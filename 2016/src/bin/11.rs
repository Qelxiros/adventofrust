#![feature(let_chains)]

use std::{
    collections::*,
    fmt::{Debug, Display, Write},
    hash::Hash,
};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Entity {
    Chip(&'static str),
    Gen(&'static str),
}

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Chip(s) => write!(f, "c-{s}"),
            Self::Gen(s) => write!(f, "g-{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    floors: [HashSet<Entity>; 4],
    elevator: usize,
    steps: usize,
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(
            self.floors
                .iter()
                .map(|set| set.iter().sorted().join(""))
                .join("|")
                .as_bytes(),
        );
    }
}

impl State {
    fn is_valid(&self) -> bool {
        self.floors
            .iter()
            .filter(|set| set.iter().any(|e| matches!(e, Entity::Gen(_))))
            .all(|set| {
                set.iter()
                    .filter_map(|e| match e {
                        Entity::Gen(_) => None,
                        Entity::Chip(c) => Some(c),
                    })
                    .all(|c| set.contains(&Entity::Gen(c)))
            })
    }
}

fn part1(input: &str) -> usize {
    let chip_re = Regex::new(r"(\w+)-compatible microchip").unwrap();
    let gen_re = Regex::new(r"(\w+) generator").unwrap();

    let mut floors = input
        .lines()
        .map(|s| {
            chip_re
                .captures_iter(s)
                .map(|cap| Entity::Chip(cap[1][0..2].to_owned().leak()))
                .chain(
                    gen_re
                        .captures_iter(s)
                        .map(|cap| Entity::Gen(cap[1][0..2].to_owned().leak())),
                )
                .collect::<HashSet<_>>()
        })
        .collect_vec();
    let diff = 6 * floors[0].len();
    floors[0].clear();

    let init = State {
        floors: floors.try_into().unwrap(),
        elevator: 1,
        steps: 0,
    };

    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    q.push_back(init);

    while let Some(s) = q.pop_front() {
        if seen.contains(&s) {
            continue;
        }
        seen.insert(s.clone());
        if (0..3).all(|i| s.floors[i].is_empty()) {
            return s.steps + diff - 3;
        }
        q.extend(
            s.floors[s.elevator]
                .iter()
                .map(|e| vec![e])
                .chain(s.floors[s.elevator].iter().combinations(2))
                .cartesian_product(vec![true, false])
                .filter_map(|(subset, up)| {
                    if up && s.elevator + 1 == s.floors.len()
                        || !up && s.elevator == 0
                    {
                        None
                    } else {
                        let elevator =
                            if up { s.elevator + 1 } else { s.elevator - 1 };
                        let mut f = s.floors.clone();
                        f[s.elevator].retain(|e| !subset.contains(&e));
                        f[elevator].extend(subset);
                        Some(State {
                            floors: f,
                            elevator,
                            steps: s.steps + 1,
                        })
                    }
                })
                .filter(State::is_valid),
        );
    }

    0
}

fn part2(input: &str) -> impl Debug {
    part1(input) + 24
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/11")));
    println!("{:?}", part2(include_str!("../../input/11")));
}
