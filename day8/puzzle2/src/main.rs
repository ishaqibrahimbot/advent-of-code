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

fn get_scenic_score(tree: &Tree, map_of_trees: &HashMap<(usize, usize), Tree>, max_y: usize, max_x: usize) -> u32 {

    let (tree_x, tree_y) = tree.coordinates;

    if tree_x == 0 || tree_x == max_x - 1 || tree_y == 0 || tree_y == max_y - 1 {
        return 0;
    }

    let tree_height = tree.height;

    let mut scenic_score = 1;
    let mut num_visible_trees = 0;

    let mut left_index = tree_x - 1;
    loop {
        let current_tree = map_of_trees.get(&(left_index, tree_y)).unwrap();
        if current_tree.height < tree_height { 
            num_visible_trees += 1;
        } else { 
            num_visible_trees += 1;
            break;
        };

        if left_index == 0 {
            break;
        }

        left_index -= 1;
    }

    scenic_score *= num_visible_trees;
    num_visible_trees = 0;

    let mut right_index = tree_x + 1;
    while right_index < max_x {
        let current_tree = map_of_trees.get(&(right_index, tree_y)).unwrap();
        if current_tree.height < tree_height { 
            num_visible_trees += 1;
        } else { 
            num_visible_trees += 1;
            break;
        };

        right_index += 1;
    }

    scenic_score *= num_visible_trees;
    num_visible_trees = 0;

    let mut up_index = tree_y - 1;
    loop {
        let current_tree = map_of_trees.get(&(tree_x, up_index)).unwrap();
        if current_tree.height < tree_height { 
            num_visible_trees += 1;
        } else { 
            num_visible_trees += 1;
            break;
        };

        if up_index == 0 {
            break;
        }

        up_index -= 1;
    }

    scenic_score *= num_visible_trees;
    num_visible_trees = 0;

    let mut down_index = tree_y + 1;
    while down_index < max_y {
        let current_tree = map_of_trees.get(&(tree_x, down_index)).unwrap();
        if current_tree.height < tree_height { 
            num_visible_trees += 1;
        } else { 
            num_visible_trees += 1;
            break;
        };

        down_index += 1;
    }

    scenic_score *= num_visible_trees;

    scenic_score

}

fn main() {
    let path = "./input.txt";

    let contents = fs::read_to_string(path).expect("FAILURE");

    let grid_rows: Vec<String> = contents.split("\n").map(|x| String::from(x)).collect();

    let max_y = grid_rows.len();
    let max_x = &grid_rows[0].len();

    let map_of_trees = get_map_of_trees(grid_rows, *max_x, max_y);

    // println!("Map of trees: {:#?}", map_of_trees);

    let mut scenic_scores: Vec<u32> = Vec::new();

    for tree in map_of_trees.values() {
        let scenic_score = get_scenic_score(tree, &map_of_trees, max_y, *max_x);

        scenic_scores.push(scenic_score);
       
    }

    scenic_scores.sort();


    println!("Highest scenic score: {:#?}", scenic_scores.last().unwrap());
    
}
