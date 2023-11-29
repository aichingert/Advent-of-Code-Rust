// Advent of Code 2018, day 22
// (c) aichingert

use std::cmp::{Reverse, Ordering};
use std::collections::{HashMap, HashSet, BinaryHeap};

fn geologic_index(gi: &mut HashMap<(i64, i64), i64>, x: i64, y: i64, depth: i64) -> i64 {
    if gi.contains_key(&(x, y)) {
        return gi[&(x, y)];
    }

    let geoi = match (x, y) {
        (0, _) => y * 48271,
        (_, 0) => x * 16807,
        (_, _) => {
            let a = (geologic_index(gi, x - 1, y, depth) + depth) % 20183;
            let b = (geologic_index(gi, x, y - 1, depth) + depth) % 20183;

            a * b
        }
    };
    gi.insert((x, y), geoi);

    gi[&(x, y)]
}

fn part_one(depth: i64, x: i64, y: i64, gi: &mut HashMap<(i64, i64), i64>) -> i64 {
    (0..=y)
        .map(|i| (0..=x)
             .map(|j| (geologic_index(gi, j, i, depth) + depth) % 20183 % 3)
             .sum::<i64>())
        .sum::<i64>()
}

#[derive(Debug, Eq)]
struct State {
    x: i64,
    y: i64,
    d: i64,
    t: Tool,
}

impl State {
    fn new(x: i64, y: i64, d: i64, t: Tool) -> Self {
        Self {
            x,
            y,
            d,
            t,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.d.cmp(&other.d)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.d == other.d
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
enum Tool {
    Torch = 0,
    Gear = 1,
    Neither = 2,
}

// 0, Rocky
// 1, Wet
// 2, Narrow
impl Tool {
    fn has_to_switch(&self, cave_type: i64) -> Option<(Tool, Tool)> {
        match (self, cave_type) {
            (Tool::Gear, 0) | (Tool::Gear, 1) => None,
            (Tool::Gear, 2) => Some((Tool::Torch, Tool::Neither)),
            (Tool::Torch, 0) | (Tool::Torch, 2) => None,
            (Tool::Torch, 1) => Some((Tool::Gear, Tool::Neither)),
            (Tool::Neither, 0) => Some((Tool::Torch, Tool::Gear)),
            (Tool::Neither, 1) | (Tool::Neither, 2) => None,
            _ => unreachable!("not possible"),
        }
    }

    fn get_matching_tool(&self, cave_type: i64) -> Tool {
        match (self, cave_type) {
            (Tool::Gear, 0) => Tool::Torch,
            (Tool::Gear, 1) => Tool::Neither,
            (Tool::Torch, 0) => Tool::Gear,
            (Tool::Torch, 2) => Tool::Neither,
            (Tool::Neither, 1) => Tool::Gear,
            (Tool::Neither, 2) => Tool::Torch,
            _ => unreachable!("not possible"),
        }
    }
}

fn part_two(depth: i64, goal: (i64, i64), gi: &mut HashMap<(i64, i64), i64>) -> i64 {
    let mut heap = BinaryHeap::from([Reverse(State::new(0,0,0, Tool::Torch))]);
    let mut seen = HashSet::new();
    let mut ans = i64::MAX;

    while let Some(Reverse(s)) = heap.pop() {
        if goal == (s.x, s.y) && s.t == Tool::Torch {
            let max = heap.iter().max_by_key(|k| k.0.d).unwrap();
            println!("{:?}", max);
            ans = ans.min(s.d);
        }

        if !seen.insert((s.x, s.y, s.t)) {
            continue;
        }

        for (x, y) in [(0,1),(1,0),(0,-1),(-1,0)] {
            let (nx, ny) = (s.x + x, s.y + y);

            if nx < 0 || ny < 0 || nx > goal.0 * 10 || ny > goal.1 * 10 {
                continue;
            }

            let dist = s.d + 8;
            let cave_type = (geologic_index(gi, nx, ny, depth) + depth) % 20183 % 3;

            if let Some((one, two)) = s.t.has_to_switch(cave_type) {
                heap.push(Reverse(State::new(nx, ny, dist, one)));
                heap.push(Reverse(State::new(nx, ny, dist, two)));
            } else {
                heap.push(Reverse(State::new(nx, ny, dist - 7, s.t)));
                heap.push(Reverse(State::new(nx, ny, dist, s.t.get_matching_tool(cave_type))));
            }
        }
    }

    println!("{}", ans);
    panic!("help");
}

fn main() {
    let inp = std::fs::read_to_string("../input/22").unwrap().trim().to_string();
    let (depth, target) = inp.split_once('\n').unwrap();

    let depth: i64 = depth[7..].parse().unwrap();
    let (x, y) = target.split_once(',').unwrap();
    let (x, y) = (x[8..].parse::<i64>().unwrap(), y.parse::<i64>().unwrap());

    let mut geologic_index: HashMap<(i64, i64), i64> = HashMap::from([((0, 0), 0), ((x, y), 0)]);

    println!("Part 1: {}", part_one(depth, x, y, &mut geologic_index));
    println!("Part 2: {}", part_two(depth, (x, y), &mut geologic_index));
}
