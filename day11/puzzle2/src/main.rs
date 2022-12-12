use std::{fs, collections::{VecDeque, HashMap}};

struct Monkey {
    items: VecDeque<usize>,
    transform: Box<dyn Fn(usize) -> usize>,
    test_condition: usize,
    throws_to: (usize, usize),
    num_inspections: usize
}

impl Monkey {
    fn print_monkey(&self) {
        println!("Items: {:?}", self.items);
        println!("Test condition: {}", self.test_condition);
        println!("Throws to: if true -> {}, if false -> {}", self.throws_to.0, self.throws_to.1);
        println!("Transform -> 2 becomes {}", (self.transform)(2));
        println!("Num inspections: {}", self.num_inspections);
    }

    fn throw(&mut self, common_multiple: usize) -> Option<(usize, usize)> {
        
        loop {
            if self.items.len() == 0 {
                break;
            }

            let mut current_item: usize = self.items.pop_front().unwrap();

            current_item = (self.transform)(current_item);
            current_item = current_item % common_multiple;
            
            self.num_inspections += 1;
            
            if current_item % self.test_condition == 0 {
                let throws_to = self.throws_to.0;
                return Some((throws_to, current_item));
            } else {
                let throws_to = self.throws_to.1;
                return Some((throws_to, current_item));
            }
        }

        None
    }

    fn catch(&mut self, item: usize) {
        self.items.push_back(item);
    }
}

struct MonkeyGroup {
    num_monkeys: usize,
    monkeys: HashMap<usize, Monkey>
}

impl MonkeyGroup {
    fn simulate_round(&mut self, common_multiple: usize) {
        let mut i = 0;

        loop {
            let mut series_of_instructions: Vec<(usize, usize)> = Vec::new();
            loop {
                let current_monkey = self.monkeys.get_mut(&i).unwrap();
    
                match current_monkey.throw(common_multiple) {
                    Some((to, item)) => {
                        series_of_instructions.push((to, item))
                    },
                    None => {
                        break;
                    }
                }
    
            }
    
            for instruction in series_of_instructions {
                let (to, item) = instruction;
                let receiving_monkey = self.monkeys.get_mut(&to).unwrap();
                receiving_monkey.catch(item);
            }

            i += 1;

            if i == self.num_monkeys {
                break;
            }
        }
        
    }
}

fn get_transform(operation: Vec<&str>) -> Box<dyn Fn(usize) -> usize> {
    let is_last_number = !operation.last().unwrap().contains("old");

    if is_last_number {
        let last_number: usize = operation.last().unwrap().parse().unwrap();

        if operation[1] == "*" {
            return Box::new(move |x| x * last_number);
        } else {
            return Box::new(move |x| x + last_number);
        }

    } else {

        if operation[1] == "*" {
            return Box::new(|x| x * x );
        } else {
            return Box::new(|x| x + x);
        }

    }
}

fn main() {
    let path = "./input.txt";
    let contents = fs::read_to_string(path).expect("FAILURE");

    let mut common_multiple = 1;
    let mut monkeys: Vec<Monkey> = contents.split("\n\n").map(|x| {
        let mut monkey_details: Vec<&str> = x.split("\n").collect();

        let items: VecDeque<usize> = monkey_details[1].split(": ").last().unwrap().split(", ").map(|x| {
            let num: usize = x.parse().unwrap();
            num
        }).collect();

        let test_condition: usize = monkey_details[3].split(" ").last().unwrap().parse().unwrap();

        common_multiple *= test_condition;

        let true_throw_to: usize = monkey_details[4].split(" ").last().unwrap().parse().unwrap();
        let false_throw_to: usize = monkey_details[5].split(" ").last().unwrap().parse().unwrap();

        let operation: Vec<&str> = monkey_details[2].split(" = ").last().unwrap().split(" ").collect();

        let transform = get_transform(operation);

        Monkey {
            items,
            test_condition,
            throws_to: (true_throw_to, false_throw_to),
            transform,
            num_inspections: 0
        }

    }).collect();

    let num_monkeys = monkeys.len();

    let mut monkey_map: HashMap<usize, Monkey> = monkeys.into_iter().enumerate().collect();

    let mut monkey_game = MonkeyGroup {
        monkeys: monkey_map,
        num_monkeys
    };

    println!("Common multiple: {common_multiple}");

    for round in 0..10_000 {
        monkey_game.simulate_round(common_multiple);
    }
    
    
    // for (idx, monkey) in monkey_game.monkeys {
    //     println!("Monkey no: {idx}");
    //     monkey.print_monkey();
    //     println!("");
    // }

    let mut inspections: Vec<usize> = monkey_game.monkeys.values().map(|x| x.num_inspections).collect();

    inspections.sort();

    let top = inspections.pop().unwrap();
    let second = inspections.pop().unwrap();

    println!("{} * {} = {}", top, second, top * second);

}