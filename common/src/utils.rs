use std::{fmt::Debug, str::FromStr};

use itertools::Itertools;
use num::{Integer, Signed, Unsigned};
use regex::Regex;

pub fn grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|s| s.chars().collect_vec()).collect_vec()
}

#[allow(clippy::type_complexity)]
pub fn grid_idx<const N: usize>(
    input: &str,
    search: [char; N],
) -> (Vec<Vec<char>>, Option<Vec<(usize, usize)>>) {
    let mut loc = vec![None; N];
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
        #[allow(irrefutable_let_patterns)]
        if let loc = loc.into_iter().flatten().collect_vec()
            && loc.len() == N
        {
            Some(loc)
        } else {
            None
        },
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

pub fn integers_signed<T>(input: &str) -> Vec<T>
where
    T: Signed + Integer + FromStr,
    <T as FromStr>::Err: Debug,
{
    let re = Regex::new(r"-?\d+").unwrap();
    re.find_iter(input)
        .map(|m| m.as_str().parse::<T>().unwrap())
        .collect_vec()
}

pub fn integers_unsigned<T>(input: &str) -> Vec<T>
where
    T: Unsigned + Integer + FromStr,
    <T as FromStr>::Err: Debug,
{
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(input)
        .map(|m| m.as_str().parse::<T>().unwrap())
        .collect_vec()
}
