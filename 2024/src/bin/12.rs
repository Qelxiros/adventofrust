use std::{
    collections::{HashSet, VecDeque},
    fmt::Debug,
};

use itertools::Itertools;

fn neighbors(pt: (usize, usize), max: (usize, usize)) -> Vec<(usize, usize)> {
    let mut v = Vec::new();
    if pt.0 > 0 {
        v.push((pt.0 - 1, pt.1));
    }
    if pt.0 < max.0 - 1 {
        v.push((pt.0 + 1, pt.1));
    }
    if pt.1 > 0 {
        v.push((pt.0, pt.1 - 1));
    }
    if pt.1 < max.1 - 1 {
        v.push((pt.0, pt.1 + 1));
    }

    v
}

fn perim(grid: &[Vec<char>], pt: (usize, usize)) -> usize {
    let c = grid[pt.0][pt.1];
    4 - neighbors(pt, (grid.len(), grid[0].len()))
        .into_iter()
        .filter(|(x, y)| grid[*x][*y] == c)
        .count()
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct SidePart {
    dir: usize,
    idx: (usize, usize),
}

fn neighbors_perim(grid: &[Vec<char>], pt: (usize, usize)) -> Vec<SidePart> {
    let mut v = Vec::new();
    let max = (grid.len(), grid[0].len());
    let c = grid[pt.0][pt.1];
    if pt.0 == 0 || grid[pt.0 - 1][pt.1] != c {
        v.push(SidePart {
            dir: 0,
            idx: (pt.0, pt.1),
        });
    }
    if pt.0 >= max.0 - 1 || grid[pt.0 + 1][pt.1] != c {
        v.push(SidePart {
            dir: 2,
            idx: (pt.0, pt.1),
        });
    }
    if pt.1 == 0 || grid[pt.0][pt.1 - 1] != c {
        v.push(SidePart {
            dir: 1,
            idx: (pt.0, pt.1),
        });
    }
    if pt.1 >= max.1 - 1 || grid[pt.0][pt.1 + 1] != c {
        v.push(SidePart {
            dir: 3,
            idx: (pt.0, pt.1),
        });
    }

    v
}

fn merge(v: HashSet<SidePart>) -> usize {
    let (a, c): (Vec<_>, Vec<_>) = v.into_iter().partition(|s| s.dir % 2 == 0);
    let (mut a, mut b): (Vec<_>, Vec<_>) =
        a.into_iter().partition(|s| s.dir > 1);
    let (mut c, mut d): (Vec<_>, Vec<_>) =
        c.into_iter().partition(|s| s.dir > 1);

    a.sort_unstable_by(|a, b| {
        a.idx.0.cmp(&b.idx.0).then_with(|| a.idx.1.cmp(&b.idx.1))
    });
    b.sort_unstable_by(|a, b| {
        a.idx.0.cmp(&b.idx.0).then_with(|| a.idx.1.cmp(&b.idx.1))
    });
    c.sort_unstable_by(|a, b| {
        a.idx.1.cmp(&b.idx.1).then_with(|| a.idx.0.cmp(&b.idx.0))
    });
    d.sort_unstable_by(|a, b| {
        a.idx.1.cmp(&b.idx.1).then_with(|| a.idx.0.cmp(&b.idx.0))
    });

    let mut total = 0;

    for v in vec![a, b].into_iter().filter(|v| !v.is_empty()) {
        let mut new = vec![v[0]];
        for e in v.into_iter().skip(1) {
            if new.last().unwrap().idx.1.abs_diff(e.idx.1) == 1
                && new.last().unwrap().idx.0 == e.idx.0
            {
                *new.last_mut().unwrap() = e;
            } else {
                new.push(e);
            }
        }
        total += new.len();
    }
    for v in vec![c, d].into_iter().filter(|v| !v.is_empty()) {
        let mut new = vec![v[0]];
        for e in v.into_iter().skip(1) {
            if new.last().unwrap().idx.0.abs_diff(e.idx.0) == 1
                && new.last().unwrap().idx.1 == e.idx.1
            {
                *new.last_mut().unwrap() = e;
            } else {
                new.push(e);
            }
        }
        total += new.len();
    }

    total
}

fn region(
    grid: &[Vec<char>],
    start: (usize, usize),
) -> (usize, HashSet<(usize, usize)>) {
    let mut q = VecDeque::new();
    let mut region = HashSet::new();
    q.push_back(start);
    while let Some(p) = q.pop_front() {
        if region.contains(&p) {
            continue;
        }
        if grid[p.0][p.1] == grid[start.0][start.1] {
            q.extend(neighbors(p, (grid.len(), grid[0].len())));
            region.insert(p);
        }
    }

    let area = region.len();
    let perimeter = region
        .clone()
        .into_iter()
        .map(|p| perim(grid, p))
        .sum::<usize>();

    (area * perimeter, region)
}

fn region2(
    grid: &[Vec<char>],
    start: (usize, usize),
) -> (usize, HashSet<(usize, usize)>) {
    let mut q = VecDeque::new();
    let mut region = HashSet::new();
    q.push_back(start);
    while let Some(p) = q.pop_front() {
        if region.contains(&p) {
            continue;
        }
        if grid[p.0][p.1] == grid[start.0][start.1] {
            q.extend(neighbors(p, (grid.len(), grid[0].len())));
            region.insert(p);
        }
    }

    let area = region.len();
    let perimeter = region
        .clone()
        .into_iter()
        .flat_map(|p| neighbors_perim(grid, p))
        .collect::<HashSet<_>>();
    let perimeter = merge(perimeter);

    (area * perimeter, region)
}

fn part1() -> impl Debug {
    let input = include_str!("../../input/12");

    let grid = input.lines().map(|s| s.chars().collect_vec()).collect_vec();
    let mut set = HashSet::new();
    let mut total = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if set.contains(&(i, j)) {
                continue;
            }

            let (t, s) = region(&grid, (i, j));

            total += t;
            set.extend(s);
        }
    }

    total
}

fn part2() -> impl Debug {
    let input = include_str!("../../input/12");

    let grid = input.lines().map(|s| s.chars().collect_vec()).collect_vec();
    let mut set = HashSet::new();
    let mut total = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if set.contains(&(i, j)) {
                continue;
            }

            let (t, s) = region2(&grid, (i, j));

            total += t;
            set.extend(s);
        }
    }

    total
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}
