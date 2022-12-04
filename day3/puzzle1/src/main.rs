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

    // if byte code <= 90 --> byte code - 38
    // else byte code - 96

    let bytes : u32 = item as u32;

    if item.is_lowercase() {
        bytes - 96
    } else {
        bytes - 38
    }
}

fn main() {
    let input_path = "./input.txt";

    let contents = fs::read_to_string(input_path).expect("YOU FAILURE");

    let rucksacks: Vec<&str> = contents.split("\n").collect();
    let num_rucksacks = rucksacks.len();

    let mut total_priority_score: u32 = 0;
    
    for sack in rucksacks {
        println!("Looking at rucksack: {sack}");
        let num_items = sack.len();
        let midpoint = num_items/2;
        let compartment_a : Vec<char> = sack[0..midpoint].chars().collect();
        let compartment_b: Vec<char> = sack[midpoint..].chars().collect();

        let misplaced_item = compartment_a.intersect(compartment_b);
        let priority = get_priority(misplaced_item[0]);
        println!("\nculprit: {}, priority: {priority}", misplaced_item[0]);
        total_priority_score += priority;

        println!("\n");
    }

    println!("Total priority score: {total_priority_score}", );

}
