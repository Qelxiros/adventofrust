#![feature(let_chains)]

use std::{collections::*, fmt::Debug};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn rotate(v: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut out = vec![vec![' '; v.len()]; v.len()];
    for i in 0..v.len() {
        for j in 0..v.len() {
            out[i][j] = v[j][v.len() - 1 - i];
        }
    }
    out
}

fn flip_rotate(grid: &[Vec<char>]) -> HashSet<Vec<Vec<char>>> {
    let mut out = HashSet::new();

    let mut g1 = grid.to_vec();
    let mut g2 = grid.iter().cloned().rev().collect_vec();
    for _ in 0..4 {
        out.insert(g1.clone());
        g1 = rotate(&g1);
        out.insert(g2.clone());
        g2 = rotate(&g2);
    }

    out
}

fn mutate(
    v: &[Vec<char>],
    map: &HashMap<Vec<Vec<char>>, Vec<Vec<char>>>,
    cache: &mut HashMap<Vec<Vec<char>>, Vec<Vec<char>>>,
) -> Vec<Vec<char>> {
    if let Some(v) = cache.get(v) {
        return v.clone();
    }
    let res = flip_rotate(v)
        .into_iter()
        .find_map(|p| map.get(&p))
        .unwrap()
        .to_vec();
    cache.insert(v.to_vec(), res.clone());
    res
}

fn split(mut grid: &[Vec<char>], div: usize) -> Vec<Vec<Vec<Vec<char>>>> {
    let mut rows = Vec::new();
    while grid.len() >= div {
        let (a, b) = grid.split_at(div);
        rows.push(a);
        grid = b;
    }

    let res = rows
        .into_iter()
        .map(|r| {
            let mut r = r.to_vec();
            let mut cols = Vec::new();
            while r[0].len() >= div {
                let (a, b): (Vec<Vec<char>>, Vec<Vec<char>>) = r
                    .iter()
                    .map(|v| v.split_at(div))
                    .map(|(a, b)| (a.to_vec(), b.to_vec()))
                    .unzip();
                cols.push(a);
                r = b;
            }
            cols
        })
        .collect_vec();
    res
}

fn join(boxes: Vec<Vec<Vec<Vec<char>>>>) -> Vec<Vec<char>> {
    boxes
        .into_iter()
        .map(|r| {
            let len = r[0].len();
            r.into_iter().fold(vec![Vec::new(); len], |mut acc, v| {
                for i in 0..v.len() {
                    acc[i].extend(v[i].to_vec());
                }
                acc
            })
        })
        .fold(Vec::new(), |acc, v| acc.into_iter().chain(v).collect_vec())
}

fn do_iteration(
    grid: &[Vec<char>],
    map: &HashMap<Vec<Vec<char>>, Vec<Vec<char>>>,
    cache: &mut HashMap<Vec<Vec<char>>, Vec<Vec<char>>>,
) -> Vec<Vec<char>> {
    let div = if grid.len() % 2 == 0 { 2 } else { 3 };
    let boxes = split(grid, div)
        .into_iter()
        .map(|r| r.into_iter().map(|b| mutate(&b, map, cache)).collect_vec())
        .collect_vec();
    join(boxes)
}

fn part1(input: &str) -> impl Debug {
    let map = input
        .lines()
        .map(|s| s.split_once(" => ").unwrap())
        .map(|(input, output)| {
            (
                input
                    .split('/')
                    .map(|s| s.chars().collect_vec())
                    .collect_vec(),
                output
                    .split('/')
                    .map(|s| s.chars().collect_vec())
                    .collect_vec(),
            )
        })
        .collect::<HashMap<_, _>>();

    let mut grid = grid(".#.\n..#\n###".trim());
    let mut cache = HashMap::new();

    for _ in 0..5 {
        grid = do_iteration(&grid, &map, &mut cache);
    }

    grid.into_iter().flatten().filter(|c| *c == '#').count()
}

fn part2(input: &str) -> impl Debug {
    let map = input
        .lines()
        .map(|s| s.split_once(" => ").unwrap())
        .map(|(input, output)| {
            (
                input
                    .split('/')
                    .map(|s| s.chars().collect_vec())
                    .collect_vec(),
                output
                    .split('/')
                    .map(|s| s.chars().collect_vec())
                    .collect_vec(),
            )
        })
        .collect::<HashMap<_, _>>();

    let mut grid = grid(".#.\n..#\n###".trim());
    let mut cache = HashMap::new();

    for _ in 0..18 {
        grid = do_iteration(&grid, &map, &mut cache);
    }

    grid.into_iter().flatten().filter(|c| *c == '#').count()
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/21")));
    println!("{:?}", part2(include_str!("../../input/21")));
}
