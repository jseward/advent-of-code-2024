use aoc_runner_derive::aoc;
use std::collections::*;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Coord {
    pub x : i32,
    pub y : i32
}

struct Map {
    plots : HashMap<Coord, char>
}

fn parse_input(input: &str) -> Map {
    let mut plots = HashMap::<Coord, char>::new();
    let mut y = 0;
    for line in input.lines() {
        let mut x = 0;
        for c in line.chars() {
            plots.insert(Coord{x, y}, c);
            x += 1;
        }
        y += 1;
    }
    Map { plots }
}

#[derive(Debug)]
struct Region {
    area : i32,
    perim : i32
}

enum GetRegionResult {
    AlreadyVisited,
    DifferentRegion,
    More(Region)
}

fn get_region(visited_plots : &mut HashMap<char, HashSet<Coord>>, map : &Map, pos : &Coord, region_type : &char) -> GetRegionResult {
    if visited_plots.entry(*region_type).or_insert(HashSet::<Coord>::new()).contains(&pos) {
        return GetRegionResult::AlreadyVisited;
    }

    match map.plots.get(pos) {
        Some(rt) => {
            if rt != region_type {
                return GetRegionResult::DifferentRegion;
            }
        }
        None => { return GetRegionResult::DifferentRegion; }
    }

    visited_plots.get_mut(&region_type.clone()).unwrap().insert(pos.clone());
    let mut r = Region { area: 1, perim: 0 };
    for offset in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        match get_region(visited_plots, map, &Coord{ x: pos.x + offset.0, y: pos.y + offset.1 }, region_type) {
            GetRegionResult::More(off_r) => {
                r.area += off_r.area;
                r.perim += off_r.perim;
            }
            GetRegionResult::DifferentRegion => {
                r.perim += 1;
            },
            GetRegionResult::AlreadyVisited => {                
            }
        }
    }
    GetRegionResult::More(r)
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> i32 {
    let map = parse_input(input);
    let mut acc = 0;
    let mut visited_plots = HashMap::<char, HashSet::<Coord>>::new();
    for (k, v) in map.plots.iter() {
        match get_region(&mut visited_plots, &map, k, v) {
            GetRegionResult::More(r) => {
                //println!("{:?} {} = {:?}", k, v, r);
                acc += r.area * r.perim;
            },
            _ => {}
        }
    }
    acc
}

#[derive(Debug)]
struct Region2 {
    area : i32,
    side_coords : Vec<SideCoord>
}

enum GetRegionResult2 {
    AlreadyVisited,
    DifferentRegion,
    More(Region2)
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum SideDir {
    LVer,
    RVer,
    THor,
    BHor
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct SideCoord {
    coord : Coord,
    dir : SideDir
}

fn get_region2(visited_plots : &mut HashMap<char, HashSet<Coord>>, map : &Map, pos : &Coord, region_type : &char) -> GetRegionResult2 {
    if visited_plots.entry(*region_type).or_insert(HashSet::<Coord>::new()).contains(&pos) {
        return GetRegionResult2::AlreadyVisited;
    }

    match map.plots.get(pos) {
        Some(rt) => {
            if rt != region_type {
                return GetRegionResult2::DifferentRegion;
            }
        }
        None => { return GetRegionResult2::DifferentRegion; }
    }

    visited_plots.get_mut(&region_type.clone()).unwrap().insert(pos.clone());
    let mut r = Region2 { area: 1, side_coords: vec!() };
    for offset in [(-1, 0, SideDir::LVer), (1, 0, SideDir::RVer), (0, -1, SideDir::THor), (0, 1, SideDir::BHor)] {
        let c = Coord{ x: pos.x + offset.0, y: pos.y + offset.1 };
        match get_region2(visited_plots, map, &c, region_type) {
            GetRegionResult2::More(mut off_r) => {
                r.area += off_r.area;
                r.side_coords.append(&mut off_r.side_coords);
            }
            GetRegionResult2::DifferentRegion => {
                r.side_coords.push(SideCoord { coord: c, dir : offset.2 });
            },
            GetRegionResult2::AlreadyVisited => {                
            }
        }
    }
    GetRegionResult2::More(r)
}

fn remove_side_coords(side_coords : &mut HashSet<SideCoord>, at : &SideCoord) {
    if side_coords.contains(at) {
        //println!("removing {:?}", at);
        side_coords.remove(at);
        for off in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let next = SideCoord { coord : Coord {x: at.coord.x + off.0, y : at.coord.y + off.1}, dir : at.dir.clone() };
            remove_side_coords(side_coords, &next);
        }
    }
}

fn get_num_sides(side_coords : Vec<SideCoord>) -> i32 {
    let mut set = HashSet::<SideCoord>::new();
    for sc in side_coords.iter() {
        set.insert(sc.clone());
    }

    let mut num_sides = 0;
    for sc in side_coords {
        if set.contains(&sc) {
            //println!("START removing {:?}", sc);
            num_sides += 1;
            remove_side_coords(&mut set, &sc);
        }
    }
    num_sides
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> i32 {
    let map = parse_input(input);
    let mut acc = 0;
    let mut visited_plots = HashMap::<char, HashSet::<Coord>>::new();
    for (k, v) in map.plots.iter() {
        match get_region2(&mut visited_plots, &map, k, v) {
            GetRegionResult2::More(r) => {
                let num_sides = get_num_sides(r.side_coords);
                //println!("{:?} {} = area={:?} , num_sides={}", k, v, r.area, num_sides);                
                acc += r.area * num_sides;
            },
            _ => {}
        }
    }
    acc
}

// #[test]
// fn test_part2() {
//     assert_eq!(
//         part2("EEEEE
// EXXXX
// EEEEE
// EXXXX
// EEEEE"), 236);
// }

// #[test]
// fn test_part2() {
//     assert_eq!(
//         part2("AAAA
// BBCD
// BBCC
// EEEC"), 80);
// }
