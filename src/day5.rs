use aoc_runner_derive::aoc;
use std::collections::HashMap;

#[aoc(day5, part1)]
pub fn part1(input: &str) -> i32 {
    let mut total = 0;
    let mut rule_map = HashMap::<i32, Vec<i32>>::with_capacity(1000);
    let mut is_rules = true;
    for line in input.lines() {
        if line == "" {
            assert!(is_rules);
            is_rules = false;
        }
        else {
            if is_rules {
                let rule : Vec<i32> = line.split("|").map(|p| p.parse().unwrap()).collect();
                unsafe {
                    assert!(rule.len() == 2);
                    let lv = rule.get_unchecked(0);
                    let rv = rule.get_unchecked(1);
                    rule_map.entry(*rv).or_insert(Vec::<i32>::new()).push(*lv);
                }
            }
            else {
                let order : Vec<i32> = line.split(",").map(|p| p.parse().unwrap()).collect();
                let mut is_order_ok = true;
                for i in 0..order.len() {
                    let page = order.get(i).unwrap();
                    let mut is_page_ok = true;
                    for j in (i + 1)..order.len() {
                        if let Some(rule) = rule_map.get(page) {
                            if rule.contains(order.get(j).unwrap()) {
                                is_page_ok = false;
                                break;
                            }
                        }
                    }
                    if !is_page_ok {
                        is_order_ok = false;
                        break;
                    }
                }
                if is_order_ok {
                    total += order.get(order.len() / 2).unwrap();
                }
            }
        }
    }
    total
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> i32 {
    let mut total = 0;
    let mut rule_map = HashMap::<i32, Vec<i32>>::with_capacity(1000);
    
    let get_bad_page_index = |rule_map : &HashMap<i32, Vec<i32>>, order : &Vec<i32>, from : usize| -> Option<usize> {
        if let Some(rule) = rule_map.get(order.get(from).unwrap()) {
            let mut bi = order.len() - 1;
            while bi > from {
                if rule.contains(order.get(bi).unwrap()) {
                    return Some(bi);
                }
                bi -= 1;
            }
        }
        None
    };

    let mut is_rules = true;
    for line in input.lines() {
        if line == "" {
            assert!(is_rules);
            is_rules = false;
        }
        else {
            if is_rules {
                let rule : Vec<i32> = line.split("|").map(|p| p.parse().unwrap()).collect();
                unsafe {
                    assert!(rule.len() == 2);
                    let lv = rule.get_unchecked(0);
                    let rv = rule.get_unchecked(1);
                    rule_map.entry(*rv).or_insert(Vec::<i32>::new()).push(*lv);
                }
            }
            else {
                let mut order : Vec<i32> = line.split(",").map(|p| p.parse().unwrap()).collect();
                let mut is_order_ok = true;
                let mut i : usize = 0;
                while i < order.len() {
                    match get_bad_page_index(&rule_map, &order, i) {
                        Some(bi) => {
                            is_order_ok = false;
                            order.swap(i, bi);
                        },
                        None => {
                            i += 1;
                        }
                    }
                }

                if !is_order_ok {
                    total += order.get(order.len() / 2).unwrap();
                }
            }
        }
    }
    total
}
