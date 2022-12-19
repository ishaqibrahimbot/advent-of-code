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

fn update_sides_covered(coordinates: (i32, i32, i32), cube: &mut Cube, cubes: &mut HashMap<(i32, i32, i32), Cube>) {
    let (x, y, z) = coordinates;

    // x + 1
    if !cube.sides_covered.contains(&Side::XPlusOne) {
        let coord = (x + 1, y, z);
        let exists = cubes.contains_key(&coord);
        if exists {
            update_sides(coordinates, coord, cubes, Side::XPlusOne);
        }
    }

    // x - 1
    if !cube.sides_covered.contains(&Side::XMinusOne) {
        let coord = (x - 1, y, z);
        let exists = cubes.contains_key(&coord);
        if exists {
            update_sides(coordinates, coord, cubes, Side::XMinusOne);
        }
    }

    // y + 1
    if !cube.sides_covered.contains(&Side::YPlusOne) {
        let coord = (x, y + 1, z);
        let exists = cubes.contains_key(&coord);
        if exists {
            update_sides(coordinates, coord, cubes, Side::YPlusOne);
        }
    }

    // y - 1
    if !cube.sides_covered.contains(&Side::YMinusOne) {
        let coord = (x, y - 1, z);
        let exists = cubes.contains_key(&coord);
        if exists {
            update_sides(coordinates, coord, cubes, Side::YMinusOne);
        }
    }

    // z + 1
    if !cube.sides_covered.contains(&Side::ZPlusOne) {
        let coord = (x, y, z + 1);
        let exists = cubes.contains_key(&coord);
        if exists {
            update_sides(coordinates, coord, cubes, Side::ZPlusOne);
        }
    }

    // z - 1
    if !cube.sides_covered.contains(&Side::ZMinusOne) {
        let coord = (x, y, z - 1);
        let exists = cubes.contains_key(&coord);
        if exists {
            update_sides(coordinates, coord, cubes, Side::ZMinusOne);
        }
    }

}

fn main() {
    let path = "./input.txt";
    let contents = fs::read_to_string(path).expect("FAILURE");

    let mut cubes: HashMap<(i32, i32, i32), Cube> = HashMap::new();

    for cube in contents.split("\n") {
        let coordinates: Vec<i32> = cube.split(",").map(|ord| ord.parse::<i32>().unwrap()).collect();

        let cube = Cube {
            sides_covered: HashSet::new()
        };
        
        let x = coordinates.get(0).unwrap();
        let y = coordinates.get(1).unwrap();
        let z = coordinates.get(2).unwrap();

        cubes.insert((*x, *y, *z), cube);
    }

    let  mut cubes_copy = cubes.clone();

    for (coord, mut cube) in cubes {
        update_sides_covered(coord, &mut cube, &mut cubes_copy);
    }

    let mut sides_exposed = 0;
    for cube in cubes_copy.values() {
        let sides_covered = cube.get_num_covered_sides();
        sides_exposed += (6 - sides_covered);
    }

    println!("num sides exposed: {sides_exposed}");
}
