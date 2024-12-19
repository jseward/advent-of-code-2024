use aoc_runner_derive::aoc;
use std::collections::*;

fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
    let mut line_iter = input.lines().into_iter();
    let towels = line_iter.next().unwrap().split(",").map(|t| t.trim().to_string()).collect();
    line_iter.next();
    let designs = line_iter.map(|line| line.to_string()).collect();
    (towels, designs)
}

fn can_arrange(design : &str, towels : &Vec<String>) -> bool {
    for towel in towels {
        if design.starts_with(towel) {
            if design.len() == towel.len() {
                return true;
            }
            if can_arrange(&design[towel.len()..], towels) {
                return true;
            }
        }
    }
    false
}

fn get_arrangment_count<'a>(design : &'a str, towel_map : &HashMap<usize, HashSet<&str>>, towel_lens : &Vec<usize>, design_memory : &mut HashMap<&'a str, usize>) -> usize {
    if let Some(memory) = design_memory.get(design) {
        return *memory;
    }

    let mut count = 0;
    for towel_len in towel_lens.iter().rev() {
        let towels = towel_map.get(towel_len).unwrap();
        if *towel_len <= design.len() {
            if towels.contains(&design[..*towel_len]) {
                if design.len() == *towel_len {
                    count += 1;
                }
                else {
                    count += get_arrangment_count(&design[*towel_len..], towel_map, towel_lens, design_memory);
                }
            }
        }        
    }

    design_memory.insert(design, count);
    count
}

#[aoc(day19, part1)]
pub fn part1(input: &str) -> i32 {
    let (towels, designs) = parse_input(input);
    designs.iter().map(|design| if can_arrange(design, &towels) { 1 } else { 0 } ).sum()
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> usize {
    let (towels, designs) = parse_input(input);
    let mut towel_map = HashMap::<usize, HashSet<&str>>::new();
    for towel in towels.iter() {
        towel_map.entry(towel.len()).or_insert(HashSet::<&str>::new()).insert(&towel);
    }
    let mut towel_lens : Vec<usize> = towel_map.keys().map(|k| *k).collect();
    towel_lens.sort();
    let mut design_memory = HashMap::<&str, usize>::new();
    designs.iter().map(|design| get_arrangment_count(design, &towel_map, &towel_lens, &mut design_memory)).sum()
}
