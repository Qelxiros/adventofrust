use std::{collections::HashSet, fmt::Debug};

fn check(
    mut guard: (usize, usize),
    grid: &[Vec<char>],
    extra: Option<(usize, usize)>,
) -> Option<HashSet<(usize, usize)>> {
    let mut grid = grid.to_owned();
    if let Some((x, y)) = extra {
        grid[x][y] = '#';
    }
    let mut dir = 0;
    let mut positions = HashSet::new();
    positions.insert((guard, dir));
    loop {
        match dir {
            0 => {
                if grid[guard.0 - 1][guard.1] == '#' {
                    dir += 1;
                    continue;
                }
                guard.0 -= 1;
                if !positions.insert((guard, dir)) {
                    return None;
                }
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
                if !positions.insert((guard, dir)) {
                    return None;
                }
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
                if !positions.insert((guard, dir)) {
                    return None;
                }
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
                if !positions.insert((guard, dir)) {
                    return None;
                }
                if guard.1 == 0 {
                    break;
                }
            }
            _ => panic!(),
        }
    }

    Some(
        positions
            .into_iter()
            .map(|(guard, _)| guard)
            .collect::<HashSet<_>>(),
    )
}

fn part1() -> impl Debug {
    let input = include_str!("../../input/06");

    let guard = input
        .chars()
        .filter(|&c| c != '\n')
        .position(|c| c == '^')
        .unwrap();
    let grid: Vec<Vec<char>> =
        input.lines().map(|s| s.chars().collect()).collect();
    let guard = (guard / grid.len(), guard % grid.len());

    let positions = check(guard, &grid, None);

    positions.unwrap().len()
}

fn part2() -> impl Debug {
    let input = include_str!("../../input/06");

    let guard = input
        .chars()
        .filter(|&c| c != '\n')
        .position(|c| c == '^')
        .unwrap();
    let grid: Vec<Vec<char>> =
        input.lines().map(|s| s.chars().collect()).collect();
    let guard = (guard / grid.len(), guard % grid.len());

    check(guard, &grid, None)
        .unwrap()
        .into_iter()
        .filter(|g| check(guard, &grid, Some(*g)).is_none())
        .count()
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}
