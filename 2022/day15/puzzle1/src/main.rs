extern crate regex;
use std::{fs, collections::HashMap};
use regex::Regex;

#[derive(Debug)]
enum Point {
    Sensor,
    Beacon
}

#[derive(Debug)]
struct Sensor {
    coordinates: (i32, i32),
    manhattan_dist_to_nearest_beacon: i32
}

fn get_manhattan_distance(point_a: (i32, i32), point_b: (i32, i32)) -> i32 {
    return (point_a.0 - point_b.0).abs() + (point_a.1 - point_b.1).abs(); 
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

    let y = 2000000;

    let min_x = sensors.iter().map(|x| x.coordinates.0).min().unwrap();
    let max_x = sensors.iter().map(|x| x.coordinates.0).max().unwrap();
    let max_distance = sensors.iter().map(|x| x.manhattan_dist_to_nearest_beacon).max().unwrap();

    let mut covered = 0;


    for x in (min_x - max_distance)..(max_x + max_distance) {
        let is_beacon = beacons.get(&(x, y));
        if let Some(beacon) = is_beacon {
            continue;
        }

        for sensor in &sensors {
            let dist_from_sensor = get_manhattan_distance((x, y), sensor.coordinates);

            if dist_from_sensor <= sensor.manhattan_dist_to_nearest_beacon {
                covered += 1;
                break;
            }
        }
    }


    println!("Covered: {covered}");
}
