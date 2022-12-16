extern crate regex;
use std::{fs, collections::{HashMap, HashSet}};
use regex::Regex;

#[derive(Debug)]
enum Point {
    Sensor,
    Beacon
}

#[derive(Debug, Clone, Copy)]
struct Sensor {
    coordinates: (i32, i32),
    manhattan_dist_to_nearest_beacon: i32
}

fn add_intermediate_points(point_a: (i32, i32), point_b: (i32, i32), intermediate_points: &mut HashSet<(i32, i32)>) {
    let (a_x, a_y) = point_a;
    let (b_x, b_y) = point_b;

    let dx: i32 = b_x - a_x;
    let dy: i32 = b_y - a_y;

    let unit_x = if dx == 0 { 0 } else { dx / dx.abs() };
    let unit_y = if dy == 0 { 0 } else { dy / dy.abs() };

    let mut next_x: i32 = a_x;
    let mut next_y: i32 = a_y;

    intermediate_points.insert((a_x, a_y));

    loop {
        next_x = next_x + unit_x;
        next_y = next_y + unit_y;

        if next_x == b_x && next_y == b_y {
            intermediate_points.insert((next_x, next_y));
            break;
        }

        intermediate_points.insert((next_x, next_y));
    }

}

fn get_manhattan_distance(point_a: (i32, i32), point_b: (i32, i32)) -> i32 {
    return (point_a.0 - point_b.0).abs() + (point_a.1 - point_b.1).abs(); 
}

fn is_in_range(point: (i32, i32)) -> bool {
    let max_x = 4000000;
    let max_y = 4000000;

    let (x, y) = point;

    if x >= 0 && y >= 0 && x <= max_x && y <= max_y {
        return true;
    }

    return false;
}

fn get_peripheral_points(sensor: &Sensor) -> Vec<(i32, i32)> {
    let mut peripheral_points: HashSet<(i32, i32)> = HashSet::new();

    let dist = sensor.manhattan_dist_to_nearest_beacon;
    let (sensor_x, sensor_y) = sensor.coordinates;

    let top = (sensor_x, sensor_y - dist - 1);
    let bottom = (sensor_x, sensor_y + dist + 1);
    let right = (sensor_x + dist + 1, sensor_y);
    let left = (sensor_x - dist - 1, sensor_y);

    add_intermediate_points(top, right, &mut peripheral_points);
    add_intermediate_points(right, bottom, &mut peripheral_points);
    add_intermediate_points(bottom, left, &mut peripheral_points);
    add_intermediate_points(left, top, &mut peripheral_points);

    peripheral_points.into_iter().collect::<Vec<(i32, i32)>>()
}

fn is_away_from_all_sensors(point: (i32, i32), sensors: &Vec<Sensor>) -> bool {

    for sensor in sensors {
        let dist = get_manhattan_distance(point, sensor.coordinates);

        if dist <= sensor.manhattan_dist_to_nearest_beacon {
            return false;
        }
    }

    return true;
}

fn main() {
    let path = "./input.txt";
    let contents = fs::read_to_string(path).expect("YOU FAILURE");

    let re = Regex::new(r"=(?P<num>(\-)?(\d*))").unwrap();

    let mut sensors: Vec<Sensor> = Vec::new();
    let mut beacons: HashMap<(i32, i32), Point> = HashMap::new();

    for line in contents.split("\n") {
        let mut captures = re.captures_iter(line);
    
        let sensor_x = captures.next().unwrap().name("num").map_or_else(|| None, |x| Some(x.as_str())).unwrap();
        let sensor_y = captures.next().unwrap().name("num").map_or_else(|| None, |x| Some(x.as_str())).unwrap();
    
        let beacon_x = captures.next().unwrap().name("num").map_or_else(|| None, |x| Some(x.as_str())).unwrap();
        let beacon_y = captures.next().unwrap().name("num").map_or_else(|| None, |x| Some(x.as_str())).unwrap();

        let sensor = (sensor_x.parse::<i32>().unwrap(), sensor_y.parse::<i32>().unwrap());
        let beacon = (beacon_x.parse::<i32>().unwrap(), beacon_y.parse::<i32>().unwrap());

        beacons.insert(beacon, Point::Beacon);
        
        let man_distance = get_manhattan_distance(sensor, beacon);

        sensors.push(Sensor {
            coordinates: sensor,
            manhattan_dist_to_nearest_beacon: man_distance
        });

    }

    let mut candidates: Vec<(i32, i32)> = Vec::new();

    let mut num_searches = 0;

    let sensors_copy = sensors.clone();

    let mut found = false;

    for sensor in sensors {
        let points = get_peripheral_points(&sensor);

        println!("Searching points for: {:?}", sensor);

        for point in points {
            num_searches += 1;
            let in_range = is_in_range(point);

            if !in_range {
                continue;
            }

            let is_away = is_away_from_all_sensors(point, &sensors_copy);

            if is_away {
                candidates.push(point);
                found = true;
                break;
            }

        }

        println!("Num searches: {}\n", num_searches);

        if found {
            break;
        }
    }

    println!("\ncandidates: {:#?}", candidates);
    println!("Num searches: {}", num_searches);
  
}
