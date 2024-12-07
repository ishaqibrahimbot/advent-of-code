use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    let input = include_str!("input.txt").to_string();
    let lines = input.split("\n").collect::<VecDeque<&str>>();
    let mut numbers_map: HashMap<i64, VecDeque<i64>> = HashMap::new();
    for line in lines {
        let (left, right) = line.split_once(": ").unwrap();
        let left = left.parse::<i64>().unwrap();
        let right = right
            .split(" ")
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<VecDeque<i64>>();
        numbers_map.insert(left, right);
    }

    let mut sum = 0;

    for (result, numbers) in numbers_map {
        if can_be_made_valid(result, &numbers) {
            sum += result;
        }
    }

    println!("{:?}", sum);
}

fn can_be_made_valid(result: i64, numbers: &VecDeque<i64>) -> bool {
    let mut numbers_clone = numbers.clone();
    let first_num = numbers_clone.pop_front().unwrap();
    reduce_recursively(result, &mut numbers_clone, first_num)
}

fn reduce_recursively(result: i64, numbers: &mut VecDeque<i64>, current_result: i64) -> bool {
    if numbers.len() == 0 {
        return current_result == result;
    }

    let next_num = numbers.pop_front().unwrap();
    let option_one = current_result + next_num;
    let option_two = current_result * next_num;
    let option_three = format!("{}{}", current_result, next_num)
        .parse::<i64>()
        .unwrap();

    reduce_recursively(result, &mut numbers.clone(), option_one)
        || reduce_recursively(result, &mut numbers.clone(), option_two)
        || reduce_recursively(result, &mut numbers.clone(), option_three)
}
