struct Grid {
    elements: Vec<Vec<String>>,
}

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownLeft,
    DownRight,
}

impl Grid {
    fn get(&self, pos: (usize, usize)) -> Option<&String> {
        let (x, y) = pos;
        let row = self.elements.get(y);
        // println!("Found row: {:?}", row);
        if let Some(valid_row) = row {
            let position = valid_row.get(x);
            // println!("found pos: {:?}", position);
            if let Some(valid_position) = position {
                return Some(valid_position);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
    fn get_string(&self, pos: (usize, usize), dir: Direction) -> Option<String> {
        let points = get_points(pos, dir, 3);
        let mut string = String::new();

        for point in points {
            if let Some(valid_char) = self.get(point) {
                string.push_str(valid_char.as_str());
            } else {
                return None;
            }
        }

        Some(string)
    }
}

fn get_points(pos: (usize, usize), dir: Direction, num_points: usize) -> Vec<(usize, usize)> {
    let (x, y) = pos;
    let mut positions: Vec<(usize, usize)> = vec![];

    match dir {
        Direction::Up => {
            for i in 0..=num_points {
                if y >= i {
                    positions.push((x, y - i));
                }
            }
        }
        Direction::Down => {
            positions.push((x, y));
            for i in 1..=num_points {
                positions.push((x, y + i));
            }
        }
        Direction::Left => {
            for i in 0..=num_points {
                if x >= i {
                    positions.push((x - i, y));
                }
            }
        }
        Direction::Right => {
            positions.push((x, y));
            for i in 1..=num_points {
                positions.push((x + i, y));
            }
        }
        Direction::UpRight => {
            for i in 0..=num_points {
                if y >= i {
                    let new_x = x.saturating_add(i);
                    positions.push((new_x, y - i));
                }
            }
        }
        Direction::UpLeft => {
            for i in 0..=num_points {
                if y >= i && x >= i {
                    positions.push((x - i, y - i));
                }
            }
        }
        Direction::DownLeft => {
            for i in 0..=num_points {
                if x >= i {
                    positions.push((x - i, y + i));
                }
            }
        }
        Direction::DownRight => {
            for i in 0..=num_points {
                let new_x = x.saturating_add(i);
                positions.push((new_x, y + i));
            }
        }
    }

    positions
}

fn main() {
    let input = include_str!("input.txt").to_string();

    let rows: Vec<Vec<String>> = input
        .split("\n")
        .map(|row| {
            return row.chars().map(|char| char.to_string()).collect();
        })
        .collect();

    let grid = Grid { elements: rows };

    let mut num_xmases = 0;

    for (y, row) in grid.elements.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if *char != String::from("X") {
                continue;
            }

            for dir in vec![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
                Direction::UpLeft,
                Direction::UpRight,
                Direction::DownLeft,
                Direction::DownRight,
            ]
            .iter()
            {
                let string = grid.get_string((x, y), *dir);
                if let Some(valid_string) = string {
                    if valid_string == "XMAS" {
                        println!("{valid_string}");
                        num_xmases += 1;
                    }
                }
            }
        }
    }

    println!("xmases: {num_xmases}")
}
