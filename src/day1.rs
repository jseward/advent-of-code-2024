use aoc_runner_derive::aoc;
use std::collections::HashMap;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let mut l_values = Vec::<i32>::with_capacity(1000);
    let mut r_values = Vec::<i32>::with_capacity(1000);
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        l_values.push(parts.next().unwrap().parse().unwrap());
        r_values.push(parts.next().unwrap().parse().unwrap());
    }
    l_values.sort_unstable();
    r_values.sort_unstable();
    let mut total_diff = 0;
    for i in 0..l_values.len() {
        total_diff += (l_values[i] - r_values[i]).abs();
    }
    total_diff
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let mut l_values = Vec::<i32>::with_capacity(1000);
    let mut r_values = HashMap::<i32, i32>::new();
    for line in input.lines() {
        let parts : Vec<&str> = line.split_whitespace().collect();
        l_values.push(parts[0].parse().unwrap());
        let r_v : i32 = parts[1].parse().unwrap();
        *r_values.entry(r_v).or_insert(0) += 1;
    }
    let mut score = 0;
    for v in l_values {
        match r_values.get(&v) {
            None => {},
            Some(count) => {
                score += v * count;
            }
        }
    }
    score
}
