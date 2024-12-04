use aoc_runner_derive::aoc;

fn get_char_at(lines : &Vec<&str>, x : i32, y : i32) -> Option<char> {
    if x >= 0 && y >= 0 {
        if let Some(line) = lines.get(y as usize) {
            if x < line.len() as i32 {
                return Some(line.as_bytes()[x as usize] as char);
            }
        }
    }
    None
}

const XMAS : &str = "XMAS";
const DELTAS : &'static[(i32, i32)] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1)
];

fn find_xmas_from(lines : &Vec<&str>, x : i32, y : i32, i : usize, delta : &(i32, i32)) -> i32 {
    match get_char_at(lines, x, y) {
        Some(c) => {
            if c == XMAS.as_bytes()[i] as char {
                let next_i = i + 1;
                if next_i == XMAS.len() {
                    //println!("{} {}", x, y);
                    return 1;
                }
                else {
                    return find_xmas_from(&lines, x + delta.0, y + delta.1, i + 1, &delta);
                }
            }
        },
        None => {}
    }
    0
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> i32 {
    let mut total_matches = 0;
    let lines : Vec<&str> = input.lines().collect();
    let width : i32 = lines.get(0).unwrap().len() as i32;
    let height : i32 = lines.len() as i32;
    for y in 0..height {
        for x in 0..width {
            if get_char_at(&lines, x, y) == Some('X') {
                for delta in DELTAS.iter() {
                    total_matches += find_xmas_from(&lines, x + delta.0, y + delta.1, 1, &delta);
                }
            }
        }
    }
    total_matches
}

fn is_mas(lines : &Vec<&str>, x : i32, y : i32, d0 : (i32, i32), d1 : (i32, i32)) -> bool {
    if let Some(c0) = get_char_at(lines, x + d0.0, y + d0.1) {
        if let Some(c1) = get_char_at(lines, x + d1.0, y + d1.1) {
            return (c0 == 'M' && c1 == 'S') || (c0 == 'S' && c1 == 'M');
        }
    }
    return false;
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> i32 {
    let mut total_matches = 0;
    let lines : Vec<&str> = input.lines().collect();
    let width : i32 = lines.get(0).unwrap().len() as i32;
    let height : i32 = lines.len() as i32;
    for y in 0..height {
        for x in 0..width {
            if get_char_at(&lines, x, y) == Some('A') {
                if 
                    is_mas(&lines, x, y, (-1, -1), (1, 1)) &&
                    is_mas(&lines, x, y, (-1, 1), (1, -1)) {
                        total_matches += 1;
                }
            }
        }
    }
    total_matches
}

#[test]
fn test() {
    assert_eq!(part2("MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"), 9);
}
