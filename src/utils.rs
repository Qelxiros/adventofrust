use std::{fmt::Debug, str::FromStr};

use itertools::Itertools;
use regex::Regex;

pub fn grid(input: String) -> Vec<Vec<char>> {
    input.lines().map(|s| s.chars().collect_vec()).collect_vec()
}

pub fn integers<T>(input: String) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let re = Regex::new(r"-?\d+").unwrap();
    re.find_iter(&input)
        .map(|m| m.as_str().parse::<T>().unwrap())
        .collect_vec()
}
