use std::{
    collections::{HashMap, HashSet},
    fs,
};

use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

fn antinodes(a: (usize, usize), b: (usize, usize), set: &mut HashSet<(isize, isize)>) {
    set.insert((
        a.0 as isize - (b.0 as isize - a.0 as isize),
        a.1 as isize - (b.1 as isize - a.1 as isize),
    ));
    set.insert((
        b.0 as isize + (b.0 as isize - a.0 as isize),
        b.1 as isize + (b.1 as isize - a.1 as isize),
    ));
}

fn part1() -> Result<()> {
    let input = include_str!("/home/jeremy/github/adventofrust/input/08");

    let mut nodes: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    input.lines().enumerate().for_each(|(idx, s)| {
        s.chars()
            .enumerate()
            .filter(|(_, c)| *c != '.')
            .for_each(|(i, c)| match nodes.get_mut(&c) {
                Some(v) => v.push((idx, i)),
                None => {
                    nodes.insert(c, vec![(idx, i)]);
                }
            })
    });

    let mut set = HashSet::new();
    for v in nodes.values() {
        v.iter()
            .combinations(2)
            .for_each(|v| antinodes(*v[0], *v[1], &mut set));
    }

    println!(
        "{:?}",
        set.clone()
            .into_iter()
            .filter(|a| a.0 >= 0
                && a.0 < input.lines().count() as isize
                && a.1 > 0
                && a.1 <= input.lines().next().unwrap().len() as isize)
            .collect_vec()
    );
    println!(
        "{}",
        set.into_iter()
            .filter(|a| a.0 >= 0
                && a.0 < input.lines().count() as isize
                && a.1 >= 0
                && a.1 < input.lines().next().unwrap().len() as isize)
            .count()
    );

    Ok(())
}

fn antinodes_harmonic(
    a: (usize, usize),
    b: (usize, usize),
    set: &mut HashSet<(isize, isize)>,
    lines: usize,
    line_len: usize,
) {
    let a = (a.0 as isize, a.1 as isize);
    let b = (b.0 as isize, b.1 as isize);
    let delta = (b.0 - a.0, b.1 - a.1);
    let mut pos = b;
    while pos.0 >= 0 && pos.0 < lines as isize && pos.1 >= 0 && pos.1 < line_len as isize {
        set.insert(pos);
        pos.0 += delta.0;
        pos.1 += delta.1;
    }
    let mut pos = a;
    while pos.0 >= 0 && pos.0 < lines as isize && pos.1 >= 0 && pos.1 < line_len as isize {
        set.insert(pos);
        pos.0 -= delta.0;
        pos.1 -= delta.1;
    }
}

fn part2() -> Result<()> {
    let input = include_str!("/home/jeremy/github/adventofrust/input/08");

    let mut nodes: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    input.lines().enumerate().for_each(|(idx, s)| {
        s.chars()
            .enumerate()
            .filter(|(_, c)| *c != '.')
            .for_each(|(i, c)| match nodes.get_mut(&c) {
                Some(v) => v.push((idx, i)),
                None => {
                    nodes.insert(c, vec![(idx, i)]);
                }
            })
    });

    let mut set = HashSet::new();
    for v in nodes.values() {
        v.iter().combinations(2).for_each(|v| {
            antinodes_harmonic(
                *v[0],
                *v[1],
                &mut set,
                input.lines().count(),
                input.lines().next().unwrap().len(),
            )
        });
    }

    println!("{}", set.len());

    Ok(())
}

fn main() {
    part1().unwrap();
    part2().unwrap();
}
