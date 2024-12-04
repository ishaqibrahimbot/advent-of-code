struct Grid {
    elements: Vec<Vec<String>>,
}

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    LeftDiagonal,
    RightDiagonal,
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
        Direction::LeftDiagonal => {
            if x >= 1 && y >= 1 {
                positions.push((x - 1, y - 1));
                positions.push((x, y));
                positions.push((x + 1, y + 1));
            }
        }
        Direction::RightDiagonal => {
            if x >= 1 && y >= 1 {
                positions.push((x + 1, y - 1));
                positions.push((x, y));
                positions.push((x - 1, y + 1));
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
            if *char != String::from("A") {
                continue;
            }

            let mut is_valid = 0;

            for dir in vec![Direction::LeftDiagonal, Direction::RightDiagonal].iter() {
                let string = grid.get_string((x, y), *dir);
                if let Some(valid_string) = string {
                    if valid_string == "MAS" || valid_string == "SAM" {
                        is_valid += 1;
                    } else {
                        is_valid = 0;
                    }
                }
            }

            if is_valid == 2 {
                num_xmases += 1;
            }
        }
    }

    println!("xmases: {num_xmases}")
}
