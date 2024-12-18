use aoc_runner_derive::aoc;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Vec2 {
    pub x : i32,
    pub y : i32
}

#[derive(PartialEq, Eq, Debug, Clone, Hash, Copy)]
enum Facing {
    E,
    W,
    S,
    N
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
struct Node {
    pub pos : Vec2,
    pub facing : Facing
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct NodeCost {
    pub node : Node,
    pub cost : i32
}

impl Ord for NodeCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.cost < other.cost {
            return std::cmp::Ordering::Greater;
        }
        else if self.cost > other.cost {
            return std::cmp::Ordering::Less;
        }
        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for NodeCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(Self::cmp(self, other))
    }
}

#[derive(Debug)]
struct Maze {
    walls : HashSet<Vec2>,
    end : Vec2,
    start : Node
}

impl Maze {
    pub fn from_str(s : &str) -> Maze {
        let mut walls = HashSet::<Vec2>::new();
        let mut end = Vec2{x:0,y:0};
        let mut start_pos = Vec2{x:0,y:0}; 
        let mut y = 0;
        for line in s.lines() {
            let mut x = 0;
            for c in line.chars() {
                match c {
                    '#' => { walls.insert(Vec2{x,y}); },
                    '.' => {},
                    'E' => { end = Vec2{x,y} },
                    'S' => { start_pos = Vec2{x,y} },
                    _ => panic!()
                }
                x += 1;
            }
            y += 1;
        }
        Maze { walls, end, start : Node{pos : start_pos, facing: Facing::E} }
    }

    pub fn best_path_cost(&self) -> i32 {
        let mut heap = BinaryHeap::<NodeCost>::new();
        let mut costs = HashMap::<Node, i32>::new();
        costs.insert(self.start.clone(), 0);
        heap.push(NodeCost{node: self.start.clone(), cost:0});
        while !heap.is_empty() {
            let top = heap.pop().unwrap();
            for next_node in self.get_next_nodes(&top.node, &top.cost) {
                match costs.get(&next_node.node) {
                    Some(nc) => {
                        if next_node.cost < *nc {
                            costs.insert(next_node.node.clone(), next_node.cost);
                            heap.push(next_node);    
                        }
                    },
                    None => {
                        costs.insert(next_node.node.clone(), next_node.cost);
                        heap.push(next_node);
                    }
                }
            }
        }
        self.get_best_cost_inner(&costs)
    }

    fn get_best_cost_inner(&self, costs : &HashMap<Node, i32>) -> i32 {
        let mut best_cost : Option<i32> = None;
        for f in [Facing::E, Facing::W, Facing::N, Facing::S] {
            if let Some(c) = costs.get(&Node{pos:self.end.clone(), facing:f}) {
                if let Some(bc) = best_cost {
                    best_cost = Some(bc.min(*c));
                }
                else {
                    best_cost = Some(*c);
                }
            }

        }
        best_cost.unwrap()
    }

    pub fn get_tile_counts_on_best_paths(&self) -> i32 {
        let mut heap = BinaryHeap::<NodeCost>::new();
        let mut costs = HashMap::<Node, i32>::new();
        costs.insert(self.start.clone(), 0);
        heap.push(NodeCost{node: self.start.clone(), cost:0});
        while !heap.is_empty() {
            let top = heap.pop().unwrap();
            for next_node in self.get_next_nodes(&top.node, &top.cost) {
                match costs.get(&next_node.node) {
                    Some(nc) => {
                        if next_node.cost < *nc {
                            costs.insert(next_node.node.clone(), next_node.cost);
                            heap.push(next_node);    
                        }
                    },
                    None => {
                        costs.insert(next_node.node.clone(), next_node.cost);
                        heap.push(next_node);
                    }
                }
            }
        }

        let tiles = self.get_best_path_tiles(costs);
        self.print_tiles(&tiles);
        tiles.len() as i32
    }

    fn get_best_path_tiles(&self, costs : HashMap<Node, i32>) -> HashSet<Vec2> {
        let mut visited = vec!();

        let best_cost = self.get_best_cost_inner(&costs);

        let mut to_visit = vec!();
        for facing in [Facing::E, Facing::W, Facing::N, Facing::S] {
            to_visit.push(NodeCost{ node: Node{ pos: self.end.clone(), facing }, cost:best_cost});
        }

        let mut tiles = HashSet::<Vec2>::new();

        while !to_visit.is_empty() {
            let top = to_visit.pop().unwrap();
            tiles.insert(top.node.pos.clone());
            visited.push(top.node.clone());
            
            {
                let prev_delta = match top.node.facing {
                    Facing::E => (-1, 0),
                    Facing::W => (1, 0),
                    Facing::N => (0, 1),
                    Facing::S => (0, -1)
                };
                let prev_pos = Vec2 { x:top.node.pos.x + prev_delta.0, y:top.node.pos.y + prev_delta.1 };
                let prev_node = Node{pos:prev_pos, facing:top.node.facing};
                if !visited.contains(&prev_node) {
                    if let Some(prev_cost) = costs.get(&prev_node) {
                        if (prev_cost + 1) == top.cost {
                            to_visit.push(NodeCost{node:prev_node, cost:*prev_cost});           
                        }
                    }
                }
            }

            let prev_facings = match top.node.facing {
                Facing::E | Facing::W => [Facing::N, Facing::S],
                Facing::S | Facing::N => [Facing::W, Facing::E]
            };

            for prev_facing in prev_facings {
                let prev_facing_node = Node{pos:top.node.pos.clone(), facing:prev_facing};
                if !visited.contains(&prev_facing_node) {
                    if let Some(prev_cost) = costs.get(&prev_facing_node) {
                        if (prev_cost + 1000) == top.cost {
                            to_visit.push(NodeCost{node:prev_facing_node, cost:*prev_cost});           
                        }
                    }
                }                
            } 
        }

        tiles
    }

    fn print_tiles(&self, tiles : &HashSet<Vec2>) {
        let mut max_x = 0;
        let mut max_y = 0;
        for w in self.walls.iter() {
            max_x = w.x.max(max_x);
            max_y = w.y.max(max_y);
        }
        for y in 0..(max_y + 1) {
            let mut s = String::new();
            for x in 0..(max_x + 1) {
                if self.walls.contains(&Vec2{x,y}) {
                    s += "#";
                }
                else if tiles.contains(&Vec2{x,y}) {
                    s += "0";
                }
                else {
                    s += ".";
                }
            }
            println!("{}", s);
        }
    }

    fn get_next_nodes(&self, node: &Node, curr_cost : &i32) -> Vec<NodeCost> {
        let mut next_nodes = vec!();
        let move_delta =  match node.facing {
            Facing::E => (1, 0),
            Facing::W => (-1, 0),
            Facing::N => (0, -1),
            Facing::S => (0, 1)
        };
        let move_pos = Vec2{ x: node.pos.x + move_delta.0, y: node.pos.y + move_delta.1 };
        if !self.walls.contains(&move_pos) {
            next_nodes.push(NodeCost{node:Node{pos:move_pos, facing:node.facing.clone()}, cost:curr_cost + 1});
        }

        let mut add_turn_node = |new_facing| {
            next_nodes.push(NodeCost{node:Node{pos:node.pos.clone(), facing:new_facing}, cost:curr_cost + 1000});
        };

        if node.facing == Facing::W || node.facing == Facing::E {
            add_turn_node(Facing::N);
            add_turn_node(Facing::S);
        }
        else {
            add_turn_node(Facing::W);
            add_turn_node(Facing::E);
        }
        next_nodes
    }
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> i32 {
    let maze = Maze::from_str(input);
    maze.best_path_cost()
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> i32 {
    let maze = Maze::from_str(input);
    maze.get_tile_counts_on_best_paths()
}

#[test]
fn test() {
    let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
    let maze = Maze::from_str(input);
    assert_eq!(maze.get_tile_counts_on_best_paths(), 64);
}