extern crate serde_json;
use serde_json::{Value, json};
use std::fs;
use std::collections::HashMap;

fn get_nested_array(chars: Vec<char>, mut index: u32) -> (Value, u32) {

    let mut this_vec: Vec<Value> = vec![];

    // only push when you encounter a comma
    // until then add characters onto a single str

    let mut digit_str = String::new();
    loop {
        let char = chars.clone().into_iter().nth(index as usize);

        if let Some(valid_char) = char {
            if valid_char.is_ascii_digit() {
                digit_str.push(valid_char);
            } else if valid_char == '[' {
                if index == 0 {
                    index += 1;
                    continue;
                } else {
                    let nested_value: Value;
                    let chars_copy = chars.clone();
                    (nested_value, index) = get_nested_array(chars_copy, index + 1);
                    this_vec.push(nested_value);
                }
            } else if valid_char == ']' {
                match digit_str.parse::<u32>() {
                    Ok(num) => {
                        this_vec.push(json!(num));
                    }
                    Err(_) => {}
                }
                return (json!(this_vec), index);
            } else if valid_char == ',' {
                match digit_str.parse::<u32>() {
                    Ok(num) => {
                        this_vec.push(json!(num));
                        digit_str = String::new();
                    },
                    Err(_) => {}
                    }
                }
        } else {
            return (json!(this_vec), index);
        }

        index += 1;
    }
}


fn check_order(value_left: Value, value_right: Value) -> (bool, bool) {

    let mut decided = false;
    let mut ordered = false;

    let mut index = 0;

    loop {
        let value_left_first = value_left.get(index);
        let value_right_first = value_right.get(index);

        // println!("INDEX: {index}");
        // println!("value_left_first: {:?}", value_left_first);
        // println!("value_right_first: {:?}", value_right_first);

        // if both have no value, we just give back control to parent function
        // with decided = false

        if let None = value_left_first {
            if let None = value_right_first {
                return (false, false);
            }
        }

        if let Some(value_left_first) = value_left_first {
            if let Some(value_right_first) = value_right_first {
                if value_left_first.is_number() && value_right_first.is_number() {
                    if value_left_first.as_u64().unwrap() < value_right_first.as_u64().unwrap() {
                        decided = true;
                        ordered = true;
                        break;
                    } else if value_left_first.as_u64().unwrap() > value_right_first.as_u64().unwrap() {
                        decided = true;
                        ordered = false;
                        break;
                    } else {
                        index += 1;
                        continue;
                    }
        
                } else if value_left_first.is_array() && value_right_first.is_number() {
                    (decided, ordered) = check_order(value_left_first.clone(), json!([value_right_first.as_u64().unwrap()]));
                    if decided {
                        break;
                    }
        
                } else if value_left_first.is_number() && value_right_first.is_array() {
                    (decided, ordered) = check_order(json!([value_left_first.as_u64().unwrap()]), value_right_first.clone());
                    if decided {
                        break;
                    }
        
                } else if value_left_first.is_array() && value_right_first.is_array() {
                    (decided, ordered) = check_order(value_left_first.clone(), value_right_first.clone());
                    if decided {
                        break;
                    }
                }
            } else {
                decided = true;
                ordered = false;
                break;
            }

        } else {
            decided = true;
            ordered = true;
            break;
        }

        index += 1;

    }

    return (decided, ordered);
}

fn main() {
    let path = "./input.txt";
    let contents = fs::read_to_string(path).expect("YOU FAILURE");

    let mut map: HashMap<u32, (Value, Value)> = HashMap::new();

    for (index, pairs) in contents.split("\n\n").enumerate() {
        let mut value_1: Value = json!(0);
        let mut value_2: Value = json!(0);

        for (num, string) in pairs.split("\n").enumerate() {
            let chars: Vec<char> = string.chars().collect();

            let (value, final_index) = get_nested_array(chars, 0);

            if num == 0 {
                value_1 = value;
            } else {
                value_2 = value;
            }
            
        }

        map.insert((index + 1) as u32, (value_1, value_2));
    }

    let mut total = 0;

    for (index, pair) in map {
        println!("\nINDEX: {index}, PAIR: {:?}", pair);
        let (value_left, value_right) = pair.clone();
        let (decided, ordered) = check_order(value_left, value_right);

        if ordered {
            println!("ORDERED");
            total += index;
        } else {
            println!("NOT ORDERED");
        }
    }

    // let pair = map.get(&9).unwrap();
    // let (value_left, value_right) = pair.clone();

    // println!("PAIR: {:?}", pair);

    // let (decided, ordered) = check_order(value_left, value_right);

    // if ordered {
    //     println!("ORDERED");
    //     // total += index;
    // } else {
    //     println!("NOT ORDERED");
    // }


    println!("TOTAL OF ORDERED: {total}");
    
}
