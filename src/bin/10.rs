use std::{
    collections::{HashSet, VecDeque},
    fs,
};

use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

fn get_neighbors(map: &Vec<Vec<u8>>, row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    for j in -1..=1 {
        if j == 0 {
            continue;
        }
        if col as isize + j >= 0 && col as isize + j < map[0].len() as isize {
            res.push((row, (col as isize + j) as usize));
        }
    }
    for i in -1..=1 {
        if i == 0 {
            continue;
        }
        if row as isize + i >= 0 && row as isize + i < map.len() as isize {
            res.push(((row as isize + i) as usize, col));
        }
    }

    res
}

fn score(map: &Vec<Vec<u8>>, head: (usize, usize)) -> usize {
    let mut q = VecDeque::new();
    let mut set = HashSet::new();
    q.push_back(head);

    while let Some(cur) = q.pop_front() {
        if map[cur.0][cur.1] == 9 {
            set.insert(cur);
            continue;
        }
        for n in get_neighbors(map, cur.0, cur.1) {
            if map[cur.0][cur.1] + 1 == map[n.0][n.1] {
                q.push_back(n);
            }
        }
    }

    set.len()
}

fn part1() -> Result<()> {
    let input = include_str!("/home/jeremy/github/adventofrust/input/10");

    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| String::from(c).parse().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut heads = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 0 {
                heads.push((i, j));
            }
        }
    }

    let out = heads.into_iter().map(|n| score(&grid, n)).sum::<usize>();

    println!("{out}");

    Ok(())
}

fn rate(map: &Vec<Vec<u8>>, head: (usize, usize)) -> usize {
    let mut q = VecDeque::new();
    let mut score = 0;
    q.push_back(head);

    while let Some(cur) = q.pop_front() {
        if map[cur.0][cur.1] == 9 {
            score += 1;
            continue;
        }
        for n in get_neighbors(map, cur.0, cur.1) {
            if map[cur.0][cur.1] + 1 == map[n.0][n.1] {
                q.push_back(n);
            }
        }
    }

    score
}
fn part2() -> Result<()> {
    let input = include_str!("/home/jeremy/github/adventofrust/input/10");

    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| String::from(c).parse().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut heads = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 0 {
                heads.push((i, j));
            }
        }
    }

    let out = heads.into_iter().map(|n| rate(&grid, n)).sum::<usize>();

    println!("{out}");
    Ok(())
}

fn main() {
    part1().unwrap();
    part2().unwrap();
}
