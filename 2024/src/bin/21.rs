#![feature(let_chains)]

use std::{collections::*, fmt::Debug, iter::once};

use itertools::Itertools;

const CHARS: [char; 5] = ['^', '>', 'v', '<', 'A'];

fn get_routes_numeric(
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<String> {
    let vert_c = if start.0 < end.0 { 'v' } else { '^' };
    let horz_c = if start.1 < end.1 { '>' } else { '<' };

    once(vert_c)
        .cycle()
        .take(start.0.abs_diff(end.0))
        .chain(once(horz_c).cycle().take(start.1.abs_diff(end.1)))
        .permutations(start.0.abs_diff(end.0) + start.1.abs_diff(end.1))
        .filter(move |v| {
            !(start == (3, 1) && matches!(v.first(), Some('<'))
                || start == (3, 2)
                    && matches!(v.first(), Some('<'))
                    && matches!(v.get(1), Some('<'))
                || start == (2, 0) && matches!(v.first(), Some('v'))
                || start == (1, 0)
                    && matches!(v.first(), Some('v'))
                    && matches!(v.get(1), Some('v'))
                || start == (0, 0)
                    && matches!(v.first(), Some('v'))
                    && matches!(v.get(1), Some('v'))
                    && matches!(v.get(2), Some('v')))
        })
        .map(|mut v| {
            v.push('A');
            v.into_iter().collect::<String>()
        })
        .min_set_by_key(|s| s.len())
}

fn get_routes(start: (usize, usize), end: (usize, usize)) -> Vec<String> {
    let vert_c = if start.0 < end.0 { 'v' } else { '^' };
    let horz_c = if start.1 < end.1 { '>' } else { '<' };

    once(vert_c)
        .cycle()
        .take(start.0.abs_diff(end.0))
        .chain(once(horz_c).cycle().take(start.1.abs_diff(end.1)))
        .permutations(start.0.abs_diff(end.0) + start.1.abs_diff(end.1))
        .filter(move |v| {
            !(start == (0, 1) && matches!(v.first(), Some('<'))
                || start == (0, 2)
                    && matches!(v.first(), Some('<'))
                    && matches!(v.get(1), Some('<'))
                || start == (1, 0) && matches!(v.first(), Some('^')))
        })
        .map(|mut v| {
            v.push('A');
            v.into_iter().collect::<String>()
        })
        .min_set_by_key(|s| s.len())
}

fn obfuscate_first(
    line: &str,
    locs: &HashMap<char, (usize, usize)>,
) -> Vec<String> {
    line.chars()
        .tuple_windows()
        .map(|(a, b)| {
            get_routes_numeric(*locs.get(&a).unwrap(), *locs.get(&b).unwrap())
        })
        .multi_cartesian_product()
        .map(|v| v.join(""))
        .unique()
        .collect_vec()
}

fn process_line(
    line: &str,
    locs: &HashMap<char, (usize, usize)>,
    cache: &HashMap<(char, char), usize>,
) -> usize {
    let line = format!("A{line}");
    obfuscate_first(&line, locs)
        .into_iter()
        .map(|s| {
            let s = format!("A{s}");
            (
                s.clone(),
                s.chars()
                    .tuple_windows()
                    .map(|(start, end)| cache.get(&(start, end)).unwrap())
                    .sum(),
            )
        })
        .min_by_key(|s| s.1)
        .map(|s| s.1)
        .unwrap()
}

fn part1(input: &str) -> impl Debug {
    let mut locs = HashMap::new();
    locs.insert('0', (3, 1));
    locs.insert('1', (2, 0));
    locs.insert('2', (2, 1));
    locs.insert('3', (2, 2));
    locs.insert('4', (1, 0));
    locs.insert('5', (1, 1));
    locs.insert('6', (1, 2));
    locs.insert('7', (0, 0));
    locs.insert('8', (0, 1));
    locs.insert('9', (0, 2));
    locs.insert('A', (3, 2));
    let mut pos_locs = HashMap::new();
    pos_locs.insert('^', (0, 1));
    pos_locs.insert('>', (1, 2));
    pos_locs.insert('v', (1, 1));
    pos_locs.insert('<', (1, 0));
    pos_locs.insert('A', (0, 2));
    let mut cache = CHARS
        .into_iter()
        .cartesian_product(CHARS)
        .map(|(a, b)| {
            let start: (usize, usize) = *pos_locs.get(&a).unwrap();
            let end = *pos_locs.get(&b).unwrap();
            (
                (a, b),
                start.0.abs_diff(end.0) + start.1.abs_diff(end.1) + 1,
            )
        })
        .collect::<HashMap<_, _>>();
    cache = CHARS
        .into_iter()
        .cartesian_product(CHARS)
        .map(|(a, b)| {
            (
                (a, b),
                get_routes(
                    *pos_locs.get(&a).unwrap(),
                    *pos_locs.get(&b).unwrap(),
                )
                .into_iter()
                .map(|s| {
                    let s = format!("A{s}");
                    s.chars()
                        .tuple_windows()
                        .map(|(c1, c2)| cache.get(&(c1, c2)).unwrap())
                        .sum::<usize>()
                })
                .min()
                .unwrap(),
            )
        })
        .collect::<HashMap<_, _>>();
    input
        .lines()
        .map(|line| {
            process_line(line, &locs, &cache)
                * line
                    .chars()
                    .take(3)
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap()
        })
        .sum::<usize>()
}

fn part2(input: &str) -> impl Debug {
    let mut locs = HashMap::new();
    locs.insert('0', (3, 1));
    locs.insert('1', (2, 0));
    locs.insert('2', (2, 1));
    locs.insert('3', (2, 2));
    locs.insert('4', (1, 0));
    locs.insert('5', (1, 1));
    locs.insert('6', (1, 2));
    locs.insert('7', (0, 0));
    locs.insert('8', (0, 1));
    locs.insert('9', (0, 2));
    locs.insert('A', (3, 2));
    let mut pos_locs = HashMap::new();
    pos_locs.insert('^', (0, 1));
    pos_locs.insert('>', (1, 2));
    pos_locs.insert('v', (1, 1));
    pos_locs.insert('<', (1, 0));
    pos_locs.insert('A', (0, 2));
    let mut cache = CHARS
        .into_iter()
        .cartesian_product(CHARS)
        .map(|(a, b)| {
            let start: (usize, usize) = *pos_locs.get(&a).unwrap();
            let end = *pos_locs.get(&b).unwrap();
            (
                (a, b),
                start.0.abs_diff(end.0) + start.1.abs_diff(end.1) + 1,
            )
        })
        .collect::<HashMap<_, _>>();
    for _ in 0..24 {
        cache = CHARS
            .into_iter()
            .cartesian_product(CHARS)
            .map(|(a, b)| {
                (
                    (a, b),
                    get_routes(
                        *pos_locs.get(&a).unwrap(),
                        *pos_locs.get(&b).unwrap(),
                    )
                    .into_iter()
                    .map(|s| {
                        let s = format!("A{s}");
                        s.chars()
                            .tuple_windows()
                            .map(|(c1, c2)| cache.get(&(c1, c2)).unwrap())
                            .sum::<usize>()
                    })
                    .min()
                    .unwrap(),
                )
            })
            .collect::<HashMap<_, _>>();
    }
    input
        .lines()
        .map(|line| {
            process_line(line, &locs, &cache)
                * line
                    .chars()
                    .take(3)
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap()
        })
        .sum::<usize>()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/21")));
    println!("{:?}", part2(include_str!("../../input/21")));
}
