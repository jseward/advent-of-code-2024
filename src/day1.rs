use aoc_runner_derive::aoc;
use std::collections::HashMap;

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut l_values = Vec::<i32>::with_capacity(1000);
    let mut r_values = Vec::<i32>::with_capacity(1000);
    for line in input.lines() {
        let parts : Vec<&str> = line.split_whitespace().collect();
        l_values.push(parts[0].parse().unwrap());
        r_values.push(parts[1].parse().unwrap());
    }
    l_values.sort();
    r_values.sort();
    let mut total_diff = 0;
    for i in 0..l_values.len() {
        total_diff += (l_values[i] - r_values[i]).abs();
    }
    total_diff
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let mut l_values = Vec::<i32>::with_capacity(1000);
    let mut r_values = HashMap::<i32, i32>::new();
    for line in input.lines() {
        let parts : Vec<&str> = line.split_whitespace().collect();
        l_values.push(parts[0].parse().unwrap());
        let r_v : i32 = parts[1].parse().unwrap();
        match r_values.get_mut(&r_v) {
            Some(v) => { *v += 1; }
            None => { r_values.insert(r_v, 1); }
        }
    }
    let mut score = 0;
    l_values.iter().for_each(|v| {
        match r_values.get(v) {
            None => {},
            Some(count) => {
                score += v * count;
            }
        }
    });
    score
}
