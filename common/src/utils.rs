use std::{fmt::Debug, ops::Add, str::FromStr, sync::LazyLock};

use itertools::Itertools;
use num::{Integer, Signed, Unsigned};
use regex::Regex;

pub fn grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|s| s.chars().collect_vec()).collect_vec()
}

#[allow(clippy::type_complexity)]
pub fn grid_idx(
    input: &str,
    search: &[char],
) -> (Vec<Vec<char>>, Vec<Option<(usize, usize)>>) {
    let mut loc = vec![None; search.len()];
    (
        input
            .lines()
            .enumerate()
            .map(|(idx, s)| {
                s.chars()
                    .enumerate()
                    .map(|(i, c)| {
                        if let Some(pos) = search.iter().position(|&a| a == c) {
                            loc[pos] = Some((idx, i));
                        }
                        c
                    })
                    .collect_vec()
            })
            .collect_vec(),
        loc,
    )
}

pub fn grid_with<T>(input: &str, f: impl Fn(char) -> T) -> Vec<Vec<T>> {
    input
        .lines()
        .map(|s| s.chars().map(&f).collect_vec())
        .collect_vec()
}

#[allow(clippy::type_complexity)]
pub fn grid_idx_with<T>(
    input: &str,
    search: &[char],
    f: impl Fn(char) -> T,
) -> (Vec<Vec<T>>, Vec<Option<(usize, usize)>>) {
    let mut loc = vec![None; search.len()];
    (
        input
            .lines()
            .enumerate()
            .map(|(idx, s)| {
                s.chars()
                    .enumerate()
                    .map(|(i, c)| {
                        if let Some(pos) = search.iter().position(|&a| a == c) {
                            loc[pos] = Some((idx, i))
                        }
                        f(c)
                    })
                    .collect_vec()
            })
            .collect_vec(),
        loc,
    )
}

static RE_SIGNED: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"-?\d+").unwrap());

pub fn integers_signed<T>(input: &str) -> Vec<T>
where
    T: Signed + Integer + FromStr,
    <T as FromStr>::Err: Debug,
{
    RE_SIGNED
        .find_iter(input)
        .map(|m| m.as_str().parse::<T>().unwrap())
        .collect_vec()
}

static RE_UNSIGNED: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\d+").unwrap());

pub fn integers_unsigned<T>(input: &str) -> Vec<T>
where
    T: Unsigned + Integer + FromStr,
    <T as FromStr>::Err: Debug,
{
    RE_UNSIGNED
        .find_iter(input)
        .map(|m| m.as_str().parse::<T>().unwrap())
        .collect_vec()
}

pub fn add2<T>(a: (T, T), b: (T, T)) -> (T, T)
where
    T: Add<Output = T>,
{
    (a.0 + b.0, a.1 + b.1)
}

pub fn add3<T>(a: (T, T, T), b: (T, T, T)) -> (T, T, T)
where
    T: Add<Output = T>,
{
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

pub fn extended_euclid(a: isize, b: isize) -> (isize, (isize, isize)) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
        (old_t, t) = (t, old_t - quotient * t);
    }

    (old_r, (old_s, old_t))
}

pub fn crt(vals: &[isize], moduli: &[isize]) -> isize {
    if vals.len() != moduli.len() {
        panic!()
    }
    let (_, (m1, m2)) = extended_euclid(vals[0], vals[1]);
    let x = (m1 * vals[0] * moduli[1] + m2 * vals[1] * moduli[0])
        .rem_euclid(vals[0] * vals[1]);
    if vals.len() == 2 {
        x
    } else {
        let mut v = vec![vals[0] * vals[1]];
        v.extend(&vals[2..]);
        let mut m = vec![x.rem_euclid(vals[0] * vals[1])];
        m.extend(&moduli[2..]);

        crt(&v, &m)
    }
}
