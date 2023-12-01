extern crate pathfinding;
use pathfinding::prelude::bfs;
use std::fs;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Hash)]
struct Pos(i32, i32);

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
struct Node {
    pos: Pos,
    value: char
}

struct Terrain {
    nodes: HashMap<(i32, i32), Node>
}

impl Terrain {
    fn get_successors(&self, position: &Pos) -> Vec<Node> {
        let mut successors: Vec<Node> = Vec::new();
        let (x, y) = (position.0, position.1);
        let current_pos = self.nodes.get(&(x, y)).unwrap();
        let current_value = current_pos.value as i32;
        
        for dx in -1i32..=1 {
            for dy in -1i32..=1 {
                // no diagonals
                if (dx + dy).abs() != 1 {
                    continue
                }

                let neighbor = self.nodes.get(&(x + dx, y + dy));

                if let Some(neighbor) = neighbor {
                    let neighbor_value = neighbor.value as i32;

                    if neighbor_value - current_value <= 1 {
                        successors.push(neighbor.clone());
                    }
                }
            }
        }

        successors

    }

    fn print_path(&self, result: Vec<Pos>) {
        for position in result {
            let (x, y) = (position.0, position.1);
            let node = self.nodes.get(&(x, y)).unwrap();
            let value = node.value;
            print!("{value}");
            print!("->");
        }
    }
}

fn main() {
    let path = "./input.txt";
    let contents = fs::read_to_string(path).expect("FAILURE");

    let mut terrain_map: HashMap<(i32, i32), Node> = HashMap::new();
    let mut starting_position: Pos = Pos(0, 0);
    let mut ending_position: Pos = Pos(0, 0);

    for (x, row) in contents.split("\n").enumerate() {
        for (y, str) in row.chars().enumerate() {
            // println!("char: {str}");
            let value = str;

            if value == 'S' {
                // starting position
                starting_position = Pos(x as i32, y as i32);
                let node = Node {
                    pos: Pos(x as i32, y as i32),
                    value: 'a'
                };
                terrain_map.insert((x as i32, y as i32), node);

            } else if value == 'E' {
                // ending position
                ending_position = Pos(x as i32, y as i32);
                let node = Node {
                    pos: Pos(x as i32, y as i32),
                    value: 'z'
                };
                terrain_map.insert((x as i32, y as i32), node);

            } else {
                let node = Node {
                    pos: Pos(x as i32, y as i32),
                    value
                };
                terrain_map.insert((x as i32, y as i32), node);
            }
        }
        // println!(" ");
    }

    let terrain = Terrain {
        nodes: terrain_map
    };

    println!("{:?}, {:?}", starting_position, ending_position);

    let result = bfs(&starting_position, |p| terrain.get_successors(p).iter().map(| x: &Node | x.pos.clone()).collect::<Vec<_>>(), |p| *p == ending_position);

    let result = result.expect("LOL");

    println!("Num steps: {}", result.len() -1);
    println!("{:?}", result);

    terrain.print_path(result);
}
