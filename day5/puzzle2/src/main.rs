use std::fs;
use std::collections::HashMap;

fn get_stack_collection(lines: Vec<String>, num_stacks: usize, max_size: usize) -> HashMap<usize, Vec<char>> {

    let mut stack_collection: HashMap<usize, Vec<char>> = HashMap::new();

    for nth_stack in 0..num_stacks {
        let mut stack = Vec::new();
        let mut line_number: usize = max_size - 1;

        loop {
            let item_index = (4 * nth_stack) + 1;
            let char_at_index = lines[line_number].chars().nth(item_index).unwrap();
            if char_at_index != ' ' {
                stack.push(char_at_index);
            }
            if line_number == 0 {
                break;
            } else {
                line_number -= 1;
            }
        }

        stack_collection.insert(nth_stack + 1, stack);
    }

    stack_collection

}

fn process_instruction(instruction: &String, mut stack_collection: HashMap<usize, Vec<char>>) -> HashMap<usize, Vec<char>> {
    let instruction_array: Vec<&str> = instruction.split(" ").collect();

    let num_to_move: usize = instruction_array[1].parse().unwrap();
    let from: usize = instruction_array[3].parse().unwrap();
    let to: usize = instruction_array[5].parse().unwrap();
    
    let from_stack = stack_collection.get_mut(&from).unwrap();
    let mut items_to_move: Vec<char> = from_stack.drain(from_stack.len() - num_to_move..).collect();

    let to_stack = stack_collection.get_mut(&to).unwrap();
    to_stack.append(&mut items_to_move);

    stack_collection
}

fn get_top_items(stack_collection: HashMap<usize, Vec<char>>, num_stacks: usize) -> String {
    let mut top_items = String::new();
    for key in 0..num_stacks {
        let stack = stack_collection.get(&(key + 1)).unwrap();
        let top_element = stack.last().unwrap();
        top_items.push(*top_element);
    }

    top_items
}

fn main() {
    let path = "./input.txt";
    let num_stacks = 9;
    let max_size = 8;
    let contents = fs::read_to_string(path).expect("FAILED TO READ");
    let lines: Vec<String> = contents.split("\n").map(|x| String::from(x)).collect();
    
    let mut stack_collection = get_stack_collection(lines.clone(), num_stacks, max_size);

    let instructions = &lines[max_size + 2..];

    for instruction in instructions {
        stack_collection = process_instruction(instruction, stack_collection);
    }

    let top_items = get_top_items(stack_collection, num_stacks);
    
    println!("{top_items}");
}

