use aoc_runner_derive::aoc;

struct Equation {
    answer : u64,
    values : Vec<u64>
}

fn parse_line(line : &str) -> Equation {
    let parts : Vec<&str> = line.split(':').collect();
    let answer = parts.get(0).unwrap().parse().unwrap();
    let values = parts.get(1).unwrap().split_whitespace().into_iter().map(|s| s.parse().unwrap() ).collect();
    Equation {
        answer,
        values
    }
}

fn recurse(eq : &Equation, v : u64, i : usize) -> bool {
    if let Some(nv) = eq.values.get(i) {
        let mul_v = v * nv;
        if mul_v == eq.answer {
            return true;
        }
        

        if recurse(eq, mul_v, i + 1) {
            return true;
        }

        let add_v = v + nv;
        if add_v == eq.answer {
            return true;
        }
        
        if recurse(eq, add_v, i + 1) {
            return true;
        }
    }

    false
}

fn process_equation(eq : &Equation) -> bool {
    let mut iter = eq.values.iter();
    let v = *iter.next().unwrap();
    if v == eq.answer {
        return true;
    }

    if let Some(nv) = iter.next() {
        let mul_v = v * nv;
        if mul_v == eq.answer {
            return true;
        }

        if recurse(eq, mul_v, 2) {
            return true;
        }

        let add_v = v + nv;
        if add_v == eq.answer {
            return true;
        }
        
        if recurse(eq, add_v, 2) {
            return true;
        }
    }
    
    false
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u64 {
    let mut acc = 0;
    for line in input.lines() {
        let eq = parse_line(line);
        if process_equation(&eq) {
            acc += eq.answer;
        }
    }
    acc
}

struct Equation2 {
    answer : u64,
    values : Vec<(String, u64)>
}

fn parse_line2(line : &str) -> Equation2 {
    let parts : Vec<&str> = line.split(':').collect();
    let answer = parts.get(0).unwrap().parse().unwrap();
    let values : Vec<(String, u64)> = parts.get(1).unwrap().split_whitespace().into_iter().map(|s| (s.to_string(), s.parse::<u64>().unwrap())).collect();
    Equation2 {
        answer,
        values
    }
}

fn process_equation2(ans : u64, curr_value : &(&str, u64), next_values : &[(&str, u64)]) -> bool {

    //println!("{:?} : [{}]", curr_value_s, next_values.join(" "));

    if curr_value.1 <= ans {
        if let Some(next) = next_values.get(0) {
            let mul_v = curr_value.1 * next.1;
            let add_v = curr_value.1 + next.1;
            let mut concat_s = String::with_capacity(curr_value.0.len() + next.0.len());
            concat_s.push_str(curr_value.0);
            concat_s.push_str(next.0);
            
            let recurse_vals = &next_values[1..];
            if process_equation2(ans, &(&mul_v.to_string(), mul_v), &recurse_vals) {
                return true;
            }

            if process_equation2(ans, &(&add_v.to_string(), add_v), &recurse_vals) {
                return true;
            }

            return process_equation2(ans, &(&concat_s, concat_s.parse::<u64>().unwrap()), &recurse_vals);
        }
        else {
            return curr_value.1 == ans;
        }
    }

    false
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u64 {
    let mut acc = 0;
    for line in input.lines() {
        let eq = parse_line2(line);
        let mut iter = eq.values.iter();
        let s = iter.next().unwrap();
        if process_equation2(eq.answer, &(s.0.as_str(), s.1), &Vec::from_iter(iter.map(|s| (s.0.as_str(), s.1)))) {
            acc += eq.answer;
        }
    }
    acc
}

// #[test]
// fn test_part2() {
//     assert_eq!(part2("190: 10 19
// 3267: 81 40 27
// 83: 17 5
// 156: 15 6
// 7290: 6 8 6 15
// 161011: 16 10 13
// 192: 17 8 14
// 21037: 9 7 18 13
// 292: 11 6 16 20"), 11387)
// }

// #[test]
// fn test2() {
//     assert_eq!(part2("1638669965109: 7 9 6 9 762 6 23 3 4 3 5 9"), 0);
// }
