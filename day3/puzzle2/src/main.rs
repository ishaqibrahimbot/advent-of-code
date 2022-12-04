extern crate array_tool;
use std::fs;
use array_tool::vec::Intersect;


fn get_priority(item: char) -> u32 {
    // UTF-8 decimal
    // A 65 --> Z 90
    // a 97 --> z 122

    // game priority
    // A 27 --> Z 52
    // a 1 --> z 26

    // if code is uppercase --> code - 38
    // else code - 96

    let code : u32 = item as u32;

    if item.is_lowercase() {
        code - 96
    } else {
        code - 38
    }
}

fn groups_of_three(sacks: Vec<&str>) -> Vec<Vec<Vec<char>>> {
    let mut final_vec: Vec<Vec<Vec<char>>> = Vec::new();

    let mut group_of_three: Vec<Vec<char>> = Vec::new();
    for i in 1..(sacks.len() + 1) {
        let sack = sacks[i-1];
        let chars_array: Vec<char> = sack.chars().collect();
        group_of_three.push(chars_array);

        if i % 3 == 0 {
            final_vec.push(group_of_three);
            group_of_three = Vec::new();
        }
    }

    final_vec
}

fn main() {
    let input_path = "./input.txt";

    let contents = fs::read_to_string(input_path).expect("YOU FAILURE");

    let rucksacks: Vec<&str> = contents.split("\n").collect();
    let num_rucksacks = rucksacks.len();
    println!("Num of rucksacks: {num_rucksacks}");
    println!("Num of groups expected: {}", num_rucksacks/3);
    
    let groups_of_three = groups_of_three(rucksacks);
    println!("Num of groups created: {}", groups_of_three.len());

    let mut total_priority_score: u32 = 0;

    for group in groups_of_three {
        let badge = group[0].intersect(group[1].clone()).intersect(group[2].clone())[0];

        let priority = get_priority(badge);
        total_priority_score += priority;


        println!("Badge: {badge}, priority: {priority}");
    }

    println!("Total priority score: {total_priority_score}", );

}

