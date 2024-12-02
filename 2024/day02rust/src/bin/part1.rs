fn main() {
    let input = include_str!("input.txt").to_string();
    println!("{}", input);

    let reports: Vec<Vec<i16>> = input
        .split_terminator("\n")
        .map(|x| {
            let vec: Vec<i16> = x.split(" ").map(|x| x.parse::<i16>().unwrap()).collect();
            return vec;
        })
        .collect();

    let mut num_safe = 0;

    reports.iter().for_each(|report| {
        let is_safe = is_safe(report);
        if is_safe {
            num_safe += 1;
        }
        println!("{:?} is {is_safe}", report);
    });

    println!("{}", num_safe);
}

#[derive(PartialEq)]
enum Difference {
    Positive,
    Negative,
}

fn is_safe(report: &Vec<i16>) -> bool {
    // either all increasing or all decreasing
    // difference between each should always be either negative or positive

    // change should be at least 1 and at most 3
    // difference between each should be within this range

    let mut is_safe = true;
    let mut prev_difference: Option<Difference> = None;

    for (id, val) in report.iter().enumerate() {
        let next = report.get(id + 1);
        if let Some(next_val) = next {
            // println!("val: {val}, next: {next_val}");
            let difference = next_val - val;
            // println!("{difference}");
            if difference < 0 && prev_difference == Some(Difference::Positive) {
                // println!("break condition 1");
                is_safe = false;
                break;
            }
            if difference > 0 && prev_difference == Some(Difference::Negative) {
                // println!("break condition 2");
                is_safe = false;
                break;
            }

            let abs_difference = difference.abs();
            if abs_difference < 1 || abs_difference > 3 {
                // println!("break condition 3");
                // not safe, we bail
                is_safe = false;
                break;
            }

            if prev_difference == None {
                if difference > 0 {
                    prev_difference = Some(Difference::Positive);
                } else {
                    prev_difference = Some(Difference::Negative)
                }
            }
        }
    }

    return is_safe;
}
