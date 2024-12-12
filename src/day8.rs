use aoc_runner_derive::aoc;
use std::collections::*;
use itertools::Itertools;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Coord {
    pub x : i32,
    pub y : i32
}

fn parse_input(input: &str) -> (i32, i32, HashMap<char, Vec<Coord>>) {
    let mut antenas = HashMap::<char, Vec<Coord>>::new();
    let mut y = 0;
    let mut max_x = 0;
    for line in input.lines() {
        let mut x = 0;
        for c in line.chars() {
            if c != '.' {
                antenas.entry(c).or_insert(Vec::<Coord>::new()).push(Coord{x,y});
            }
            x += 1;
        }
        max_x = x;
        y += 1;
    }
    (max_x, y, antenas)
}

fn get_node_coord(max_x : i32, max_y : i32, a : &Coord, b : &Coord) -> Option<Coord> {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let c = Coord {
        x : b.x + dx,
        y : b.y + dy
    };
    if c.x >= 0 && c.x < max_x && c.y >= 0 && c.y < max_y {
        return Some(c);
    }
    None
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> i32 {
    let (max_x, max_y, antenas) = parse_input(input);
    let mut nodes = HashSet::<Coord>::with_capacity(1000);
    for (_, coords) in antenas {
        for coord_pair in coords.into_iter().combinations(2) {
            let a = coord_pair.get(0).unwrap();
            let b = coord_pair.get(1).unwrap();
            if let Some(node_a) = get_node_coord(max_x, max_y, a, b) {
                nodes.insert(node_a);
            }
            if let Some(node_b) = get_node_coord(max_x, max_y, b, a) {
                nodes.insert(node_b);
            }
        }
    }
    nodes.len() as i32
}

fn add_nodes(max_x : i32, max_y : i32, nodes : &mut HashSet<Coord>, a : &Coord, b : &Coord) {
    //println!("add_nodes {:?} -> {:?}", a, b);

    nodes.insert(a.clone());
    nodes.insert(b.clone());

    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let mut c = Coord {
        x : b.x + dx,
        y : b.y + dy
    };
    while c.x >= 0 && c.x < max_x && c.y >= 0 && c.y < max_y {
        nodes.insert(c.clone());
        //println!("added node {:?}", c);
        c.x += dx;
        c.y += dy;
    }
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> i32 {
    let (max_x, max_y, antenas) = parse_input(input);
    let mut nodes = HashSet::<Coord>::with_capacity(1000);
    for (_, coords) in antenas {
        for coord_pair in coords.into_iter().combinations(2) {
            let a = coord_pair.get(0).unwrap();
            let b = coord_pair.get(1).unwrap();
            add_nodes(max_x, max_y, &mut nodes, a, b);
            add_nodes(max_x, max_y, &mut nodes, b, a);
        }
    }
    //println!("{:?}", nodes);
    nodes.len() as i32
}

// #[test]
// fn test_part2() {
//     assert_eq!(part2("T.........
// ...T......
// .T........
// ..........
// ..........
// ..........
// ..........
// ..........
// ..........
// .........."), 9);
// }