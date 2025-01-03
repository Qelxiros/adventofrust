use std::{collections::VecDeque, fmt::Debug};

use itertools::Itertools;

fn move_robot(
    grid: &mut [Vec<isize>],
    robot: (isize, isize),
    dir: (isize, isize),
) -> (isize, isize) {
    let mut boxes = vec![(robot, 2)];
    let mut entity = robot;
    loop {
        if grid[(entity.0 + dir.0) as usize][(entity.1 + dir.1) as usize] == 0 {
            for e in boxes.into_iter().rev() {
                grid[e.0.0 as usize][e.0.1 as usize] = 0;
                grid[(e.0.0 + dir.0) as usize][(e.0.1 + dir.1) as usize] = e.1;
            }
            return (robot.0 + dir.0, robot.1 + dir.1);
        }
        if grid[(entity.0 + dir.0) as usize][(entity.1 + dir.1) as usize] == 1 {
            boxes.push(((entity.0 + dir.0, entity.1 + dir.1), 1));
            entity = (entity.0 + dir.0, entity.1 + dir.1);
        }
        if grid[(entity.0 + dir.0) as usize][(entity.1 + dir.1) as usize]
            == isize::MAX
        {
            return robot;
        }
    }
}

fn part1() -> impl Debug {
    let input = include_str!("../../input/15");
    let halves = input.split("\n\n").take(2).collect_vec();

    let mut grid = halves[0]
        .trim()
        .lines()
        .map(|s| {
            s.trim()
                .chars()
                .map(|c| match c {
                    '#' => isize::MAX,
                    '.' => 0,
                    'O' => 1,
                    '@' => 2,
                    _ => panic!(),
                })
                .collect_vec()
        })
        .collect_vec();
    let mut robot = grid
        .iter()
        .enumerate()
        .filter_map(|(idx, v)| {
            v.iter()
                .enumerate()
                .find(|(_, c)| **c == 2)
                .map(|(i, _)| (idx as isize, i as isize))
        })
        .next()
        .unwrap();

    halves[1]
        .trim()
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            c => panic!("{c}"),
        })
        .for_each(|m| robot = move_robot(&mut grid, robot, m));

    grid.into_iter()
        .enumerate()
        .map(|(idx, v)| {
            v.into_iter()
                .enumerate()
                .map(|(i, c)| if c == 1 { 100 * idx + i } else { 0 })
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn move_robot2(
    grid: &mut [Vec<char>],
    robot: (isize, isize),
    dir: (isize, isize),
) -> (isize, isize) {
    let mut boxes = vec![(robot, '@')];
    let mut entities = VecDeque::new();
    entities.push_back(robot);
    while let Some(entity) = entities.pop_front() {
        if grid[(entity.0 + dir.0) as usize][(entity.1 + dir.1) as usize] == '['
        {
            boxes.push(((entity.0 + dir.0, entity.1 + dir.1), '['));
            boxes.push(((entity.0 + dir.0, entity.1 + dir.1 + 1), ']'));
            entities.push_back((entity.0 + dir.0, entity.1 + dir.1));
            if dir.1 == 0 {
                entities.push_back((entity.0 + dir.0, entity.1 + dir.1 + 1));
            }
        }
        if grid[(entity.0 + dir.0) as usize][(entity.1 + dir.1) as usize] == ']'
        {
            boxes.push(((entity.0 + dir.0, entity.1 + dir.1), ']'));
            boxes.push(((entity.0 + dir.0, entity.1 + dir.1 - 1), '['));
            entities.push_back((entity.0 + dir.0, entity.1 + dir.1));
            if dir.1 == 0 {
                entities.push_back((entity.0 + dir.0, entity.1 + dir.1 - 1));
            }
        }
        if grid[(entity.0 + dir.0) as usize][(entity.1 + dir.1) as usize] == '#'
        {
            return robot;
        }
    }
    for e in boxes.into_iter().rev() {
        grid[e.0.0 as usize][e.0.1 as usize] = '.';
        grid[(e.0.0 + dir.0) as usize][(e.0.1 + dir.1) as usize] = e.1;
    }
    (robot.0 + dir.0, robot.1 + dir.1)
}

fn part2() -> impl Debug {
    let input = include_str!("../../input/15");
    let halves = input.split("\n\n").take(2).collect_vec();

    let mut grid = halves[0]
        .trim()
        .lines()
        .map(|s| {
            s.trim()
                .chars()
                .map(|c| match c {
                    '#' => "##",
                    '.' => "..",
                    'O' => "[]",
                    '@' => "@.",
                    _ => panic!(),
                })
                .collect::<String>()
                .chars()
                .collect_vec()
        })
        .collect_vec();
    let mut robot = grid
        .iter()
        .enumerate()
        .filter_map(|(idx, v)| {
            v.iter()
                .enumerate()
                .find(|(_, c)| **c == '@')
                .map(|(i, _)| (idx as isize, i as isize))
        })
        .next()
        .unwrap();

    halves[1]
        .trim()
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            c => panic!("{c}"),
        })
        .for_each(|m| {
            robot = move_robot2(&mut grid, robot, m);
        });

    grid.into_iter()
        .enumerate()
        .map(|(idx, v)| {
            v.into_iter()
                .enumerate()
                .map(|(i, c)| if c == '[' { 100 * idx + i } else { 0 })
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}
