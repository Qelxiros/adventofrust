use std::collections::HashSet;

use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

fn part1() -> Result<()> {
    let input = include_str!("/home/jeremy/github/adventofrust/input/06");

    let guard = input
        .chars()
        .filter(|&c| c != '\n')
        .position(|c| c == '^')
        .unwrap();
    let grid: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let mut guard = (guard / grid.len(), guard % grid.len());

    let mut dir = 0;
    let mut positions = HashSet::new();
    positions.insert(guard);
    loop {
        match dir {
            0 => {
                if grid[guard.0 - 1][guard.1] == '#' {
                    dir += 1;
                    continue;
                }
                guard.0 -= 1;
                positions.insert(guard);
                if guard.0 == 0 {
                    break;
                }
            }
            1 => {
                if grid[guard.0][guard.1 + 1] == '#' {
                    dir += 1;
                    continue;
                }
                guard.1 += 1;
                positions.insert(guard);
                if guard.1 == grid[0].len() - 1 {
                    break;
                }
            }
            2 => {
                if grid[guard.0 + 1][guard.1] == '#' {
                    dir += 1;
                    continue;
                }
                guard.0 += 1;
                positions.insert(guard);
                if guard.0 == grid.len() - 1 {
                    break;
                }
            }
            3 => {
                if grid[guard.0][guard.1 - 1] == '#' {
                    dir = 0;
                    continue;
                }
                guard.1 -= 1;
                positions.insert(guard);
                if guard.1 == 0 {
                    break;
                }
            }
            _ => panic!(),
        }
    }
    println!("{}", positions.len());

    Ok(())
}

fn part2() -> Result<()> {
    let input = include_str!("/home/jeremy/github/adventofrust/input/06");

    let guard = input
        .chars()
        .filter(|&c| c != '\n')
        .position(|c| c == '^')
        .unwrap();
    let grid: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let guard = (guard / grid.len(), guard % grid.len());

    let mut valid = 0;

    for i in 0..grid.len() {
        'chom: for j in 0..grid[0].len() {
            let mut guard = guard;
            let mut grid = grid.clone();
            grid[i][j] = '#';
            let mut dir = 0;
            let mut positions = HashSet::new();
            positions.insert((guard, 0));

            'game: loop {
                match dir {
                    0 => {
                        if grid[guard.0 - 1][guard.1] == '#' {
                            dir += 1;
                            continue;
                        }
                        guard.0 -= 1;
                        if positions.contains(&(guard, dir)) {
                            valid += 1;
                            continue 'chom;
                        }
                        positions.insert((guard, dir));
                        if guard.0 == 0 {
                            break 'game;
                        }
                    }
                    1 => {
                        if grid[guard.0][guard.1 + 1] == '#' {
                            dir += 1;
                            continue;
                        }
                        guard.1 += 1;
                        if positions.contains(&(guard, dir)) {
                            valid += 1;
                            continue 'chom;
                        }
                        positions.insert((guard, dir));
                        if guard.1 == grid[0].len() - 1 {
                            break 'game;
                        }
                    }
                    2 => {
                        if grid[guard.0 + 1][guard.1] == '#' {
                            dir += 1;
                            continue;
                        }
                        guard.0 += 1;
                        if positions.contains(&(guard, dir)) {
                            valid += 1;
                            continue 'chom;
                        }
                        positions.insert((guard, dir));
                        if guard.0 == grid.len() - 1 {
                            break 'game;
                        }
                    }
                    3 => {
                        if grid[guard.0][guard.1 - 1] == '#' {
                            dir = 0;
                            continue;
                        }
                        guard.1 -= 1;
                        if positions.contains(&(guard, dir)) {
                            valid += 1;
                            continue 'chom;
                        }
                        positions.insert((guard, dir));
                        if guard.1 == 0 {
                            break 'game;
                        }
                    }
                    _ => panic!(),
                }
            }
        }
    }

    println!("{valid}");

    Ok(())
}

fn main() {
    part1().unwrap();
    part2().unwrap();
}
