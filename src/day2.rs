use aoc_runner_derive::aoc;

fn resolve_delta(v0 : &i32, v1 : &i32) -> Option<bool> {
    let diff = v1 - v0;
    let diff_abs = diff.abs();
    if diff_abs < 1 || diff_abs > 3 {
        return None;
    }
    Some(diff > 0)
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    let mut num_safe = 0;
    for line in input.lines() {
        let mut parts = line.split_whitespace();

        let v0 : i32 = parts.next().unwrap().parse().unwrap();
        let v1 : i32 = parts.next().unwrap().parse().unwrap();

        let mut is_unsafe = false;
        match resolve_delta(&v0, &v1) {
            None => { is_unsafe = true; },
            Some(is_increasing_at_start) => {
                let mut prev = v1;
                while let Some(curr_str) = parts.next() {
                    let curr : i32 = curr_str.parse().unwrap();
                    match resolve_delta(&prev, &curr) {
                        None => {
                            is_unsafe = true;
                            break; 
                        },
                        Some(is_increasing_now) => {
                            if is_increasing_now != is_increasing_at_start {
                                is_unsafe = true;
                                break;     
                            }
                        }
                    }
                    prev = curr;
                }
            }
        }

        if !is_unsafe {
            num_safe += 1;
        }
    }
    return num_safe;
}

fn is_report_safe_part2(line: &str) -> bool {
    let values :Vec<i32> = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
    let mut is_safe =
        is_report_safe_part2_parsed(&values, false) ||
        is_report_safe_part2_parsed(&values[1..], true);
    if !is_safe {
        // handle the edge case of 10 13 9 8 7 - remove 13 to make safe
        let mut clone = values.clone();
        clone.remove(1);
        is_safe = is_report_safe_part2_parsed(&clone, true);
    }

    //brute force to figure out what algo above is missing ...
    // for i in 0..values.len() {
    //     let mut clone = values.clone();
    //     clone.remove(i);
    //     let is_clone_safe = is_report_safe_part2_parsed(&clone, true);
    //     if is_clone_safe && !is_safe {
    //         println!("{}", line);
    //     }
    // }

    is_safe
}

fn is_report_safe_part2_parsed(values : &[i32], removed_bad_in : bool) -> bool {

    let mut is_unsafe = false;
    let mut is_increasing_at_start : Option<bool> = None;

    let mut iter = values.iter();
    let mut prev = iter.next().unwrap();

    let mut removed_bad = removed_bad_in;
    while let Some(curr) = iter.next() {
        match resolve_delta(&prev, curr) {
            None => {
                is_unsafe = true;
            },
            Some(is_increasing_now) => {
                match is_increasing_at_start {
                    None => {
                        is_increasing_at_start = Some(is_increasing_now);
                    }
                    Some(is_increasing_at_start) => {
                        if is_increasing_now != is_increasing_at_start {
                            is_unsafe = true;
                        }        
                    }
                }
            }
        }

        if is_unsafe {
            if removed_bad {
                break;
            }
            else {
                removed_bad = true;
                is_unsafe = false;
            }
        }
        else {
            prev = curr;
        }
    }
    
    return !is_unsafe;
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> i32 {
    let mut num_safe = 0;
    for line in input.lines() {
        if is_report_safe_part2(line) {
            num_safe += 1;
        }
    }
    return num_safe;
}

// mod tests {
//     #[test]
//     fn part2() {
//         assert_eq!(super::is_report_safe_part2("7 6 4 2 1"), true);
//         assert_eq!(super::is_report_safe_part2("1 2 7 8 9"), false);
//         assert_eq!(super::is_report_safe_part2("9 7 6 2 1"), false);
//         assert_eq!(super::is_report_safe_part2("1 3 2 4 5"), true);
//         assert_eq!(super::is_report_safe_part2("8 6 4 4 1"), true);
//         assert_eq!(super::is_report_safe_part2("1 3 6 7 9"), true);
//         assert_eq!(super::is_report_safe_part2("9 2 3 4 5"), true);
//         assert_eq!(super::is_report_safe_part2("19 19 21 20 22 25 27"), false);        
//         assert_eq!(super::is_report_safe_part2("10 11 8 7 6"), true);        
//         assert_eq!(super::is_report_safe_part2("10 9 11 12 13"), true);        
//         assert_eq!(super::is_report_safe_part2("92 95 91 89 88 87"), true);                
//     }
// }
