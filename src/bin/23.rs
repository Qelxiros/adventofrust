#![feature(let_chains)]

use std::{collections::*, fmt::Debug, iter::once};

use itertools::Itertools;
use regex::Regex;

use adventofrust::utils::*;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn part1(input: &str) -> impl Debug {
    let computers = input
        .lines()
        .flat_map(|s| {
            let v = s.split('-').collect_vec();
            vec![v[0], v[1]]
        })
        .filter(|s| s.starts_with('t'))
        .collect::<HashSet<_>>();

    let cons = input
        .lines()
        .flat_map(|s| {
            let v = s.split('-').collect_vec();
            vec![(v[0], v[1]), (v[1], v[0])]
        })
        .collect::<HashSet<_>>();

    let mut out = HashSet::new();
    for con in &cons {
        for c in &computers {
            if *c == con.0 || *c == con.1 {
                continue;
            }
            if cons.contains(&(con.0, c)) && cons.contains(&(con.1, c)) {
                let mut v = vec![con.0, con.1, c];
                v.sort();
                out.insert(v);
            }
        }
    }

    out.len()
}

fn bron_kerbosch<'a>(
    r: HashSet<&'a str>,
    mut p: HashSet<&'a str>,
    x: HashSet<&str>,
    graph: &HashSet<(&str, &str)>,
) -> Vec<HashSet<&'a str>> {
    if p.is_empty() && x.is_empty() {
        return vec![r];
    }
    let mut cliques = Vec::new();
    for v in p.clone() {
        let mut new_r = r.clone();
        new_r.insert(v);
        let nv = graph
            .iter()
            .filter(|a| a.0 == v)
            .map(|a| a.1)
            .collect::<HashSet<_>>();
        let mut new_p = p.clone();
        new_p.retain(|a| nv.contains(a));
        let mut new_x = x.clone();
        new_x.retain(|a| nv.contains(a));
        let s = bron_kerbosch(new_r, new_p, new_x, graph);
        if !s.is_empty() {
            cliques.extend(s);
        }
        p.retain(|a| *a != v);
    }

    cliques
}

fn part2(input: &str) -> impl Debug {
    let computers = input
        .lines()
        .flat_map(|s| {
            let v = s.split('-').collect_vec();
            vec![v[0], v[1]]
        })
        .collect::<HashSet<_>>();

    let cons = input
        .lines()
        .flat_map(|s| {
            let v = s.split('-').collect_vec();
            vec![(v[0], v[1]), (v[1], v[0])]
        })
        .collect::<HashSet<_>>();

    let mut set = bron_kerbosch(HashSet::new(), computers, HashSet::new(), &cons)
        .into_iter()
        .max_by_key(|s| s.len())
        .unwrap()
        .into_iter()
        .collect_vec();

    set.sort();
    set.join(",")
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/23")));
    println!("{:?}", part2(include_str!("../../input/23")));
}
