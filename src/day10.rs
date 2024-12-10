use aoc_runner_derive::aoc;
use std::collections::*;

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct Coord {
    pub x : i32,
    pub y : i32
}

fn parse_input(input: &str) -> (HashMap<Coord, i32>, Vec<Coord>, i32, i32) {
    let mut coords = HashMap::<Coord, i32>::new();
    let mut trailheads = vec!();
    let mut max_x = 0;

    let mut y = 0;
    for line in input.lines() {
        let mut x = 0;
        for c in line.chars() {
            let val = c as i32 - '0' as i32;
            coords.insert(Coord{x, y}, val);
            if val == 0 {
                trailheads.push(Coord{x, y});
            }
            x += 1;
        }
        y += 1;
        max_x = x;
    }
    (coords, trailheads, max_x, y)
}

fn get_trails(coords : &HashMap<Coord, i32>, max_x : i32, max_y : i32, c : &Coord, v : i32, trailends : &mut HashSet<Coord>) {
    if c.x < 0 || c.x >= max_x || c.y < 0 || c.y >= max_y {
        return;
    }

    if *coords.get(&c).unwrap() != v {
        return;
    }

    // println!("{:indent$}{:?} = {}", "", c, v, indent=v as usize);
    if v == 9 {
        trailends.insert(c.clone());
    }

    get_trails(coords, max_x, max_y, &Coord{x : c.x + 1, y : c.y}, v + 1, trailends);
    get_trails(coords, max_x, max_y, &Coord{x : c.x - 1, y : c.y}, v + 1, trailends); 
    get_trails(coords, max_x, max_y, &Coord{x : c.x, y : c.y + 1}, v + 1, trailends);
    get_trails(coords, max_x, max_y, &Coord{x : c.x, y : c.y - 1}, v + 1, trailends);
}

fn get_trailhead_rating(coords : &HashMap<Coord, i32>, max_x : i32, max_y : i32, c : &Coord, v : i32) -> i32 {
    if c.x < 0 || c.x >= max_x || c.y < 0 || c.y >= max_y {
        return 0;
    }

    if *coords.get(&c).unwrap() != v {
        return 0;
    }

    // println!("{:indent$}{:?} = {}", "", c, v, indent=v as usize);
    if v == 9 {
        return 1;
    }

    return 
        get_trailhead_rating(coords, max_x, max_y, &Coord{x : c.x + 1, y : c.y}, v + 1) +
        get_trailhead_rating(coords, max_x, max_y, &Coord{x : c.x - 1, y : c.y}, v + 1) +
        get_trailhead_rating(coords, max_x, max_y, &Coord{x : c.x, y : c.y + 1}, v + 1) +
        get_trailhead_rating(coords, max_x, max_y, &Coord{x : c.x, y : c.y - 1}, v + 1);
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> i32 {
    let (coords, trailheads, max_x, max_y) = parse_input(input);
    let mut acc = 0;
    for trailhead in trailheads {
        let mut trailends = HashSet::<Coord>::new();
        get_trails(&coords, max_x, max_y, &trailhead, 0, &mut trailends);
        acc += trailends.len() as i32;
    }
    acc
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> i32 {
    let (coords, trailheads, max_x, max_y) = parse_input(input);
    let mut acc = 0;
    for trailhead in trailheads {
        let rating = get_trailhead_rating(&coords, max_x, max_y, &trailhead, 0);
        acc += rating;
    }
    acc
}

// #[test]
// fn test_part1() {
//     assert_eq!(part1("89010123
// 78121874
// 87430965
// 96549874
// 45678903
// 32019012
// 01329801
// 10456732"), 36);
// }

// #[test]
// fn test_part2() {
//     assert_eq!(part2("89010123
// 78121874
// 87430965
// 96549874
// 45678903
// 32019012
// 01329801
// 10456732"), 81);
// }