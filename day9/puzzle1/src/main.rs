use std::fs;
use std::collections::HashMap;

struct RopeEnd {
    x: i32,
    y: i32,
}

fn get_next_head_coordinate(direction: &String, x: i32, y: i32) -> (i32, i32) {
    if direction == "R" {
        (x + 1, y)
    } else if direction == "L" {
        (x - 1, y)
    } else if direction == "U" {
        (x, y - 1)
    } else {
        (x, y + 1)
    }
}

fn is_touching(head_coords: (i32, i32), tail_coords: (i32, i32)) -> bool {
    let (head_x, head_y) = head_coords;
    let (tail_x, tail_y) = tail_coords;

    if (tail_x - head_x).abs() > 1 || (tail_y - head_y).abs() > 1 {
        return false;
    }

    true
}

fn get_next_tail_coordinate(head_coords: (i32, i32), tail_coords: (i32, i32)) -> (i32, i32) {

    let (head_x, head_y) = head_coords;
    let (tail_x, tail_y) = tail_coords;

    let is_touching = is_touching(head_coords, tail_coords);

    if is_touching {
        return tail_coords;
    }

    let mut new_tail_x = tail_x;
    let mut new_tail_y = tail_y;

    if head_x < tail_x {
        new_tail_x -= 1;
    } else if head_x > tail_x {
        new_tail_x += 1;
    }

    if head_y < tail_y {
        new_tail_y -= 1;
    } else if head_y > tail_y {
        new_tail_y += 1;
    }

    (new_tail_x, new_tail_y)
}

fn process_instruction(head: &mut RopeEnd, tail: &mut RopeEnd, instruction: (String, i32), points_visited: &mut HashMap<(i32, i32), bool>) {
    let (direction, steps) = instruction;


    for step in 0..steps {
        let (new_head_x, new_head_y) = get_next_head_coordinate(&direction, head.x, head.y);
        head.x = new_head_x;
        head.y = new_head_y;

        let (new_tail_x, new_tail_y) = get_next_tail_coordinate((head.x ,head.y), (tail.x, tail.y));
        points_visited.insert((new_tail_x, new_tail_y), true);
        tail.x = new_tail_x;
        tail.y = new_tail_y;
    }

}

fn main() {
    let path = "./input.txt";
    let contents = fs::read_to_string(path).expect("FAILURE");

    let _instructions: Vec<String> = contents.split("\n").map(|x| String::from(x)).collect();

    let instructions: Vec<(String, i32)> = _instructions.iter().map(|x| {
        let parts: Vec<&str> = x.split(" ").collect();
        let direction = parts[0];
        let steps: i32 = parts[1].parse().unwrap();
        (String::from(direction), steps)
    }).collect();

    let mut points_visited: HashMap<(i32, i32), bool> = HashMap::new();

    let mut head = RopeEnd {
        x: 0,
        y: 0
    };

    let mut tail = RopeEnd {
        x: 0,
        y: 0
    };

    for instruction in instructions {
        process_instruction(&mut head, &mut tail, instruction, &mut points_visited);
    }


    println!("Num positions visited by tail: {}", points_visited.keys().len());


}
