use std::collections::{HashMap, HashSet};

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

    let mut guard = Guard {
        direction: Direction::North,
        position: starting_position,
    };

    let mut can_walk = true;
    let mut points_covered = HashSet::new();
    points_covered.insert(starting_position);

    loop {
        let (did_walk, next_position) = guard.walk(&terrain);
        if !did_walk {
            break;
        }
        points_covered.insert(next_position);
    }

    println!("{}", points_covered.len());
}

// there is something that stores a direction and a current position
// and then it can "walk" but it needs to know the terrain for that
// the terrain is a hashmap where (x, y) => "." or "#"
// the walker can only walk on ".", it changes direction 90 degrees clockwise whenever it
// encounters a "#"
// on every iteration we just store the current position in a hashset and then count the number
// of values in that set

struct Guard {
    direction: Direction,
    position: (i32, i32),
}

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

        println!("no valid position: {:?}", possible_position);

        return None;
    }
}
