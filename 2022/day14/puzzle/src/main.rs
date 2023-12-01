use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
enum Point {
    Rock,
    Sand
}

fn populate_map(points: Vec<(i32, i32)>, map: &mut HashMap<(i32, i32), Point>) {

    let mut all_points: Vec<(i32, i32)> = Vec::new();

    let mut i = 0;

    let mut current_point = points.get(i).unwrap();

    loop {
        let mut next_point = points.get(i + 1);

        if let Some(next_point) = next_point {
            let intermediate_points = get_intermediate_points(*current_point, *next_point);
            all_points = [all_points, intermediate_points].concat();
            current_point = next_point;
            i += 1;
        } else {
            break;
        }
    }

    for point in all_points {
        map.insert(point, Point::Rock);
    }

}

fn get_intermediate_points(point_a: (i32, i32), point_b: (i32, i32)) -> Vec<(i32, i32)> {
    let (a_x, a_y) = point_a;
    let (b_x, b_y) = point_b;

    let dx: i32 = b_x - a_x;
    let dy: i32 = b_y - a_y;

    let mut intermediate_points: Vec<(i32, i32)> = Vec::new();

    let unit_x = if dx == 0 { 0 } else { dx / dx.abs() };
    let unit_y = if dy == 0 { 0 } else { dy / dy.abs() };

    let mut next_x: i32 = a_x;
    let mut next_y: i32 = a_y;

    intermediate_points.push((a_x, a_y));

    loop {
        next_x = next_x + unit_x;
        next_y = next_y + unit_y;

        if next_x == b_x && next_y == b_y {
            intermediate_points.push((next_x, next_y));
            break;
        }

        intermediate_points.push((next_x, next_y));
    }

    intermediate_points
}

fn bring_sand_to_rest(map: &mut HashMap<(i32, i32), Point>, lowest_rock: i32) -> bool {
    let mut starting_x = 500;
    let mut starting_y = 0;

    loop {
        let first_position = (starting_x, starting_y + 1);

        if first_position.1 > lowest_rock {
            return false;
        }

        let is_occupied = map.get(&first_position);

        if let Some(occupant) = is_occupied {
            // println!("{}, {} is occupied", first_position.0, first_position.1);
            let next_position = (starting_x - 1, starting_y + 1);

            let is_occupied = map.get(&next_position);

            if let Some(occupant) = is_occupied {
                // println!("{}, {} is occupied", next_position.0, next_position.1);

                let final_position = (starting_x + 1, starting_y + 1);

                let is_occupied = map.get(&final_position);

                if let Some(occupant) = is_occupied {
                    // println!("{}, {} is occupied", final_position.0, final_position.1);

                    // println!("Sand settled at {starting_x}, {starting_y}");
                    // no where to go, sand is at rest
                    map.insert((starting_x, starting_y), Point::Sand);
                    break;
                } else {
                    // can go right
                    (starting_x, starting_y) = final_position;
                    continue;
                }
            } {
                // can go left
                (starting_x, starting_y) = next_position;
                continue;
            }
        } else {
            // can go down
            (starting_x, starting_y) = first_position;
            continue;
        }
    }

    true

}

fn main() {
    let path = "./input.txt";
    let contents = fs::read_to_string(path).expect("YOU FAILURE");

    let mut map: HashMap<(i32, i32), Point> = HashMap::new();

    let mut lowest_rock: i32 = 0;

    for (index, line) in contents.split("\n").enumerate() {
        let points: Vec<(i32, i32)> = line.split(" -> ").map(|point_str| {
            let x_and_y: Vec<i32> = point_str.split(",").map(|pt| pt.parse::<i32>().unwrap()).collect();
            let x = x_and_y.first().unwrap();
            let y = x_and_y.last().unwrap();

            if *y > lowest_rock {
                lowest_rock = *y;
            }

            (*x, *y)
        }).collect();

        populate_map(points, &mut map);

    }

    println!("Lowest rock: {lowest_rock}");

    let mut num_rested = 0;

    loop {
        let settled = bring_sand_to_rest(&mut map, lowest_rock);

        if !settled {
            break;
        }

        num_rested += 1;
    }

    println!("Num rested: {num_rested}");
    println!("Map: {:?}", map);
}
