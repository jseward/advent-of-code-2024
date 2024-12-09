use aoc_runner_derive::aoc;
use std::collections::{VecDeque};

#[derive(Debug, PartialEq, Eq)]
enum Block {
    File(u32, u8),
    Free(u8)
}

fn parse_input(input: &str) -> VecDeque<Block> {
    let mut input_blocks = VecDeque::<Block>::with_capacity(20000);
    let mut next_id : u32 = 0;
    let mut next_free = false;
    for c in input.chars() {
        let len : u8 = ((c as i32) - ('0' as i32)) as u8;
        match next_free {
            true => input_blocks.push_back(Block::Free(len)),
            false => {
                input_blocks.push_back(Block::File(next_id, len));
                next_id += 1;
            }
        }
        next_free = !next_free;
    }
    input_blocks
}

fn get_checksum(blocks : &VecDeque<Block>) -> u64 {
    let mut checksum : u64 = 0;
    let mut pos : u32 = 0;
    for b in blocks {
        match b {
            Block::File(id, len) => {
                for _i in 0..*len {
                    checksum += (id * pos) as u64;
                    pos += 1;
                }
            },
            Block::Free(len) => {
                pos += *len as u32;
            }
        }
    }    
    checksum
}

#[aoc(day9, part1)]
pub fn part1(input: &str) -> u64 {
    let mut input_blocks = parse_input(input);
    let mut final_blocks = VecDeque::<Block>::with_capacity(20000);
    while !input_blocks.is_empty() {
        match input_blocks.pop_front() {
            Some(fb) => {
                match fb {
                    Block::File(id, len) => {
                        final_blocks.push_back(Block::File(id, len));
                    },
                    Block::Free(free_len) => {
                        let mut free_len_remaining = free_len;
                        while !input_blocks.is_empty() && free_len_remaining > 0 {
                            match input_blocks.pop_back() {
                                Some(bb) => {
                                    match bb {
                                        Block::File(id, len) => {
                                            if len >= free_len_remaining {
                                                final_blocks.push_back(Block::File(id, free_len_remaining));
                                                let file_len_rem = len - free_len_remaining;
                                                if file_len_rem > 0 {
                                                    input_blocks.push_back(Block::File(id, file_len_rem));
                                                }
                                                free_len_remaining = 0;
                                            }
                                            else {
                                                final_blocks.push_back(Block::File(id, len));
                                                free_len_remaining -= len;
                                            }
                                        },
                                        Block::Free(_) => {}
                                    }
                                },
                                None => {}
                            }
                        }
                        if free_len_remaining > 0 {
                            final_blocks.push_back(Block::Free(free_len_remaining));
                        }
                    }
                }
            }
            None => {}
        }
    }
    get_checksum(&final_blocks)
}

// fn print_blocks(blocks : &VecDeque<Block>) {
//     let mut s = String::new();
//     for b in blocks {
//         match b {
//             Block::File(id, len) => {
//                 for _i in 0..(*len) {
//                     s.push_str(id.to_string().as_str());
//                 }        
//             },
//             Block::Free(len) => {
//                 for _i in 0..(*len) {
//                     s.push('.');
//                 }
//             }
//         }
//     }
//     println!("{}", s);
// }

#[aoc(day9, part2)]
pub fn part2(input: &str) -> u64 {
    let mut input_blocks = parse_input(input);
    let mut move_blocks = VecDeque::<Block>::with_capacity(20000);
    for b in input_blocks.iter() {
        match b {
            Block::File(id, len) => {
                move_blocks.push_front(Block::File(*id, *len));
            },
            _ => {}
        }
    }

    for b in move_blocks {
        //print_blocks(&input_blocks);
        match b {
            Block::File(id, len) => {
                let free_pos = input_blocks.iter().position(|b| {
                    match b {
                        Block::Free(free_len) => {
                            len <= *free_len
                        },
                        _ => { false }
                    }
                });
                match free_pos {
                    Some(free_pos) => {
                        let remove_pos = input_blocks.iter().position(|b| {
                            match b {
                                Block::File(rid, _) => {
                                    id == *rid
                                },
                                _ => { false }
                            }
                        }).unwrap();
                        if remove_pos > free_pos {
                            let remb = input_blocks.get_mut(remove_pos).unwrap();
                            match remb {
                                Block::File(_, len) => {
                                    *remb = Block::Free(*len);
                                },
                                _ => { panic!(); }
                            }
                            
                            let free_b = input_blocks.get_mut(free_pos).unwrap();
                            match *free_b {
                                Block::Free(free_len) => {                                
                                    *free_b = b;
                                    if len < free_len {
                                        input_blocks.insert(free_pos + 1, Block::Free(free_len - len));
                                    }
                                },
                                _ => { assert!(false); }
                            }
                        }
                    },
                    None => {}
                }
            }
            Block::Free(_) => { assert!(false); }
        }
    }

    get_checksum(&input_blocks)
}

// #[test]
// fn test_part1() {
//     assert_eq!(part1("2333133121414131402"), 1928);
// }

// #[test]
// fn test_part2() {
//     assert_eq!(part2("2333133121414131402"), 2858);
// }