#![feature(let_chains)]
#![feature(map_many_mut)]

use std::{
    collections::*,
    fmt::Debug,
    hash::{self, Hash},
    mem::swap,
    ops::BitAnd,
};

use itertools::{Itertools, repeat_n};
use num::pow::Pow;
use regex::Regex;

use adventofrust::utils::*;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn part1(input: &str) -> impl Debug {
    let mut wires = HashMap::new();
    let halves = input.split("\n\n").take(2).collect_vec();

    for line in halves[0].lines() {
        let v = line.split(": ").take(2).collect_vec();
        wires.insert(v[0], v[1] == "1");
    }

    let mut lines = halves[1].lines().collect::<VecDeque<_>>();

    let mut i = 0;
    loop {
        if lines.is_empty() {
            break;
        }
        // println!("{lines:?}, {i}");
        let v = lines[i].split_whitespace().collect_vec();
        if wires.contains_key(v[0]) && wires.contains_key(v[2]) {
            match v[1] {
                "AND" => wires.insert(v[4], wires[v[0]] && wires[v[2]]),
                "OR" => wires.insert(v[4], wires[v[0]] || wires[v[2]]),
                "XOR" => wires.insert(v[4], wires[v[0]] ^ wires[v[2]]),
                _ => unreachable!(),
            };
            lines.swap_remove_back(i);
            i = 0;
        } else {
            i += 1;
        }
    }

    let mut i = 0;
    let mut out: usize = 0;
    while let Some(&wire) = wires.get(format!("z{i:>02}").as_str()) {
        out += if wire { 1 } else { 0 } * (1usize << i);
        i += 1;
    }
    out
}

#[derive(Debug, Clone, Copy)]
enum AdderType {
    Sum(usize),
    Xor(usize),
    Carry(usize),
    CarryEasy(usize),
    CarryHard(usize),
    Input(usize),
    Invalid,
}

fn hash_swap<K, V>(map: &mut HashMap<K, V>, k1: K, k2: K)
where
    K: Eq + Hash,
{
    let temp1 = map.remove(&k1).unwrap();
    let temp2 = map.remove(&k2).unwrap();
    map.insert(k1, temp2);
    map.insert(k2, temp1);
}

fn part2(input: &str) -> impl Debug {
    let mut wires = HashMap::new();
    let halves = input.split("\n\n").take(2).collect_vec();

    for line in halves[0].lines() {
        let v = line.split(": ").take(2).collect_vec();
        wires.insert(v[0], v[1] == "1");
    }

    let mut dependence = HashMap::new();
    for line in halves[1].lines() {
        let v = line.split_whitespace().collect_vec();
        dependence.insert(v[4], (v[0], v[1], v[2]));
    }

    hash_swap(&mut dependence, "djg", "z12");
    hash_swap(&mut dependence, "sbg", "z19");
    hash_swap(&mut dependence, "mcq", "hjm");
    hash_swap(&mut dependence, "dsd", "z37");

    let mut roles = HashMap::new();
    for key in dependence.keys().chain(wires.keys()) {
        if let Some(v) = key.strip_prefix(['x', 'y']) {
            roles.insert(key, Some(AdderType::Input(v.parse().unwrap())));
        } else {
            roles.insert(key, None);
        }
    }

    loop {
        if dependence.keys().all(|a| roles[a].is_some()) {
            break;
        }
        for (key, value) in dependence
            .iter()
            .filter(|(a, _)| roles[a].is_none())
            .collect_vec()
        {
            match (roles[&value.0], value.1, roles[&value.2]) {
                (Some(AdderType::Carry(a)), "XOR", Some(AdderType::Xor(val))) if val == a + 1 => {
                    roles.insert(key, Some(AdderType::Sum(val)));
                }
                (Some(AdderType::Carry(a)), "AND", Some(AdderType::Xor(val))) if val == a + 1 => {
                    roles.insert(key, Some(AdderType::CarryHard(val)));
                }
                (Some(AdderType::Input(a)), "AND", Some(AdderType::Input(val)))
                    if val == a && val == 0 =>
                {
                    roles.insert(key, Some(AdderType::Carry(val)));
                }
                (Some(AdderType::Input(a)), "AND", Some(AdderType::Input(val))) if val == a => {
                    roles.insert(key, Some(AdderType::CarryEasy(val)));
                }
                (Some(AdderType::Input(a)), "XOR", Some(AdderType::Input(val))) if val == a => {
                    roles.insert(key, Some(AdderType::Xor(val)));
                }
                (Some(AdderType::CarryEasy(a)), "OR", Some(AdderType::CarryHard(val)))
                    if val == a =>
                {
                    roles.insert(key, Some(AdderType::Carry(val)));
                }
                (None, _, _) | (_, _, None) => continue,
                _ => match (roles[&value.2], value.1, roles[&value.0]) {
                    (Some(AdderType::Carry(a)), "XOR", Some(AdderType::Xor(val)))
                        if val == a + 1 =>
                    {
                        roles.insert(key, Some(AdderType::Sum(val)));
                    }
                    (Some(AdderType::Carry(a)), "AND", Some(AdderType::Xor(val)))
                        if val == a + 1 =>
                    {
                        roles.insert(key, Some(AdderType::CarryHard(val)));
                    }
                    (Some(AdderType::Input(a)), "AND", Some(AdderType::Input(val))) if val == a => {
                        roles.insert(key, Some(AdderType::CarryEasy(val)));
                    }
                    (Some(AdderType::Input(a)), "XOR", Some(AdderType::Input(val))) if val == a => {
                        roles.insert(key, Some(AdderType::Xor(val)));
                    }
                    (Some(AdderType::CarryEasy(a)), "OR", Some(AdderType::CarryHard(val)))
                        if val == a =>
                    {
                        roles.insert(key, Some(AdderType::Carry(val)));
                    }
                    (None, _, _) | (_, _, None) => continue,
                    _ => {
                        println!(
                            "{key}, {value:?}, {:?}, {:?}, {:?}",
                            roles[key], roles[&value.0], roles[&value.2]
                        );
                        println!("{:?}", roles.iter().filter(|(_, v)| v.is_some()));
                        unreachable!()
                    }
                },
            }
        }
    }
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/24")));
    println!("{:?}", part2(include_str!("../../input/24")));
}
