use aoc_runner_derive::aoc;

#[derive(Debug)]
enum ParseState {
    None,
    M,
    U,
    L,
    LeftNumber,
    RightNumber,
}

fn parse_char(c : char) -> Option<i32> {
    match c {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        _ => None
    }
}

fn parts_to_value(parts : &[i32]) -> i32 {
    let mut value = 0;
    for i in 0..parts.len() {
        value += parts[i] * i32::pow(10, u32::try_from(parts.len() - 1 - i).unwrap());
    }
    value
}

struct ParseData {
    state : ParseState,
    left : Vec<i32>,
    right : Vec<i32>,
}

impl ParseData {
    pub fn new() -> ParseData {
        ParseData {
            state : ParseState::None,
            left : Vec::with_capacity(100),
            right : Vec::with_capacity(100),
        }
    }

    pub fn reset(&mut self) {
        self.state = ParseState::None;
        self.left.clear();
        self.right.clear();
    }

    pub fn to_mul(&self) -> i32 {
        parts_to_value(&self.left) * parts_to_value(&self.right)
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    let mut total = 0;

    let mut parse = ParseData::new();

    let mut iter = input.chars().into_iter();
    loop {
        match iter.next() {
            None => {
                break;
            },
            Some(c) => {
                match parse.state {
                    ParseState::None => {
                        if c == 'm' {
                            parse.state = ParseState::M;
                        }
                        else {
                            parse.reset();
                        }
                    },
                    ParseState::M => {
                        if c == 'u' {
                            parse.state = ParseState::U;
                        }
                        else {
                            parse.reset();
                        }
                    },
                    ParseState::U => {
                        if c == 'l' {
                            parse.state = ParseState::L;
                        }
                        else {
                            parse.reset();
                        }
                    },
                    ParseState::L => {
                        if c == '(' {
                            parse.state = ParseState::LeftNumber;
                        }
                        else {
                            parse.reset();
                        }
                    },
                    ParseState::LeftNumber => {
                        if c == ',' {
                            if parse.left.len() >= 1 && parse.left.len() <= 3 {
                                parse.state = ParseState::RightNumber;
                            }
                            else {
                                parse.reset();
                            }
                        }
                        else {
                            match parse_char(c) {
                                Some(v) => {
                                    parse.left.push(v);
                                }
                                None => {
                                    parse.reset();
                                }
                            }
                        }
                    },
                    ParseState::RightNumber => {
                        if c == ')' {
                            if parse.right.len() >= 1 && parse.right.len() <= 3 {
                                total += parse.to_mul();
                            }
                            parse.reset();
                        }
                        else {
                            match parse_char(c) {
                                Some(v) => {
                                    parse.right.push(v);
                                }
                                None => {
                                    parse.reset();
                                }
                            }
                        }
                    },
                }
            }
        }
    }
    total
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    let mut total = 0;

    let mut do_s = String::with_capacity(10);
    let mut will_do = true;
    let mut parse = ParseData::new();

    let mut iter = input.chars().into_iter();
    loop {
        match iter.next() {
            None => {
                break;
            },
            Some(c) => {
                match parse.state {
                    ParseState::None => {
                        if do_s.len() > 0 {
                            do_s.push(c);
                            if do_s == "do()" {
                                will_do = true;
                                do_s.clear();
                            }
                            else if do_s == "don't()" {
                                will_do = false;
                                do_s.clear();
                            }
                            else {
                                if !"do()".starts_with(&do_s) && !"don't()".starts_with(&do_s) {
                                    do_s.clear();
                                }
                            }
                        }
                        else {
                            if c == 'm' {
                                parse.state = ParseState::M;
                            }
                            else if c == 'd' {
                                do_s.push(c);
                            }         
                            else {
                                parse.reset();
                            }
                        }
                    },
                    ParseState::M => {
                        if c == 'u' {
                            parse.state = ParseState::U;
                        }
                        else {
                            parse.reset();
                        }
                    },
                    ParseState::U => {
                        if c == 'l' {
                            parse.state = ParseState::L;
                        }
                        else {
                            parse.reset();
                        }
                    },
                    ParseState::L => {
                        if c == '(' {
                            parse.state = ParseState::LeftNumber;
                        }
                        else {
                            parse.reset();
                        }
                    },
                    ParseState::LeftNumber => {
                        if c == ',' {
                            if parse.left.len() >= 1 && parse.left.len() <= 3 {
                                parse.state = ParseState::RightNumber;
                            }
                            else {
                                parse.reset();
                            }
                        }
                        else {
                            match parse_char(c) {
                                Some(v) => {
                                    parse.left.push(v);
                                }
                                None => {
                                    parse.reset();
                                }
                            }
                        }
                    },
                    ParseState::RightNumber => {
                        if c == ')' {
                            if parse.right.len() >= 1 && parse.right.len() <= 3 {
                                if will_do {
                                    total += parse.to_mul();
                                }
                            }
                            parse.reset();
                        }
                        else {
                            match parse_char(c) {
                                Some(v) => {
                                    parse.right.push(v);
                                }
                                None => {
                                    parse.reset();
                                }
                            }
                        }
                    },
                }
            }
        }
    }
    total
}

// #[test]
// fn test() {
//     assert_eq!(part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"), 161);
// }
