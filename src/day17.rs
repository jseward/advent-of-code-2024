use aoc_runner_derive::aoc;

struct State
{
    a : u64,
    b : u64,
    c : u64,
    instructions : Vec<u64>,
    iptr : usize,
    out : Vec<u64>
}

fn parse_input(input: &str) -> State {
    let mut lines = input.lines();
    let a = lines.next().unwrap().split_once(':').unwrap().1.trim().parse().unwrap();
    let b = lines.next().unwrap().split_once(':').unwrap().1.trim().parse().unwrap();
    let c = lines.next().unwrap().split_once(':').unwrap().1.trim().parse().unwrap();
    lines.next();
    let instructions = lines.next().unwrap().split_once(':').unwrap().1.trim().split(',').map(|s| s.parse().unwrap()).collect();
    State {
        a, b, c, instructions,
        iptr : 0,
        out : vec!()
    }
}

impl State {
    pub fn run(&mut self) {
        loop {
            match self.instructions.get(self.iptr) {
                Some(ins) => {
                    self.iptr = self.execute(*ins, *self.instructions.get(self.iptr + 1).unwrap());
                }
                None => {
                    break;
                }
            }
        }
    }

    fn get_combo_op(&self, lit_op : u64) -> u64 {
        match lit_op {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!()
        }
    }

    fn execute(&mut self, ins : u64, lit_op : u64) -> usize {
        let mut jmp_iptr = None;
        match ins {
            0 => {
                let num = self.a;
                let den = 2_u64.pow(self.get_combo_op(lit_op) as u32);
                self.a = num / den;
            },
            1 => {
                self.b = self.b ^ lit_op;
            },
            2 => {
                self.b = self.get_combo_op(lit_op) % 8;
            },
            3 => {
                if self.a != 0 {
                    jmp_iptr = Some(lit_op as usize);
                }
            },
            4 => {
                self.b = self.b ^ self.c;  
            },
            5 => {
                self.out.push(self.get_combo_op(lit_op) % 8);
            },
            6 => {
                let num = self.a;
                let den = 2_u64.pow(self.get_combo_op(lit_op) as u32);
                self.b = num / den;
            },
            7 => {
                let num = self.a;
                let den = 2_u64.pow(self.get_combo_op(lit_op) as u32);
                self.c = num / den;
            },
            _ => { panic!(); }
        }
        jmp_iptr.unwrap_or(self.iptr + 2)
    }
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> String {
    let mut state = parse_input(input);
    state.run();
    state.out.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(",")
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> u64 {
    let mut state = parse_input(input);
    let mut a_components = vec!();
    let mut ai = 1;
    let mut out_a;
    loop {
        if state.instructions.len() > a_components.len() {
            let ci = state.instructions.len() - a_components.len() - 1;
            let cv = ai * 8_u64.pow(ci as u32);
            state.a = a_components.iter().sum::<u64>() + cv;

            state.iptr = 0;
            state.out.clear();
            state.run();
    
            println!("{:?} != {:?}", state.out, state.instructions);
 
            let c_expect = state.instructions.get(ci).unwrap(); 
            let c_current = state.out.get(ci).unwrap();
            if c_expect == c_current {
                a_components.push(cv);
                ai = 1;
            }
            else {
                ai += 1;
            }    
        }
        else {
            out_a = a_components.iter().sum::<u64>() + ai;
            state.a = out_a;

            state.iptr = 0;
            state.out.clear();
            state.run();
            println!("{:?} ?= {:?}", state.out, state.instructions);
            if state.out == state.instructions {
                break;
            }

            ai += 1;
        }
    }
    out_a
}

// #[test]
// fn test() {
//     let input = "Register A: 0
// Register B: 29
// Register C: 0

// Program: 1,7";
//     let mut state = parse_input(input);
//     state.run();
//     assert_eq!(state.b, 26);
//     //assert_eq!(state.out, "4,6,3,5,6,3,5,2,1,0");
// }
