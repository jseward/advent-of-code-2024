use aoc_runner_derive::aoc;
use std::collections::*;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Coord {
    pub x : i32,
    pub y : i32
}

fn parse_input(input: &str) -> Vec<Coord> {
    let mut start= Coord{x:0, y:0};
    let mut end = Coord{x:0, y:0};
    let mut tiles = HashSet::<Coord>::new();
    let mut y = 0;
    for line in input.lines() {
        let mut x = 0;
        for c in line.chars() {
            match c {
                'S' => start = Coord{x,y},
                'E' => end = Coord{x,y},
                '.' => { tiles.insert(Coord{x,y}); },
                '#' => {},
                _ => panic!()
            }
            x += 1;
        }
        y += 1;
    }

    let mut path = Vec::<Coord>::new();
    path.push(start);
    while !tiles.is_empty() {
        let path_back = path.get(path.len() - 1).unwrap();
        for delta in [(1,0), (-1,0), (0,1), (0,-1)] {
            let next = Coord{ x:path_back.x + delta.0, y:path_back.y + delta.1 };
            if tiles.remove(&next) {
                path.push(next);
                break;
            }
        }
    }
    path.push(end);
    path
}

#[aoc(day20, part1)]
pub fn part1(input: &str) -> i32 {
    let path = parse_input(input);
    let mut acc = 0;
    for i in 0..path.len() {
        let pi = path.get(i).unwrap();
        for delta in [(2,0),(-2,0),(0,2),(0,-2)] {
            let pd = Coord{x: pi.x + delta.0, y: pi.y + delta.1};
            if let Some(ji) = path.iter().position(|p| *p == pd) {
                if ji > i {
                    let saved = ji - i - 2;
                    if saved >= 100 {
                        acc += 1;
                    }
                }
            }
        }
    }
    acc
}

#[aoc(day20, part2)]
pub fn part2(input: &str) -> i32 {
    let path = parse_input(input);

    let mut deltas = vec!();
    let mut x = -20;
    while x <= 20 {
        let y_range = 20 - (x as i32).abs();
        let mut y = 0;
        while y <= y_range {
            deltas.push((x,y));
            if y != 0 {
                deltas.push((x,-y));
            }
            y += 1;
        }
        x += 1;
    }
    println!("{deltas:?}");

    let mut acc = 0;
    for i in 0..path.len() {
        let pi = path.get(i).unwrap();
        for delta in deltas.iter() {
            let pd = Coord{x: pi.x + delta.0, y: pi.y + delta.1};
            if let Some(ji) = path.iter().position(|p| *p == pd) {
                if ji > i {
                    let delta_dist = (delta.0.abs() + delta.1.abs()) as usize;
                    let saved_j = ji - i;
                    if saved_j > delta_dist {
                        let saved = saved_j - delta_dist;
                        if saved >= 100 {
                            acc += 1;
                        }
                    }
                }
            }
        }
        if i % 1000 == 0 {
            println!("{i}");
        }
    }
    acc
}

// #[test]
// fn test() {
//     let input = "###############
// #...#...#.....#
// #.#.#.#.#.###.#
// #S#...#.#.#...#
// #######.#.#.###
// #######.#.#...#
// #######.#.###.#
// ###..E#...#...#
// ###.#######.###
// #...###...#...#
// #.#####.#.###.#
// #.#...#.#.#...#
// #.#.#.#.#.#.###
// #...#...#...###
// ###############";
//     let path = parse_input(input);

//     let mut deltas = vec!();
//     let mut x = -20;
//     while x <= 20 {
//         let y_range = 20 - (x as i32).abs();
//         let mut y = 0;
//         while y <= y_range {
//             deltas.push((x,y));
//             if y != 0 {
//                 deltas.push((x,-y));
//             }
//             y += 1;
//         }
//         x += 1;
//     }

//     let mut cheats = HashMap::<i32, i32>::new();
//     for i in 0..path.len() {
//         let pi = path.get(i).unwrap();
//         for delta in deltas.iter() {
//             let pd = Coord{x: pi.x + delta.0, y: pi.y + delta.1};
//             if let Some(ji) = path.iter().position(|p| *p == pd) {
//                 if ji > i {
//                     let delta_dist = (delta.0.abs() + delta.1.abs()) as usize;
//                     let saved_j = ji - i;
//                     if saved_j > delta_dist {
//                         let saved = saved_j - delta_dist;
//                         if saved >= 50 {
//                             *cheats.entry(saved as i32).or_default() += 1;
//                         }
//                     }
//                 }
//             }
//         }
//     }
    
//     println!("{cheats:?}");
//     assert!(false);
// }