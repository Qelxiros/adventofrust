#![feature(let_chains)]

use std::{collections::*, fmt::Debug, str::FromStr};

use common::utils::*;
use itertools::Itertools;
use regex::Regex;
use serde_json::Value;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn part1(input: &str) -> impl Debug {
    integers_signed::<isize>(input).into_iter().sum::<isize>()
}

fn process(v: Value) -> isize {
    match v {
        Value::Null | Value::Bool(_) | Value::String(_) => 0,
        Value::Number(n) => n.as_i64().unwrap() as isize,
        Value::Array(a) => a.into_iter().map(process).sum(),
        Value::Object(o) => {
            if o.values().contains(&Value::String("red".to_owned())) {
                0
            } else {
                o.values().cloned().map(process).sum()
            }
        }
    }
}

fn part2(input: &str) -> impl Debug {
    let o = Value::from_str(input).unwrap();
    process(o)
}

fn main() {
    println!("{:?}", part1(include_str!("../../input/12")));
    println!("{:?}", part2(include_str!("../../input/12")));
}
