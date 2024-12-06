use std::{cmp::Ordering, collections::HashSet};

fn main() {
    let input = include_str!("input.txt").to_string();
    let mut rules_set = HashSet::new();

    let both: Vec<Vec<&str>> = input
        .split("\n\n")
        .map(|x| {
            return x.split("\n").collect();
        })
        .collect();

    let rules = both.get(0).unwrap();
    rules.iter().for_each(|rule| {
        rules_set.insert(rule.to_string());
    });

    let numbers: Vec<Vec<usize>> = both
        .get(1)
        .unwrap()
        .iter()
        .map(|x| x.split(",").map(|x| x.parse::<usize>().unwrap()).collect())
        .collect();

    let mut sum = 0;

    for number in numbers {
        let mut is_valid = true;
        for i in 0..number.len() {
            if i == number.len() - 1 {
                continue;
            }

            let num = number.get(i).unwrap();
            let next_num = number.get(i + 1).unwrap();
            if !rules_set.contains(&format!("{}|{}", num, next_num)) {
                is_valid = false;
                break;
            }
        }

        if !is_valid {
            let sorted = sort_numbers_by_rules(&number, &rules_set);
            sum += get_middle_number(sorted);
        }
    }

    println!("{}", sum);
}

fn get_middle_number(number: Vec<usize>) -> usize {
    let len = number.len();
    return *number.get((len - 1) / 2).unwrap();
}

fn sort_numbers_by_rules(number: &Vec<usize>, rules: &HashSet<String>) -> Vec<usize> {
    let mut sorted = number.clone();
    sorted.sort_by(|a, b| {
        if rules.contains(&format!("{}|{}", a, b)) {
            return Ordering::Less;
        }
        if rules.contains(&format!("{}|{}", b, a)) {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    });
    return sorted;
}
