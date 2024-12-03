use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let sample = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let re = Regex::new(r"mul\((?<d1>\d{1,3}),(?<d2>\d{1,3})\)|(?<do>do\(\))|(?<dont>don't\(\))")
        .unwrap();
    let mut sum = 0;
    let mut should_multiply = true;
    let result = re.captures_iter(input).for_each(|caps| {
        let do_please = caps.name("do"); // do is a reserved keyword lol
        let dont = caps.name("dont");
        if let Some(multiply_yes) = do_please {
            should_multiply = true;
        }
        if let Some(multiply_no) = dont {
            should_multiply = false;
        }

        let digit_one = caps.name("d1");
        if let Some(digit_one) = digit_one {
            let first_number = digit_one.as_str().parse::<i32>().unwrap();
            let second_number = caps.name("d2").unwrap().as_str().parse::<i32>().unwrap();
            if should_multiply {
                let product = first_number * second_number;
                sum += product;
                println!("{first_number} * {second_number} = {product}");
            }
        }
    });

    println!("sum: {sum}")
}
