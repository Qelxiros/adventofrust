#![feature(let_chains)]

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Debug,
};

use common::utils::grid_idx;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct State {
    pos: (usize, usize),
    cost: usize,
    dir: (isize, isize),
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
            .then_with(|| self.dir.cmp(&other.dir))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn part1() -> impl Debug {
    let input = include_str!("../../input/16");

    let (grid, Some(indices)) = grid_idx(input, ['S', 'E']) else {
        panic!()
    };
    let idx = indices[0];
    let end_idx = indices[1];

    let mut q = BinaryHeap::new();
    let mut seen = HashSet::new();
    q.push(State {
        pos: idx,
        cost: 0,
        dir: (0, 1),
    });
    while let Some(State { pos, cost, dir }) = q.pop() {
        if seen.contains(&(pos, dir)) {
            continue;
        }
        seen.insert((pos, dir));
        if pos == end_idx {
            return cost;
        }
        for d in DIRS {
            if d == (-dir.0, -dir.1) {
                continue;
            }
            if let Ok(x) = <isize as std::convert::TryInto<usize>>::try_into(
                pos.0 as isize + d.0,
            ) && let Ok(y) =
                <isize as std::convert::TryInto<usize>>::try_into(
                    pos.1 as isize + d.1,
                )
                && grid
                    .get(x)
                    .and_then(|l| l.get(y))
                    .is_some_and(|c| *c != '#')
            {
                let inc = if dir == d { 1 } else { 1001 };
                let state = State {
                    pos: (x, y),
                    cost: cost + inc,
                    dir: d,
                };
                q.push(state);
            }
        }
    }

    0
}

fn part2() -> impl Debug {
    let input = include_str!("../../input/16");

    let (grid, Some(indices)) = grid_idx(input, ['S', 'E']) else {
        panic!()
    };
    let idx = indices[0];
    let end_idx = indices[1];

    let mut q = BinaryHeap::new();
    let mut pred = HashMap::<State, Vec<State>>::new();
    let mut seen = HashSet::new();
    let mut tiles = HashSet::new();
    let mut best_cost = None;
    q.push(State {
        pos: idx,
        cost: 0,
        dir: (0, 1),
    });
    tiles.insert(idx);
    tiles.insert(end_idx);
    while let Some(State { pos, cost, dir }) = q.pop() {
        if seen.contains(&(pos, dir)) {
            continue;
        }
        seen.insert((pos, dir));
        if pos == end_idx && best_cost.is_none_or(|c: usize| c == cost) {
            best_cost = Some(cost);
            let mut states = VecDeque::new();
            states.push_back(State { pos, cost, dir });
            while let Some(s) = states.pop_front() {
                if let Some(s) = pred.get(&s) {
                    tiles.extend(s.iter().map(|a| a.pos));
                    states.extend(s);
                }
            }
        }
        for d in DIRS {
            if d == (-dir.0, -dir.1) {
                continue;
            }
            if let Ok(x) = <isize as std::convert::TryInto<usize>>::try_into(
                pos.0 as isize + d.0,
            ) && let Ok(y) =
                <isize as std::convert::TryInto<usize>>::try_into(
                    pos.1 as isize + d.1,
                )
                && grid
                    .get(x)
                    .and_then(|l| l.get(y))
                    .is_some_and(|c| *c != '#')
            {
                let inc = if dir == d { 1 } else { 1001 };
                let state = State {
                    pos: (x, y),
                    cost: cost + inc,
                    dir: d,
                };
                q.push(state);
                if let Some(v) = pred.get_mut(&state) {
                    v.push(State { pos, cost, dir });
                } else {
                    pred.insert(state, vec![State { pos, cost, dir }]);
                }
            }
        }
    }

    for (idx, row) in grid.into_iter().enumerate() {
        let mut s = String::new();
        for (i, j) in row.into_iter().enumerate() {
            if j == '.' && tiles.contains(&(idx, i)) {
                s.push('O');
            } else {
                s.push(j)
            }
        }
    }

    tiles.len()
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}
