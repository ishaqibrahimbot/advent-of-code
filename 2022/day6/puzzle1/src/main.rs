use std::fs;

fn find_first_marker_position(signal: &String) -> usize {

    let signal_vec: Vec<char> = signal.chars().collect();
    let window_size = 4;

    let mut position = 0;
    loop {
        let mut signal_portion = signal_vec[position..window_size+position].to_vec();
        signal_portion.sort();
        signal_portion.dedup();
        if signal_portion.len() == window_size {
            return window_size + position;
        }
        position += 1;
    }

}

fn main() {
    let path = "./input.txt";
    let contents = fs::read_to_string(path).expect("FAILURE YOU ARE");

    let samples: Vec<String> = contents.split("\n").map(|x| String::from(x)).collect();

    for sample in samples {
        let first_marker = find_first_marker_position(&sample);
        println!("First marker for {sample} is at position {first_marker}");
    }
}

