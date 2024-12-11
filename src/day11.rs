use aoc_runner_derive::aoc;
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Stone {
    s : String,
    v : u64
}

impl Stone {
    pub fn new(mut s : String) -> Stone {
        while s.len() > 1 && s.starts_with('0') {
            let new_s = s.as_mut().split_at(1);
            s = new_s.1.to_string();
        }
        let v = s.parse().unwrap();
        Stone { s, v }
    }
}

fn parse_input(input: &str) -> Vec<Stone> {
    input.split_whitespace().into_iter().map(|i| Stone::new(i.to_string())).collect()
}

// fn blink(stones : &mut Vec<Stone>) {
//     let mut i : usize = 0;
//     while i < stones.len() {
//         let stone = stones.get_mut(i).unwrap();
//         if stone.v == 0 {
//             stone.s = "1".to_string();
//             stone.v = 1;
//             i += 1; 
//         }
//         else if stone.s.len() % 2 == 0 {
//             let new_stone = Stone::new(stone.s.split_off(stone.s.len() / 2));
//             stone.v = stone.s.parse().unwrap();
//             stones.insert(i + 1, new_stone);
//             i += 2;
//         }
//         else {
//             stone.v *= 2024;
//             stone.s = stone.v.to_string();
//             i += 1;
//         }
//     }
// }

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct StoneKey {
    stone : Stone,
    blink : u32
}

fn get_stone_count(table : &mut HashMap::<StoneKey, u64>, stone : Stone, blinks_remaining: u32, curr_count : u64) -> u64 {
    //println!("get_stone_count stone={} , blinks_remaining={} , curr_count={}", stone.v, blinks_remaining, curr_count);
    let key = StoneKey { stone: stone.clone(), blink: blinks_remaining };
    match table.get(&key) {
        Some(count) => *count,
        None => {
            let mut next_count = curr_count;            
            if blinks_remaining > 0 {
                let next_blinks_remaining = blinks_remaining - 1;
                if stone.v == 0 {
                    let next_stone = Stone {
                        s : "1".to_string(),
                        v : 1
                    };
                    next_count = get_stone_count(table, next_stone, next_blinks_remaining, curr_count);
                }
                else if stone.s.len() % 2 == 0 {
                    let split = stone.s.split_at(stone.s.len() / 2);
                    next_count = 
                        (curr_count - 1) +
                        get_stone_count(table, Stone::new(split.0.to_string()), next_blinks_remaining, 1) + 
                        get_stone_count(table, Stone::new(split.1.to_string()), next_blinks_remaining, 1);
                }
                else {
                    let next_v = stone.v * 2024;
                    let next_stone = Stone {
                        s : next_v.to_string(),
                        v : next_v
                    };
                    next_count = get_stone_count(table, next_stone, next_blinks_remaining, curr_count);
                }
            }    
            //println!("{:?}|{:?} = {}", key.stone.v, key.blink, next_count);
            table.insert(key, next_count);
            next_count        
        }
    }
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> u64 {
    // let mut stones = parse_input(input);
    // let mut last_len = 0;
    // for _i in 0..25 {
    //     blink(&mut stones);
    //     println!("{} : +{}", stones.len(), stones.len() - last_len);
    //     last_len = stones.len();
    // }
    // stones.len() as i32

    let stones = parse_input(input);
    let mut table = HashMap::<StoneKey, u64>::new();
    let mut acc : u64 = 0;
    for s in stones {
        acc += get_stone_count(&mut table, s, 25, 1);
    }
    acc
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> u64 {
    let stones = parse_input(input);
    let mut table = HashMap::<StoneKey, u64>::new();
    let mut acc : u64 = 0;
    for s in stones {
        acc += get_stone_count(&mut table, s, 75, 1);
    }
    acc
}

// #[test]
// fn test_part1() {
//     let mut stones = parse_input("125 17");
//     let mut last_len = 0;
//     for _i in 0..25 {
//         blink(&mut stones);
//         println!("{} : +{}", stones.len(), stones.len() - last_len);
//         //println!("{}", stones.iter().map(|s| s.s.as_str()).collect::<Vec<&str>>().join(" "));
//         last_len = stones.len();
//     }
//     assert_eq!(stones.len(), 55312);
// }

// #[test]
// fn test_part2() {
//     let stones = parse_input("125 17");
//     let mut table = HashMap::<StoneKey, u64>::new();
//     let mut acc : u64 = 0;
//     for s in stones {
//         acc += get_stone_count(&mut table, s, 25, 1);
//     }
//     assert_eq!(acc, 55312);
// }