use aoc_runner_derive::aoc;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Pos {
    pub x : i32,
    pub y : i32
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct PosCost {
    pub pos : Pos,
    pub cost : i32
}

impl Ord for PosCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.cost < other.cost {
            return std::cmp::Ordering::Greater;
        }
        else if self.cost > other.cost {
            return std::cmp::Ordering::Less;
        }
        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for PosCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(Self::cmp(self, other))
    }
}

fn parse_input(input: &str) -> Vec<Pos> {
    input.lines().map(|line| {
        let (xs, ys) = line.split_once(",").unwrap();
        Pos { x: xs.parse().unwrap(), y: ys.parse().unwrap() }
    }).collect()
}

fn get_path_cost(map : &HashSet<Pos>, width : i32, height : i32) -> Option<i32> {
    let mut heap = BinaryHeap::<PosCost>::new();
    let mut costs = HashMap::<Pos, i32>::new();
    let start = Pos{x:0, y:0};
    costs.insert(start.clone(), 0);
    heap.push(PosCost{pos: start, cost:0});
    while !heap.is_empty() {
        let top = heap.pop().unwrap();
        for off in [(-1,0), (1,0), (0,-1), (0,1)] {
            let next_pos = Pos { x:top.pos.x + off.0, y:top.pos.y + off.1 };
            let next_cost = top.cost + 1;
            if next_pos.x >= 0 && next_pos.x <= width && next_pos.y >= 0 && next_pos.y <= height {
                if !map.contains(&next_pos) {
                    let c = costs.get(&next_pos).unwrap_or(&i32::MAX);
                    if next_cost < *c {
                        costs.insert(next_pos.clone(), next_cost);
                        heap.push(PosCost{ pos:next_pos, cost:next_cost});    
                    }
                }
            }
        }
    }

    match costs.get(&Pos{x:width, y:height}) {
        Some(c) => Some(*c),
        None => None
    }
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> i32 {
    let bytes = parse_input(input);
    let mut map = HashSet::<Pos>::new();
    for i in 0..1024 {
        map.insert(bytes.get(i).unwrap().clone());
    }
    get_path_cost(&map, 70, 70).unwrap()
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> String {
    let bytes = parse_input(input);
    let mut map = HashSet::<Pos>::new();
    let mut i = 0;
    while i < 1024 {
        map.insert(bytes.get(i).unwrap().clone());
        i += 1;
    }

    loop {
        map.insert(bytes.get(i).unwrap().clone());
        if get_path_cost(&map, 70, 70).is_none() {
            break;
        }
        i += 1;
    }
    let byte = bytes.get(i).unwrap();
    format!("{},{}", byte.x, byte.y)

}
