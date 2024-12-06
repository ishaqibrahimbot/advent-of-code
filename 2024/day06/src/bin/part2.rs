use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt").to_string();
    let mut terrain: HashMap<(i32, i32), char> = HashMap::new();
    let mut starting_position: (i32, i32) = (0, 0);

    input.split("\n").enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '^' {
                starting_position = (x as i32, y as i32);
                terrain.insert(starting_position, '.');
            } else {
                terrain.insert((x as i32, y as i32), c);
            }
        });
    });

    println!("starting_position: {:?}", starting_position);

    let mut num_possible_obstruction_positions = 0;

    for (key, value) in terrain.iter() {
        if value != &'.' {
            continue;
        }
        if *key == starting_position {
            continue;
        }
        let mut cloned_terrain = terrain.clone();
        cloned_terrain.insert(*key, '#');

        let mut guard = Guard {
            direction: Direction::North,
            position: starting_position,
        };

        let mut loop_iter = 0;
        loop {
            let (did_walk, next_position) = guard.walk(&cloned_terrain);
            if !did_walk {
                break;
            }
            loop_iter += 1;
            if loop_iter > 10000 {
                num_possible_obstruction_positions += 1;
                break;
            }
        }
    }

    println!("{}", num_possible_obstruction_positions);
}

struct Guard {
    direction: Direction,
    position: (i32, i32),
}

#[derive(PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Guard {
    fn walk(&mut self, terrain: &HashMap<(i32, i32), char>) -> (bool, (i32, i32)) {
        let next_position = self.get_next_position(terrain);
        if let Some(next_position) = next_position {
            self.position = next_position;
            return (true, next_position);
        } else {
            return (false, (-10000, -10000));
        }
    }

    fn change_direction(&mut self) {
        match self.direction {
            Direction::North => self.direction = Direction::East,
            Direction::East => self.direction = Direction::South,
            Direction::South => self.direction = Direction::West,
            Direction::West => self.direction = Direction::North,
        }
    }

    fn get_next_position(&mut self, terrain: &HashMap<(i32, i32), char>) -> Option<(i32, i32)> {
        let (x, y) = self.position;
        let possible_position = match self.direction {
            Direction::North => (x, y - 1),
            Direction::East => (x + 1, y),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y),
        };

        let next_position_terrain = terrain.get(&possible_position);

        if next_position_terrain == Some(&'.') {
            return Some(possible_position);
        }

        if next_position_terrain == Some(&'#') {
            self.change_direction();
            return self.get_next_position(terrain);
        }

        // println!("no valid position: {:?}", possible_position);

        return None;
    }
}
