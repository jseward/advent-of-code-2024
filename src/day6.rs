use aoc_runner_derive::aoc;
use std::{collections::HashSet, fmt::Display};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Coord {
    pub x : i32,
    pub y : i32
}

impl Coord {
    pub fn new(x : i32, y : i32) -> Coord {
        Coord {
            x: x,
            y: y
        }
    }

    pub fn is_in_bounds(&self, max_x : i32, max_y : i32) -> bool {
        self.x >= 0 && self.x < max_x && self.y >= 0 && self.y < max_y
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> i32 {
    let mut pos = Coord::new(0, 0);
    let mut dir = Dir::Up;
    let mut walls = HashSet::<Coord>::with_capacity(1000);
    let mut y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for line in input.lines() {
        max_y += 1;
        if max_x == 0 {
            max_x = line.len() as i32;
        }

        let mut x = 0;
        for c in line.chars() {
            match c {
                '.' => {},
                '#' => { 
                    walls.insert(Coord::new(x, y));
                },
                '>' => { 
                    pos = Coord::new(x, y);
                    dir = Dir::Right;
                },
                '<' => { 
                    pos = Coord::new(x, y);
                    dir = Dir::Left;
                }
                '^' => { 
                    pos = Coord::new(x, y);
                    dir = Dir::Up;
                },
                'V' => { 
                    pos = Coord::new(x, y);
                    dir = Dir::Down;
                },
                _ => { assert!(false); }
            }
            x += 1;
        }
        y += 1;
    }

    let mut distinct = HashSet::<Coord>::with_capacity(1000);
    distinct.insert(pos.clone());
    while pos.is_in_bounds(max_x, max_y) {
        match dir {
            Dir::Up => {
                let next = Coord::new(pos.x, pos.y - 1);
                if walls.contains(&next) {
                    dir = Dir::Right;
                }
                else {
                    pos = next.clone();
                    distinct.insert(next);
                }
            },
            Dir::Right => {
                let next = Coord::new(pos.x + 1, pos.y);
                if walls.contains(&next) {
                    dir = Dir::Down;
                }
                else {
                    pos = next.clone();
                    distinct.insert(next);
                }
            },
            Dir::Down => {
                let next = Coord::new(pos.x, pos.y + 1);
                if walls.contains(&next) {
                    dir = Dir::Left;
                }
                else {
                    pos = next.clone();
                    distinct.insert(next);
                }
            },
            Dir::Left => {
                let next = Coord::new(pos.x - 1, pos.y);
                if walls.contains(&next) {
                    dir = Dir::Up;
                }
                else {
                    pos = next.clone();
                    distinct.insert(next);
                }
            }
        }
    }
    (distinct.len() - 1) as i32
}

enum Action {
    NewPos(Coord),
    NewDir(Dir)
}

fn is_infinite(walls : &Vec<Coord>, max_x : &i32, max_y : &i32, start_dir : &Dir, start_pos : &Coord) -> bool {
    let mut pos = start_pos.clone();
    let mut dir = start_dir.clone();

    let mut path = HashSet::<(Dir, Coord)>::new();

    while pos.is_in_bounds(*max_x, *max_y) {
        if path.contains(&(dir.clone(), pos.clone())) {
            return true;
        }
        path.insert((dir.clone(), pos.clone()));

        let action = match dir {
            Dir::Up => {
                let next = Coord::new(pos.x, pos.y - 1);
                if walls.contains(&next) {
                    Action::NewDir(Dir::Right)
                }
                else {
                    Action::NewPos(next)
                }
            },
            Dir::Right => {
                let next = Coord::new(pos.x + 1, pos.y);
                if walls.contains(&next) {
                    Action::NewDir(Dir::Down)
                }
                else {
                    Action::NewPos(next)
                }
            },
            Dir::Down => {
                let next = Coord::new(pos.x, pos.y + 1);
                if walls.contains(&next) {
                    Action::NewDir(Dir::Left)
                }
                else {
                    Action::NewPos(next)
                }
            },
            Dir::Left => {
                let next = Coord::new(pos.x - 1, pos.y);
                if walls.contains(&next) {
                    Action::NewDir(Dir::Up)
                }
                else {
                    Action::NewPos(next)
                }
            }
        };

        match action {
            Action::NewPos(p) => { 
                pos = p;
            },
            Action::NewDir(d) => {
                dir = d;
            }
        }
    }

    false        
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> i32 {
    let mut pos = Coord::new(0, 0);
    let mut dir = Dir::Up;

    let mut walls: Vec<Coord> = Vec::<Coord>::with_capacity(1000);
    let mut y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for line in input.lines() {
        max_y += 1;
        if max_x == 0 {
            max_x = line.len() as i32;
        }

        let mut x = 0;
        for c in line.chars() {
            match c {
                '.' => {},
                '#' => { 
                    walls.push(Coord::new(x, y));
                },
                '>' => { 
                    pos = Coord::new(x, y);
                    dir = Dir::Right;
                },
                '<' => { 
                    pos = Coord::new(x, y);
                    dir = Dir::Left;
                }
                '^' => { 
                    pos = Coord::new(x, y);
                    dir = Dir::Up;
                },
                'V' => { 
                    pos = Coord::new(x, y);
                    dir = Dir::Down;
                },
                _ => { assert!(false); }
            }
            x += 1;
        }
        y += 1;
    }

    let mut new_walls = HashSet::<Coord>::with_capacity(1000);
    for x in 0..max_x {
        for y in 0..max_y {
            let c = Coord::new(x, y);
            if !walls.contains(&c) {
                walls.push(c.clone());
                if is_infinite(&walls, &max_x, &max_y, &dir, &pos) {
                    //println!("is_infinite : {:?}", walls);
                    new_walls.insert(c);
                }
                walls.pop();
            }
        }
    }

    new_walls.len() as i32
}

pub fn part2_no_brute_force(input: &str) -> i32 {
    let mut pos = Coord::new(0, 0);
    let mut dir = Dir::Up;
    let mut walls = HashSet::<Coord>::with_capacity(1000);
    let mut y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for line in input.lines() {
        max_y += 1;
        if max_x == 0 {
            max_x = line.len() as i32;
        }

        let mut x = 0;
        for c in line.chars() {
            match c {
                '.' => {},
                '#' => { 
                    walls.insert(Coord::new(x, y));
                },
                '>' => { 
                    pos = Coord::new(x, y);
                    dir = Dir::Right;
                },
                '<' => { 
                    pos = Coord::new(x, y);
                    dir = Dir::Left;
                }
                '^' => { 
                    pos = Coord::new(x, y);
                    dir = Dir::Up;
                },
                'V' => { 
                    pos = Coord::new(x, y);
                    dir = Dir::Down;
                },
                _ => { assert!(false); }
            }
            x += 1;
        }
        y += 1;
    }

    let mut up_walls : Vec<Vec<i32>> = (0..max_x).map(|_i| {Vec::<i32>::new()} ).collect();
    let mut down_walls : Vec<Vec<i32>> = (0..max_x).map(|_i| {Vec::<i32>::new()} ).collect();
    let mut left_walls : Vec<Vec<i32>> = (0..max_y).map(|_i| {Vec::<i32>::new()} ).collect();
    let mut right_walls : Vec<Vec<i32>> = (0..max_y).map(|_i| {Vec::<i32>::new()} ).collect();

    let mut new_walls = HashSet::<Coord>::with_capacity(1000);
    let mut last_turn_pos = Coord::new(0, 0);
    let mut path = Vec::<(Dir, Coord)>::new();
    while pos.is_in_bounds(max_x, max_y) {
        let action = match dir {
            Dir::Up => {
                let next = Coord::new(pos.x, pos.y - 1);
                if walls.contains(&next) {
                    Action::NewDir(Dir::Right)
                }
                else {
                    Action::NewPos(next)
                }
            },
            Dir::Right => {
                let next = Coord::new(pos.x + 1, pos.y);
                if walls.contains(&next) {
                    Action::NewDir(Dir::Down)
                }
                else {
                    Action::NewPos(next)
                }
            },
            Dir::Down => {
                let next = Coord::new(pos.x, pos.y + 1);
                if walls.contains(&next) {
                    Action::NewDir(Dir::Left)
                }
                else {
                    Action::NewPos(next)
                }
            },
            Dir::Left => {
                let next = Coord::new(pos.x - 1, pos.y);
                if walls.contains(&next) {
                    Action::NewDir(Dir::Up)
                }
                else {
                    Action::NewPos(next)
                }
            }
        };

        let add_new_walls = |p : &Coord, nws : &mut HashSet<Coord>| {
            match dir {
                Dir::Up => {
                    for y in (p.y + 1)..(last_turn_pos.y + 1) {
                        for rw_x in right_walls.get(y as usize).unwrap() {
                            if *rw_x > p.x {
                                let nw = Coord::new(p.x, y - 1);
                                println!(". dir={:?} , last={:?} , curr={:?} , wall={:?}", dir, last_turn_pos, p, nw);
                                assert!(!walls.contains(&nw));
                                nws.insert(nw);
                            }
                        }
                    }
                },
                Dir::Right => {
                    for x in last_turn_pos.x..p.x {
                        let dw_ys = down_walls.get(x as usize).unwrap();
                        println!("down_walls @ {} = {:?}", x, dw_ys);
                        for dw_y in dw_ys {
                            if *dw_y > p.y {
                                let nw = Coord::new(x + 1, p.y);
                                println!(". dir={:?} , last={:?} , curr={:?} , wall={:?}", dir, last_turn_pos, p, nw);
                                assert!(!walls.contains(&nw));
                                nws.insert(nw);
                            }
                        }
                    }
                },
                Dir::Down => {
                    for y in last_turn_pos.y..p.y {
                        for lw_x in left_walls.get(y as usize).unwrap() {
                            if *lw_x < p.x {
                                let nw = Coord::new(p.x, y + 1);
                                println!(". dir={:?} , last={:?} , curr={:?} , wall={:?}", dir, last_turn_pos, p, nw);
                                assert!(!walls.contains(&nw));
                                nws.insert(nw);
                            }
                        }
                    }
                },
                Dir::Left => {
                    for x in (p.x + 1)..(last_turn_pos.x + 1) {
                        let uw_ys = up_walls.get(x as usize).unwrap();
                        println!("up_walls @ {} = {:?}", x, uw_ys);
                        for uw_y in uw_ys {
                            if *uw_y < p.y {
                                let nw = Coord::new(x - 1, p.y);
                                println!(". dir={:?} , last={} , curr={} , wall={:?}", dir, last_turn_pos, p, nw);
                                // println!("path = {} - {:?}{}", path.iter().map(|p| { format!("{:?}{}", p.0, p.1.to_string()) }).collect::<Vec<String>>().join(","), dir, p);
                                // if walls.contains(&nw) {
                                //     println!("!!!!!!!!!!!! WTF !!!!!!!!!!!!!!!!!");
                                // }
                                assert!(!walls.contains(&nw));
                                nws.insert(nw);
                            }
                        }
                    }
                }
            }
        };

        match action {
            Action::NewPos(p) => { 
                if !p.is_in_bounds(max_x, max_y) {
                    add_new_walls(&pos, &mut new_walls);
                }
                pos = p;
            },
            Action::NewDir(d) => {
                add_new_walls(&pos, &mut new_walls);
                match dir {
                    Dir::Up => {
                        up_walls.get_mut(pos.x as usize).unwrap().push(pos.y - 1);
                        println!("add up_wall @ x={} : y={}", pos.x, pos.y - 1);
                    },
                    Dir::Right => {
                        right_walls.get_mut(pos.y as usize).unwrap().push(pos.x + 1);
                        println!("add right_wall @ y={} : x={}", pos.y, pos.x + 1);
                    },
                    Dir::Down => {
                        down_walls.get_mut(pos.x as usize).unwrap().push(pos.y + 1);
                        println!("add down_wall @ x={} : y={}", pos.x, pos.y + 1);
                    },
                    Dir::Left => {
                        left_walls.get_mut(pos.y as usize).unwrap().push(pos.x - 1);
                        println!("add left_wall @ y={} : x={}", pos.y, pos.x - 1);
                    }
                }
                last_turn_pos = pos.clone();
                path.push((dir, pos.clone()));
                dir = d;
            }
        }
    }    
    new_walls.len() as i32
}

// #[test]
// fn test_part2() {
//     let a = part2("....#.....
// .........#
// ..........
// ..#.......
// .......#..
// ..........
// .#..^.....
// ........#.
// #.........
// ......#...");
//     assert_eq!(a, 6);
// }
