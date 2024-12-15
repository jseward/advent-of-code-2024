use aoc_runner_derive::aoc;
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Debug, Clone, PartialOrd, Ord)]
struct Vec2 {
    pub x : i32,
    pub y : i32
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Object {
    Wall,
    Box,
    WideBoxLeft,
    WideBoxRight,
    Robot
}

#[derive(Debug, Clone)]
enum Move {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
struct Map {
    pub objects : HashMap<Vec2, Object>,
    pub robot : Vec2,
    pub moves : Vec<Move>
}

impl Map {
    fn from_str(input: &str, is_wide : bool) -> Map {
        let mut objects = HashMap::<Vec2, Object>::new();
        let mut robot : Vec2 = Vec2{x:0,y:0};

        let mut lines = input.lines().into_iter();
        let wall = lines.next().unwrap();
        {
            let mut x = 0;
            for _c in wall.chars() {
                if is_wide {
                    objects.insert(Vec2{x, y:0}, Object::Wall);
                    objects.insert(Vec2{x: x+1, y:0}, Object::Wall);
                    x += 2;
                }
                else {
                    objects.insert(Vec2{x, y:0}, Object::Wall);
                    x += 1;
                }
            }
        }

        let mut y = 1;
        loop {
            let map_line = lines.next().unwrap();
            if map_line == wall {
                break;
            }
            let map_line = map_line.trim();
            let mut x= 0;
            for c in map_line.chars() {
                match c {
                    '.' => {},
                    'O' => {
                        if is_wide {
                            objects.insert(Vec2{x,y}, Object::WideBoxLeft); 
                            objects.insert(Vec2{x: x + 1,y}, Object::WideBoxRight);                    
                        }
                        else {
                            objects.insert(Vec2{x,y}, Object::Box); 
                        }
                    },
                    '#' => {
                        objects.insert(Vec2{x,y}, Object::Wall); 
                        if is_wide {
                            objects.insert(Vec2{x : x + 1,y}, Object::Wall); 
                        }
                    },
                    '@' => {objects.insert(Vec2{x,y}, Object::Robot); robot = Vec2{x,y};}
                    _ => { assert!(false); }
                }
                x += 1;
                if is_wide {
                    x += 1;
                }
            }
            y += 1;
        }

        {
            let mut x = 0;
            for _c in wall.chars() {
                if is_wide {
                    objects.insert(Vec2{x, y}, Object::Wall);
                    objects.insert(Vec2{x: x+1, y}, Object::Wall);
                    x += 2;
                }
                else {
                    objects.insert(Vec2{x, y}, Object::Wall);
                    x += 1;
                }
            }
        }
        
        let blank = lines.next().unwrap().trim();
        assert!(blank.len() == 0);

        let mut moves = vec!();
        loop {
            if let Some(line) = lines.next() {
                for c in line.chars() {
                    moves.push(
                        match c {
                            '<' => { Move::Left },
                            'v' => { Move::Down },
                            '>' => { Move::Right },
                            '^' => { Move::Up },
                            _ => { panic!("bad char '{}'", c) }
                        });
                }
            }
            else {
                break;
            }
        }

        Map {
            objects,
            robot,
            moves
        }
    }

    pub fn update(&mut self, m : Move) {
        let off = match m {
            Move::Left => (-1, 0),
            Move::Up => (0, -1),
            Move::Right => (1, 0),
            Move::Down => (0, 1)
        };
        let robot_pos = self.robot.clone();
        //println!("MOVING ROBOT @ {:?}", robot_pos);
        let mut moves = vec!();
        if self.try_move(&robot_pos, off, &mut moves) {
            moves.sort();
            moves.dedup();
            //println!("moves = {:?}", moves);
            for m in moves.iter() {
                let rem = self.objects.remove(&m.1).unwrap();
                assert_eq!(rem, m.0);
            }
            for m in moves.iter() {
                let ins = self.objects.insert(m.2.clone(), m.0.clone());
                assert!(ins.is_none());
            }
            if !moves.is_empty() {
                let rob_move = moves.get(moves.len() - 1).unwrap();
                assert!(rob_move.0 == Object::Robot);
                assert!(rob_move.1 == self.robot);
                self.robot = rob_move.2.clone();
            }
        }
    }

    fn try_move(&mut self, at : &Vec2, off : (i32, i32), moves : &mut Vec<(Object, Vec2, Vec2)>) -> bool {
        match self.objects.get(at) {
            Some(curr_obj) => {
                match curr_obj {
                    Object::Wall => {
                        false
                    },
                    Object::Robot => {
                        let next_pos: Vec2 = Vec2{ x: at.x + off.0, y : at.y + off.1 };
                        let move_ok = self.try_move(&next_pos, off, moves);
                        if move_ok {
                            moves.push((Object::Robot, at.clone(), next_pos));
                        }
                        move_ok
                    },
                    Object::Box => {
                        let next_pos: Vec2 = Vec2{ x: at.x + off.0, y : at.y + off.1 };
                        let move_ok = self.try_move(&next_pos, off, moves);
                        if move_ok {
                            moves.push((Object::Box, at.clone(), next_pos));
                        }
                        move_ok
                    },
                    Object::WideBoxLeft => {
                        let right_box_pos = Vec2{ x: at.x + 1, y : at.y };
                        let next_left_pos: Vec2 = Vec2{ x: at.x + off.0, y : at.y + off.1 };
                        let next_right_pos: Vec2 = Vec2{ x: right_box_pos.x + off.0, y : right_box_pos.y + off.1 };
                        let is_left_move_ok : bool;
                        let is_right_move_ok: bool;
                        if off.0 == -1 {
                            is_left_move_ok = self.try_move(&next_left_pos, off, moves);
                            is_right_move_ok = is_left_move_ok;
                        }
                        else if off.0 == 1 {
                            is_right_move_ok = self.try_move(&next_right_pos, off, moves);
                            is_left_move_ok = is_right_move_ok;
                        }
                        else {
                            is_left_move_ok = self.try_move(&next_left_pos, off, moves);
                            is_right_move_ok = self.try_move(&next_right_pos, off, moves);
                        }
                        let are_both_moves_ok = is_left_move_ok && is_right_move_ok;
                        if are_both_moves_ok {
                            moves.push((Object::WideBoxLeft, at.clone(), next_left_pos));
                            moves.push((Object::WideBoxRight, right_box_pos, next_right_pos));
                        }
                        are_both_moves_ok
                    },
                    Object::WideBoxRight => {
                        let left_box_pos = Vec2{ x: at.x - 1, y : at.y };
                        let next_right_pos: Vec2 = Vec2{ x: at.x + off.0, y : at.y + off.1 };
                        let next_left_pos: Vec2 = Vec2{ x: left_box_pos.x + off.0, y : left_box_pos.y + off.1 };
                        let is_left_move_ok : bool;
                        let is_right_move_ok: bool;
                        if off.0 == -1 {
                            is_left_move_ok = self.try_move(&next_left_pos, off, moves);
                            is_right_move_ok = is_left_move_ok;
                        }
                        else if off.0 == 1 {
                            is_right_move_ok = self.try_move(&next_right_pos, off, moves);
                            is_left_move_ok = is_right_move_ok;
                        }
                        else {
                            is_left_move_ok = self.try_move(&next_left_pos, off, moves);
                            is_right_move_ok = self.try_move(&next_right_pos, off, moves);
                        }
                        let are_both_moves_ok = is_left_move_ok && is_right_move_ok;
                        if are_both_moves_ok {
                            moves.push((Object::WideBoxLeft, left_box_pos, next_left_pos));
                            moves.push((Object::WideBoxRight, at.clone(), next_right_pos));
                        }
                        are_both_moves_ok
                    }
                }
            },
            None => {
                true
            }
        }
    }

    pub fn gps_sum(&self) -> i32 {
        let mut sum = 0;
        for (k, v) in self.objects.iter() {
            match v {
                Object::Box => {
                    sum += (100 * k.y) + k.x;
                },
                Object::WideBoxLeft => {
                    sum += (100 * k.y) + k.x;
                },
                _ => {}
            }
        }
        sum
    }
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> i32 {
    let mut map = Map::from_str(input, false);
    let moves = map.moves.clone();
    for m in moves {
        map.update(m);
        //print_map(&map, 50, 50);
    }
    map.gps_sum()
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> i32 {
    let mut map = Map::from_str(input, true);
    let moves = map.moves.clone();
    for m in moves {
        map.update(m);
        //print_map(&map, 50, 50);
    }
    map.gps_sum()
}

// fn print_map(map : &Map, width : i32, height : i32) {
//     for y in 0..height {
//         let mut s = String::new();
//         for x in 0..width {
//             match map.objects.get(&Vec2{x,y}) {
//                 Some(obj) => {
//                     s += match obj {
//                         Object::Box => "O",
//                         Object::Robot => "@",
//                         Object::Wall => "#",
//                         Object::WideBoxLeft => "[",
//                         Object::WideBoxRight => "]"
//                     }
//                 },
//                 None => {
//                     s += ".";
//                 }
//             }
//         }
//         println!("{}", s);
//     }
// }

// #[test]
// fn test() {
//     let input = "########
// #..O.O.#
// ##@.O..#
// #...O..#
// #.#.O..#
// #...O..#
// #......#
// ########

// <^^>>>vv<v>>v<<";
//     let mut map = Map::from_str(input, false);
//     let moves = map.moves.clone();
//     for m in moves {
//         println!("MOVE = {:?}", m);
//         map.update(m);
//         print_map(&map, 8, 8);
//         println!("");
//     }
//     assert_eq!(map.gps_sum(), 2028);
// }

// #[test]
// fn test() {
//     let input = "##########
// #..O..O.O#
// #......O.#
// #.OO..O.O#
// #..O@..O.#
// #O#..O...#
// #O..O..O.#
// #.OO.O.OO#
// #....O...#
// ##########

// <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
// vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
// ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
// <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
// ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
// ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
// >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
// <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
// ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
// v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
//     let mut map = Map::from_str(input, true);
//     let moves = map.moves.clone();
//     for m in moves {
//         println!("MOVE = {:?}", m);
//         map.update(m);
//         print_map(&map, 14, 7);
//         println!("");
//     }
//     assert_eq!(map.gps_sum(), 9021);
// }
