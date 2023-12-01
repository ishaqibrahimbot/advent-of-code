use std::fs;
use std::collections::HashMap;

// store bounds of grid
// make a hashmap of coordinate and tree
// tree --> struct with height and is_visible
// loop through each tree (value in hashmap) and determine whether it's visible

#[derive(Debug)]
struct Tree {
    height: u32,
    is_visible: bool,
    coordinates: (usize, usize)
}

fn get_map_of_trees(grid_rows: Vec<String>, max_x: usize, max_y: usize) -> HashMap<(usize, usize), Tree> {
    let mut map_of_trees: HashMap<(usize, usize), Tree> = HashMap::new();

    for (y, row) in grid_rows.iter().enumerate() {
        let trees: Vec<char> = row.chars().collect();

        for (x, tree) in trees.iter().enumerate() {
            let height: u32 = tree.to_digit(10).unwrap();
            let tree = Tree {
                height,
                coordinates: (x, y),
                is_visible: if x == 0 || y == 0 || x == max_x - 1 || y == max_y -1 { true } else { false }
            };
            map_of_trees.insert((x, y), tree);
        }
    }

    map_of_trees
}

fn is_tree_visible(tree: &Tree, map_of_trees: &HashMap<(usize, usize), Tree>, max_y: usize, max_x: usize) -> bool {

    if tree.is_visible == true {
        return true;
    };

    let (tree_x, tree_y) = tree.coordinates;
    let tree_height = tree.height;

    let mut is_visible: bool = false;

    for x in 0..tree_x {
        // println!("({x}, {tree_y})");
        let current_tree = map_of_trees.get(&(x, tree_y)).unwrap();
        if current_tree.height < tree_height { 
            is_visible = true; 
        } else { 
            is_visible = false;
            break;
        };
        // println!("{is_visible}");
    }

    if is_visible == true { return true; };

    for x in tree_x + 1..max_x {
        // println!("({x}, {tree_y})");
        let current_tree = map_of_trees.get(&(x, tree_y)).unwrap();
        if current_tree.height < tree_height { 
            is_visible = true; 
        } else { 
            is_visible = false;
            break;
        };
        // println!("{is_visible}");
    }

    if is_visible == true { return true; };

    for y in 0..tree_y {
        // println!("({tree_x}, {y})");
        let current_tree = map_of_trees.get(&(tree_x, y)).unwrap();
        if current_tree.height < tree_height { 
            is_visible = true; 
        } else { 
            is_visible = false;
            break;
        };
        // println!("{is_visible}");
    }

    if is_visible == true { return true; };

    for y in tree_y + 1..max_y {
        // println!("({tree_x}, {y})");
        let current_tree = map_of_trees.get(&(tree_x, y)).unwrap();
        if current_tree.height < tree_height { 
            is_visible = true; 
        } else { 
            is_visible = false;
            break;
        };
        // println!("{is_visible}");
    }

    return is_visible;

}

fn main() {
    let path = "./input.txt";

    let contents = fs::read_to_string(path).expect("FAILURE");

    let grid_rows: Vec<String> = contents.split("\n").map(|x| String::from(x)).collect();

    let max_y = grid_rows.len();
    let max_x = &grid_rows[0].len();

    let map_of_trees = get_map_of_trees(grid_rows, *max_x, max_y);

    // println!("Map of trees: {:#?}", map_of_trees);

    let mut num_visible = 0;

    for tree in map_of_trees.values() {
        let is_visible = is_tree_visible(tree, &map_of_trees, max_y, *max_x);

        if is_visible {
            num_visible += 1;
        }
    }

    println!("Number of trees visible: {num_visible}");
    
}
