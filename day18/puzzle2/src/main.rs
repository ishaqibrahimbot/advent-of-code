use std::{fs, collections::{HashSet, HashMap}};

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
enum Side {
    XPlusOne,
    XMinusOne,
    YPlusOne,
    YMinusOne,
    ZPlusOne,
    ZMinusOne
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Cube {
    sides_covered: HashSet<Side>
}

impl Cube {
    fn add_side(&mut self, side: Side) {
        self.sides_covered.insert(side);
    }
    fn get_num_covered_sides(&self) -> i32 {
        return self.sides_covered.len() as i32;
    }
}

fn get_opposite_side(side: Side) -> Side {
    match side {
        Side::XMinusOne => Side::XPlusOne,
        Side::XPlusOne => Side::XMinusOne,
        Side::YPlusOne => Side::YMinusOne,
        Side::YMinusOne => Side::YPlusOne,
        Side::ZPlusOne => Side::ZMinusOne,
        Side::ZMinusOne => Side::ZPlusOne
    }
}

fn update_sides(cube: (i32, i32, i32), neighbor: (i32, i32, i32), cubes: &mut HashMap<(i32, i32, i32), Cube>, side: Side) {
    let current_cube = cubes.get_mut(&cube).unwrap();
    current_cube.add_side(side);
    
    let neighbor_cube = cubes.get_mut(&neighbor).unwrap();
    let opposite_side = get_opposite_side(side);
    neighbor_cube.add_side(opposite_side);
}

fn get_new_ord(side: Side, i: i32, coordinates: (i32, i32, i32)) -> (i32, i32, i32) {
    match side {
        Side::XPlusOne => (coordinates.0 + i, coordinates.1, coordinates.2),
        Side::XMinusOne => (coordinates.0 - i, coordinates.1, coordinates.2),
        Side::YPlusOne => (coordinates.0, coordinates.1 + i, coordinates.2),
        Side::YMinusOne => (coordinates.0, coordinates.1 - i, coordinates.2),
        Side::ZPlusOne => (coordinates.0, coordinates.1, coordinates.2 + i),
        Side::ZMinusOne => (coordinates.0, coordinates.1, coordinates.2 - i)
    }
}

fn check(coordinates: (i32, i32, i32), cubes: &mut HashMap<(i32, i32, i32), Cube>, side: Side, differentials: (i32, i32, i32)) {
    
    let mut i = 1;

    println!("Checking: {:?}, {:?}", coordinates, side);

    let max = differentials.0.max(differentials.1).max(differentials.2);

    while i < max {
        let coord = get_new_ord(side, i, coordinates);
        let exists = cubes.contains_key(&coord);

        if exists {
            println!("Found neighbor: {i}, neighbor: {:?}", coord);
            update_sides(coordinates, coord, cubes, side);
            return;
        }

        println!("no neighbor: {i}");
        i += 1;
    }
    
}

fn update_sides_covered(coordinates: (i32, i32, i32), cube: &mut Cube, cubes: &mut HashMap<(i32, i32, i32), Cube>, differentials: (i32, i32, i32)) {
    let (x, y, z) = coordinates;

    // x + 1
    if !cube.sides_covered.contains(&Side::XPlusOne) {
        check(coordinates, cubes, Side::XPlusOne, differentials);
    }

    // x - 1
    if !cube.sides_covered.contains(&Side::XMinusOne) {
        check(coordinates, cubes, Side::XMinusOne, differentials);
    }

    // y + 1
    if !cube.sides_covered.contains(&Side::YPlusOne) {
        check(coordinates, cubes, Side::YPlusOne, differentials);
    }

    // y - 1
    if !cube.sides_covered.contains(&Side::YMinusOne) {
        check(coordinates, cubes, Side::YMinusOne, differentials);
    }

    // z + 1
    if !cube.sides_covered.contains(&Side::ZPlusOne) {
        check(coordinates, cubes, Side::ZPlusOne, differentials);
    }

    // z - 1
    if !cube.sides_covered.contains(&Side::ZMinusOne) {
        check(coordinates, cubes, Side::ZMinusOne, differentials);
    }

}

fn main() {
    let path = "./sample.txt";
    let contents = fs::read_to_string(path).expect("FAILURE");

    let mut cubes: HashMap<(i32, i32, i32), Cube> = HashMap::new();

    let mut lowest_x = 10000;
    let mut highest_x = 0;
    let mut lowest_y = 10000;
    let mut highest_y = 0;
    let mut lowest_z = 10000;
    let mut highest_z = 0;

    for cube in contents.split("\n") {
        let coordinates: Vec<i32> = cube.split(",").map(|ord| ord.parse::<i32>().unwrap()).collect();

        let cube = Cube {
            sides_covered: HashSet::new()
        };
        
        let x = coordinates.get(0).unwrap();
        let y = coordinates.get(1).unwrap();
        let z = coordinates.get(2).unwrap();

        if *x < lowest_x {
            lowest_x = *x;
        }

        if *x > highest_x {
            highest_x = *x;
        }

        if *y < lowest_y {
            lowest_y = *y;
        }

        if *y > highest_y {
            highest_y = *y;
        }

        if *z < lowest_z {
            lowest_z = *z;
        }

        if *z > highest_z {
            highest_z = *z;
        }

        cubes.insert((*x, *y, *z), cube);
    }

    let dx = highest_x - lowest_x;
    let dy = highest_y - lowest_y;
    let dz = highest_z - lowest_z;

    let  mut cubes_copy = cubes.clone();

    for (coord, mut cube) in cubes {
        update_sides_covered(coord, &mut cube, &mut cubes_copy, (dx, dy, dz));
    }

    let mut sides_exposed = 0;
    for cube in cubes_copy.values() {
        let sides_covered = cube.get_num_covered_sides();
        sides_exposed += (6 - sides_covered);
    }

    println!("num sides exposed: {sides_exposed}");
}
