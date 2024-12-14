use aoc_runner_derive::aoc;
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Debug)]
struct Vec2 {
    pub x : i32,
    pub y : i32
}

// impl Vec2 {
//     pub fn length(&self) -> f32 {
//         ((self.x as f32).powf(2.0) + (self.y as f32).powf(2.0)).sqrt()
//     }
// }

#[derive(Hash, PartialEq, Eq, Debug)]
struct Robot {
    pub pos : Vec2,
    pub vel : Vec2
}

fn parse_vec2(input: &str) -> Vec2 {
    let (_, vec2_str) = input.split_once('=').unwrap();
    let (x_str, y_str) = vec2_str.split_once(',').unwrap();
    Vec2 {
        x : x_str.parse().unwrap(),
        y : y_str.parse().unwrap()
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    let mut robots = vec!();
    for line in input.lines() {
        let p_v_parts = line.split_once(' ').unwrap();
        robots.push(Robot {
            pos : parse_vec2(p_v_parts.0),
            vel : parse_vec2(p_v_parts.1)
        });
    }
    robots
}

fn get_p(start_p : i32, vel : i32, seconds : i32, max_p : i32) -> i32 {
    let mut p = (start_p + (vel * seconds)) % max_p;
    if p < 0 {
        p = max_p + p;
    }
    p
}

fn get_tiles(robots : &Vec<Robot>, width : i32, height : i32, seconds : i32) -> HashMap<Vec2, i32> {
    let mut tiles = HashMap::<Vec2, i32>::new();
    for r in robots {
        let p = Vec2 {
            x: get_p(r.pos.x, r.vel.x, seconds, width),
            y: get_p(r.pos.y, r.vel.y, seconds, height),
        };
        *tiles.entry(p).or_insert(0) += 1;
    }
    tiles
}

fn get_quad_factor(tiles : &HashMap<Vec2, i32>, quad : (i32, i32, i32, i32)) -> i32 {
    let mut factor = 0;
    for x in (quad.0)..(quad.1) {
        for y in (quad.2)..(quad.3) {
            factor += tiles.get(&Vec2{x,y}).unwrap_or(&0);
        }
    }
    factor
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> i32 {
    let robots = parse_input(input);
    let tiles = get_tiles(&robots, 101, 103, 100);
    let mut factor = 1;
    for quad in [(0,50,0,51), (51,101,0,51), (0,50,52,103), (51,101,52,103)] {
        factor *= get_quad_factor(&tiles, quad);
    }
    factor
}
//475571
//529920
//217328832

fn print_tiles(tiles : HashMap<Vec2, i32>, width : i32, height : i32) {
    for y in 0..height {
        let mut line = String::new();
        for x in 0..width {
            line += match tiles.get(&Vec2{x,y}) {
                Some(_) => {
                    match y == height / 2 {
                        true => "!",
                        false => "*"
                    }
                },
                None => "."
            }
        }
        println!("{line}");
    }
}

// fn detect_tree(tiles : &HashMap<Vec2, i32>, width : i32, height : i32) -> i32 {
//     let mut score = 0.0;
//     for x in 0..width {
//         for y in 0..height {
//             if tiles.contains_key(&Vec2{x,y}) {
//                 let center_offset = Vec2 {
//                     x: ((width / 2) - x).abs(),
//                     y: ((height / 2) - y).abs(),
//                 };
//                 score += center_offset.length();
//             }
//         }
//     }
//     score as i32
// }

fn detect_tree2(tiles : &HashMap<Vec2, i32>, width : i32, height : i32) -> bool {
    for y in 0..height {
        for x in 0..width {
            if tiles.contains_key(&Vec2{x,y}) {
                let mut is_tree = true;
                for off in 0..10 {
                    if !tiles.contains_key(&Vec2{x: x + off, y}) {
                        is_tree = false;
                        break;
                    }
                }
                if is_tree {
                    return true;
                }
            }
        }
    }
    false
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> i32 {
    let robots = parse_input(input);
    let mut seconds = 0;
    // let mut best_tree_score : Option<i32> = None;
    // let mut best_seconds = 0;
    loop {
        let tiles = get_tiles(&robots, 101, 103, seconds);
        // let tree_score = detect_tree(&tiles, 101, 103);
        // if best_tree_score.is_none() || tree_score < best_tree_score.unwrap() {
        //     println!("");
        //     println!("SECONDS={seconds}");
        //     print_tiles(tiles, 101, 103);
        //     println!("");
        //     best_tree_score = Some(tree_score);
        //     best_seconds = seconds;
        // }
        // if seconds - best_seconds > 10000 {
        //     break;
        // }

        if detect_tree2(&tiles, 101, 103) {
            // println!("");
            // println!("SECONDS={seconds}");
            // print_tiles(tiles, 101, 103);
            // println!("");
            break;
        }

        seconds += 1;        
    }
    seconds
}

// #[test]
// fn test_part1() {
//     let robots = parse_input("p=0,4 v=3,-3
// p=6,3 v=-1,-3
// p=10,3 v=-1,2
// p=2,0 v=2,-1
// p=0,0 v=1,3
// p=3,0 v=-2,-2
// p=7,6 v=-1,-3
// p=3,0 v=-1,-2
// p=9,3 v=2,3
// p=7,3 v=-1,2
// p=2,4 v=2,-3
// p=9,5 v=-3,-3");
//     let tiles = get_tiles(robots, 11, 7, 100);
//     let mut factor = 1;
//     for quad in [(0,5,0,3), (6,11,0,3), (0,5,4,7), (6,11,4,7)] {
//         let quad_f = get_quad_factor(&tiles, quad);
//         println!("{quad_f}");
//         factor *= quad_f;
//     }
//     println!("{:?}", tiles);
//     assert_eq!(factor, 12);
// }