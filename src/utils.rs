use itertools::Itertools;
use num::{Integer, Signed, Unsigned};
use regex::Regex;
use std::{fmt::Debug, str::FromStr};

pub fn grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|s| s.chars().collect_vec()).collect_vec()
}

pub fn grid_idx(input: &str, search: char) -> (Vec<Vec<char>>, Option<(usize, usize)>) {
    let mut loc = None;
    (
        input
            .lines()
            .enumerate()
            .map(|(idx, s)| {
                s.chars()
                    .enumerate()
                    .map(|(i, c)| {
                        if c == search {
                            loc = Some((idx, i))
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

pub fn grid_idx_with<T>(
    input: &str,
    search: char,
    f: impl Fn(char) -> T,
) -> (Vec<Vec<T>>, Option<(usize, usize)>) {
    let mut loc = None;
    (
        input
            .lines()
            .enumerate()
            .map(|(idx, s)| {
                s.chars()
                    .enumerate()
                    .map(|(i, c)| {
                        if c == search {
                            loc = Some((idx, i))
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
