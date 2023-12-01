use std::{fs, collections::VecDeque};

#[derive(Debug)]
struct Instruction {
    increment_by: i32,
    num_cycles: u32,
    counter: u32
}

impl Instruction {
    fn modify_signal(&mut self, current_value: i32) -> (bool, i32) {
        let can_modify = self.can_modify();

        if can_modify {
            let new_x = self.add(current_value);
            (true, new_x)
        } else {
            (false, current_value)
        }
        
    }

    fn add(&self, current_value: i32) -> i32 {
        current_value + self.increment_by
    }

    fn can_modify(&mut self) -> bool {
        self.counter += 1;

        if self.counter == self.num_cycles {
            true
        } else {
            false
        }
    }
}

fn main() {
    let path = "./input.txt";
    let contents = fs::read_to_string(path).expect("FAILURE");

    let mut instructions: VecDeque<Instruction> = contents.split("\n").map(|x| {
        if x.starts_with("addx") {
            let parts: Vec<&str> = x.split(" ").collect();
            let increment_by = parts[1].parse().unwrap();

            Instruction {
                counter: 0,
                increment_by,
                num_cycles: 2
            }
        } else {
            Instruction {
                counter: 0,
                increment_by: 0,
                num_cycles: 1
            }
        }
    }).collect();

    let mut signal_strengths: Vec<i32> = Vec::new();
    let mut current_instruction = instructions.pop_front().unwrap();
    let mut x = 1;
    let mut signal_modified: bool;

    for cycle in 1..21 {
        if cycle == 20 {
            signal_strengths.push(x * cycle);
        }

        (signal_modified, x) = current_instruction.modify_signal(x);

        if signal_modified {
            current_instruction = instructions.pop_front().unwrap();
        }

    }

    for cycle in 1..201 {
        if cycle % 40 == 0 {
            signal_strengths.push((cycle + 20) * x);
        }

        (signal_modified, x) = current_instruction.modify_signal(x);

        if signal_modified {
            current_instruction = instructions.pop_front().unwrap();
        }
    }

    println!("Signal strengths: {:#?}", signal_strengths);

    let sum: i32 = signal_strengths.iter().sum();

    println!("Total strength: {sum}");

}
