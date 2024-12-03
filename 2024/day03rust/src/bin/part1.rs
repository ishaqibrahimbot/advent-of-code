use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let re = Regex::new(r"mul\((?<d1>\d{1,3}),(?<d2>\d{1,3})\)").unwrap();
    let mut sum = 0;
    let result = re.captures_iter(input).for_each(|caps| {
        let digit_one = caps.name("d1").unwrap().as_str().parse::<i32>().unwrap();
        let digit_two = caps.name("d2").unwrap().as_str().parse::<i32>().unwrap();
        let product = digit_one * digit_two;
        sum += product;
        return println!("{digit_one} * {digit_two} = {product}");
    });

    println!("sum: {sum}")
}
