use std::{fs, collections::{HashMap, HashSet, VecDeque}};

#[derive(Debug, Clone, Copy)]
pub enum RockType {
    Minus,
    Plus,
    Angle,
    Pipe,
    Box
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Down,
    Left,
    Right
}

#[derive(Debug, Clone)]
pub struct Rock {
    rock_type: RockType,
    coordinates: HashSet<(i64, i64)>
}

impl Rock {
    pub fn new(rock_type: &RockType, y: i64) -> Self {
        match rock_type {
            RockType::Minus => {
                Rock {
                    rock_type: RockType::Minus,
                    coordinates: HashSet::from([(2, y), (3, y), (4, y), (5, y)])
                }
            },
            RockType::Plus => {
                Rock {
                    rock_type: RockType::Plus,
                    coordinates: HashSet::from([(3, y), (3, y + 1), (3, y + 2), (2, y + 1), (4, y + 1)])
                }
            },
            RockType::Pipe => {
                Rock {
                    rock_type: RockType::Pipe,
                    coordinates: HashSet::from([(2, y), (2, y + 1), (2, y + 2), (2, y + 3)])
                }
            },
            RockType::Box => {
                Rock {
                    rock_type: RockType::Box,
                    coordinates: HashSet::from([(2, y), (3, y), (2, y + 1), (3, y + 1)])
                }
            },
            RockType::Angle => {
                Rock {
                    rock_type: RockType::Angle,
                    coordinates: HashSet::from([(2, y), (3, y), (4, y), (4, y + 1), (4, y + 2)])
                }
            }
        }
    }
    pub fn can_move(&self, direction: Direction, state: &HashSet<(i64, i64)>) -> bool {
        match direction {
            Direction::Right => {
                let new_coordinates: HashSet<(i64, i64)> = self.coordinates.iter().map(|point| (point.0 + 1, point.1)).collect();
                for coordinate in new_coordinates {
                    if coordinate.0 > 6 {
                        return false;
                    }

                    if state.contains(&coordinate) {
                        return false;
                    }
                }
                return true;
            },
            Direction::Left => {
                let new_coordinates: HashSet<(i64, i64)> = self.coordinates.iter().map(|point| (point.0 - 1, point.1)).collect();
                for coordinate in new_coordinates {
                    if coordinate.0 < 0 {
                        return false;
                    }

                    if state.contains(&coordinate) {
                        return false;
                    }
                }

                return true;
            },
            Direction::Down => {
                let new_coordinates: HashSet<(i64, i64)> = self.coordinates.iter().map(|point| (point.0, point.1 - 1)).collect();

                for coordinate in new_coordinates {
                    if coordinate.1 < 0 {
                        return false;
                    }

                    if state.contains(&coordinate) {
                        return false;
                    }
                }

                return true;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    state: HashSet<(i64, i64)>,
    highest_y: i64,
    rocks: VecDeque<RockType>,
    rock_in_motion: Option<Rock>
}

impl Map {
    pub fn new() -> Self {
        let mut initial_map: HashSet<(i64, i64)> = HashSet::new();

        Map {
            state: initial_map,
            highest_y: -1,
            rocks: VecDeque::from([RockType::Minus, RockType::Plus, RockType::Angle, RockType::Pipe, RockType::Box]),
            rock_in_motion: None
        }
    }
    pub fn new_rock(&mut self) {

        let rock_type = self.rocks.pop_front().unwrap();
        let rock = Rock::new(&rock_type, self.highest_y + 4);
        self.rocks.push_back(rock_type);
        self.rock_in_motion = Some(rock);
    }
    pub fn move_rock(&mut self, direction: Direction) -> (bool, bool) {
        let rock_option = self.rock_in_motion.clone();

        if let Some(rock) = rock_option {
            let can_move = rock.can_move(direction, &self.state);

            if direction == Direction::Down && !can_move {
                
                let resting_coordinates = rock.coordinates;

                for coordinate in resting_coordinates {
                    self.state.insert(coordinate);
                }

                return (false, true);
            } else if !can_move {
                return (false, false);
            } else {
                //move

                let old_coordinates = rock.coordinates;
                let new_coordinates: HashSet<(i64, i64)> = old_coordinates.iter().map(|point| {
                    match direction {
                        Direction::Down => {
                            (point.0, point.1 - 1)
                        },
                        Direction::Right => {
                            (point.0 + 1, point.1)
                        },
                        Direction::Left => {
                            (point.0 - 1, point.1)
                        }
                    }
                }).collect();

                self.rock_in_motion.as_mut().unwrap().coordinates = new_coordinates.clone();

                return (true, false);
            }

        }

        (false, false)

    }
    pub fn reset_highest_y(&mut self) {

        let mut highest = -2;

        for coordinate in &self.state {
            if coordinate.1 > highest {
                highest = coordinate.1;
            }
        }

        self.highest_y = highest;
    }
}

fn main() {
    let path = "./input.txt";
    let contents = fs::read_to_string(path).expect("FAILURE");

    let mut directions: VecDeque<Direction> = VecDeque::new();

    for char in contents.chars() {
        if char == '>' {
            directions.push_back(Direction::Right);
        } else if char == '<' {
            directions.push_back(Direction::Left);
        }
    }

    let mut game = Map::new();

    game.new_rock();
    let mut num_rocks: i64 = 0;
    let mut i = 0;

    loop {
        // println!("iteration: {}", i);
        i += 1;
        let direction = directions.pop_front().unwrap();
        // println!("{:?}", direction);
        game.move_rock(direction);
        directions.push_back(direction);

        // println!("Moving down");
        let (moved, settled) = game.move_rock(Direction::Down);

        // println!("moved: {moved}, settled: {settled}");

        if settled {
            num_rocks += 1;
            game.reset_highest_y();
            println!("{}", num_rocks);
            if num_rocks == 2022 {
                break;
            } else {
                game.new_rock();
                // println!("Time for a new rock");
            }
        }

    }

    println!("Units: {:?}", game.highest_y + 1);
    // println!("Rock in motion: {:?}", game.rock_in_motion);
}
