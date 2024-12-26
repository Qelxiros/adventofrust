use std::fmt::Debug;

use itertools::Itertools;

fn part1() -> impl Debug {
    let input = include_str!("../../input/09");

    let mut file = true;
    let mut idx = 0;

    let mut fs = input
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| String::from(c).parse::<usize>().unwrap())
        .flat_map(|i| {
            if file {
                idx += 1;
                file = false;
                vec![idx as isize - 1; i]
            } else {
                file = true;
                vec![-1; i]
            }
        })
        .collect_vec();

    for idx in (0..fs.len()).rev() {
        let pos = fs.iter().position(|&i| i == -1).unwrap();
        if pos > fs.iter().rposition(|&i| i != -1).unwrap() {
            break;
        }
        fs.swap(pos, idx);
    }

    fs.into_iter()
        .take_while(|&i| i != -1)
        .enumerate()
        .map(|(idx, i)| idx * i as usize)
        .sum::<usize>()
}

fn part2() -> impl Debug {
    let input = include_str!("../../input/09");

    let mut file = true;
    let mut idx = 0;

    let mut fs = input
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| String::from(c).parse::<usize>().unwrap())
        .map(|i| {
            if file {
                idx += 1;
                file = false;
                (Some(idx - 1), i)
            } else {
                file = true;
                (None, i)
            }
        })
        .collect_vec();

    for idx in (0..idx).rev() {
        let block = fs.iter().position(|(i, _)| *i == Some(idx)).unwrap();
        if let Some(dest) = fs
            .iter()
            .position(|(i, len)| i.is_none() && *len >= fs[block].1)
        {
            if dest > block {
                continue;
            }
            fs.insert(dest, fs[block]);
            fs[block + 1].0 = None;
            if fs[dest + 1].1 == fs[block + 1].1 {
                fs.remove(dest + 1);
            } else {
                fs[dest + 1].1 -= fs[dest].1;
            }
        }
        fs = help(fs);
    }

    let fs = fs
        .into_iter()
        .flat_map(|(i, len)| vec![i; len])
        .collect_vec();

    fs.into_iter()
        .enumerate()
        .filter(|(_, i)| i.is_some())
        .map(|(idx, i)| idx * i.unwrap() as usize)
        .sum::<usize>()
}

fn help(v: Vec<(Option<i32>, usize)>) -> Vec<(Option<i32>, usize)> {
    let mut out = vec![v[0]];
    v.into_iter().skip(1).for_each(|e| {
        if out.last().unwrap().0.is_none() && e.0.is_none() {
            out.last_mut().unwrap().1 += e.1;
        } else {
            out.push(e);
        }
    });
    out
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}
