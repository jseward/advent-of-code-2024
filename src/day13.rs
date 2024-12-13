use aoc_runner_derive::aoc;

#[derive(Clone, Debug)]
struct Coord {
    x : i32,
    y : i32
}

#[derive(Clone, Debug)]
struct Machine {
    ba : Coord,
    bb : Coord,
    prize : Coord 
}

fn parse_coord(line : &str, split_char : char) -> Coord {
    let (_, coord_part) = line.split_once(':').unwrap();
    let (x, y) = coord_part.split_once(',').unwrap();
    let (_, xv) = x.split_once(split_char).unwrap();
    let (_, yv) = y.split_once(split_char).unwrap();
    Coord {
        x: xv.parse().unwrap(),
        y: yv.parse().unwrap()
    }
}

fn parse_input(input : &str) -> Vec<Machine> {
    let mut machines = vec!();
    let mut line_iter = input.lines().into_iter();
    loop {
        if let Some(line) = line_iter.next() {
            let ba = parse_coord(line, '+');
            let bb = parse_coord(line_iter.next().unwrap(), '+');
            let prize = parse_coord(line_iter.next().unwrap(), '=');
            machines.push(Machine{ba, bb, prize});
            line_iter.next();
        }
        else {
            break;
        }
    }
    machines
}

// fn get_best_pressses(machine : Machine) -> Option<(i32, i32)> {
//     get_best_pressses_int(&machine.ba, &machine.bb, &machine.prize, 100)
// }

// fn get_best_pressses_int(ba : &Coord, bb : &Coord, prize : &Coord, max_num : i32) -> Option<(i32, i32)> {
//     let max_num_a = max_num.min((prize.x / ba.x).min(prize.y / ba.y) + 1);
//     //println!("{max_num} - {max_num_a}");
//     //let max_num_b = 100.min(machine.prize.x / machine.bb.x).min(machine.prize.y / machine.bb.y);
//     let mut best_presses : Option<(i32, i32)> = None;
//     for num_a in 0..max_num_a {
//         let x = ba.x * num_a;
//         let y = ba.y * num_a;
//         let x_needed = prize.x - x;
//         let y_needed = prize.y - y;
//         if x_needed % bb.x == 0 {
//             let num_b = x_needed / bb.x;
//             if num_b * bb.y == y_needed {
//                 match best_presses {
//                     None => {
//                         best_presses = Some((num_a, num_b));
//                     }
//                     Some(old) => {
//                         println!("#####");                        
//                         if (old.0 * 3 + old.1) > (num_a * 3 + num_b) {
//                             best_presses = Some((num_a, num_b));
//                         }
//                     }
//                 }
//             }
//         }
//     } 
//     best_presses
// }

#[aoc(day13, part1)]
pub fn part1(input: &str) -> i32 {
    // let mut acc = 0;
    // let machines = parse_input(input);
    // for machine in machines {
    //     //println!("{:?}", machine);        
    //     if let Some((a, b)) = get_best_pressses(machine.clone()) {
    //         acc += (a * 3) + b;
    //         //println!("{:?} = {a} , {b} , {acc}", machine);
    //     }
    // }
    // acc

    let mut acc: f64 = 0.0;
    let machines = parse_input(input);
    for m in machines {
        // let px = (m.prize.x as f64) + 10000000000000.0;
        // let py = (m.prize.y as f64) + 10000000000000.0;
        let px = m.prize.x as f64;
        let py = m.prize.y as f64;        
        let ax = m.ba.x as f64;
        let ay = m.ba.y as f64;
        let bx = m.bb.x as f64;
        let by = m.bb.y as f64;        

        let b_f : f64 = (py - (ay * px / ax)) / ((-ay * bx / ax) + by);
        let b_i : u64 = b_f.round() as u64;
        if true {//(b_f - (b_i as f64)).abs() < 0.0000001 {
            let diff_x : f64 = px - (b_f * bx);
            let a_f = diff_x / ax;
            let a_i: u64 = a_f.round() as u64;                
            if true {//(a_f - (a_i as f64)).abs() < 0.0000001 {
                let fx = (a_i * m.ba.x as u64) + (b_i * m.bb.x as u64);
                let fy = (a_i * m.ba.y as u64) + (b_i * m.bb.y as u64);
                if fx == px as u64 && fy == py as u64 {
                    acc += ((a_i as f64) * 3.0) + b_i as f64;
                    //println!("{:?} = {a_f}({a_i}) , {b_f}({b_i}) , {acc}", m);         
                }
            }
        }
    }
    acc as i32    
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> u64 {
    let mut acc: f64 = 0.0;
    let machines = parse_input(input);
    for m in machines {
        let px = (m.prize.x as f64) + 10000000000000.0;
        let py = (m.prize.y as f64) + 10000000000000.0;
        //let px = m.prize.x as f64;
        //let py = m.prize.y as f64;        
        let ax = m.ba.x as f64;
        let ay = m.ba.y as f64;
        let bx = m.bb.x as f64;
        let by = m.bb.y as f64;        

        let b_f : f64 = (py - (ay * px / ax)) / ((-ay * bx / ax) + by);
        let b_i : u64 = b_f.round() as u64;
        if true {//(b_f - (b_i as f64)).abs() < 0.0000001 {
            let diff_x : f64 = px - (b_f * bx);
            let a_f = diff_x / ax;
            let a_i: u64 = a_f.round() as u64;                
            if true {//(a_f - (a_i as f64)).abs() < 0.0000001 {
                let fx = (a_i * m.ba.x as u64) + (b_i * m.bb.x as u64);
                let fy = (a_i * m.ba.y as u64) + (b_i * m.bb.y as u64);
                if fx == px as u64 && fy == py as u64 {
                    acc += ((a_i as f64) * 3.0) + b_i as f64;
                    //println!("{:?} = {a_f}({a_i}) , {b_f}({b_i}) , {acc}", m);         
                }
            }
        }
    }
    acc as u64
}

// #[test]
// fn test_part1() {
//     let ans = part2("Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400

// Button A: X+26, Y+66
// Button B: X+67, Y+21
// Prize: X=12748, Y=12176

// Button A: X+17, Y+86
// Button B: X+84, Y+37
// Prize: X=7870, Y=6450

// Button A: X+69, Y+23
// Button B: X+27, Y+71
// Prize: X=18641, Y=10279");
//     assert_eq!(ans, 480);
// }

// #[test]
// fn test_x() {
//     let x : f64 = 5400.0 - (34.0 * 8400.0 / 94.0);
//     let y : f64 = ((-34.0 / 94.0) * 22.0) + 67.0;
//     let z = x / y;
//     assert_eq!(z, 40.0);
// }