use std::{
    cell::RefCell,
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    iter,
    rc::Rc,
};

use itertools::Itertools;
use num::integer::lcm;
use regex::Regex;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    position: (usize, usize),
    a: usize,
    b: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(goal: (usize, usize), A: (usize, usize), B: (usize, usize)) -> Option<usize> {
    let mut dist = HashMap::new();

    let mut heap = BinaryHeap::new();

    heap.push(State {
        cost: 0,
        position: (0, 0),
        a: 0,
        b: 0,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State {
        cost,
        position,
        a,
        b,
    }) = heap.pop()
    {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(cost);
        }

        if position.0 > goal.0 || position.1 > goal.1 {
            continue;
        }

        // Important as we may have already found a better way
        if dist.contains_key(&position) && cost > dist[&position] {
            continue;
        }

        let next_a = State {
            cost: cost + 3,
            position: (position.0 + A.0, position.1 + A.1),
            a: a + 1,
            b,
        };
        if !dist.contains_key(&next_a.position) || next_a.cost < dist[&next_a.position] {
            heap.push(next_a);
            // Relaxation, we have now found a better way
            dist.insert(next_a.position, next_a.cost);
        }
        let next_b = State {
            cost: cost + 1,
            position: (position.0 + B.0, position.1 + B.1),
            a,
            b: b + 1,
        };
        if !dist.contains_key(&next_b.position) || next_b.cost < dist[&next_b.position] {
            heap.push(next_b);
            // Relaxation, we have now found a better way
            dist.insert(next_b.position, next_b.cost);
        }
    }

    // Goal not reachable
    None
}

fn part1() {
    let input = include_str!("/home/jeremy/github/adventofrust/input/13");

    let re = Regex::new(r"\d+").unwrap();

    let out = input
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|s| {
            let v = re
                .find_iter(s)
                .map(|m| m.as_str().parse::<usize>().unwrap())
                .take(6)
                .collect::<Vec<_>>();
            shortest_path(
                (v[4] + 10000000000000, v[5] + 10000000000000),
                (v[0], v[1]),
                (v[2], v[3]),
            )
            .unwrap_or(0)
        })
        .sum::<usize>();

    println!("{out}");
}

fn find(C: (usize, usize), A: (usize, usize), B: (usize, usize)) -> Option<usize> {
    let d = (A.0 * B.1) as isize - (A.1 * B.0) as isize;
    if d == 0 {
        return None;
    }

    let x = (B.1 * C.0) as isize - (B.0 * C.1) as isize;
    let y = (A.0 * C.1) as isize - (A.1 * C.0) as isize;

    if x % d != 0 || y % d != 0 {
        None
    } else {
        let r = 3 * x / d + y / d;
        if r < 0 { None } else { Some(r as usize) }
    }
}

fn part2() {
    let input = include_str!("/home/jeremy/github/adventofrust/input/13");

    let re = Regex::new(r"\d+").unwrap();

    let out = input
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .filter_map(|s| {
            let v = re
                .find_iter(s)
                .map(|m| m.as_str().parse::<usize>().unwrap())
                .take(6)
                .collect::<Vec<_>>();
            find(
                (v[4] + 10000000000000, v[5] + 10000000000000),
                (v[0], v[1]),
                (v[2], v[3]),
            )
        })
        .sum::<usize>();

    println!("{out}");
}

fn main() {
    // part1();
    part2();
}
