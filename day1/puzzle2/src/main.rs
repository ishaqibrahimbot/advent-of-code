use std::fs;

fn main() {
    let file_path = "./input.txt";

    let contents = fs::read_to_string(file_path).expect("Could not read file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut elf_totals_vec: Vec<u32> = Vec::new();

    let mut highest_elf_total = 0;
    let mut elf_total = 0;

    for line in &lines {

        if line.len() > 0 {
            let number : u32 = line.parse().unwrap();
            elf_total += number;
        } else {
            println!("Elf total: {elf_total}\nHighest total: {highest_elf_total}\n");
            if elf_total >= highest_elf_total {
                highest_elf_total = elf_total;
            }
            elf_totals_vec.push(elf_total);
            elf_total = 0;
        }
    }

    elf_totals_vec.sort();

    let length_of_totals_vec = elf_totals_vec.len();

    let third_last = elf_totals_vec[length_of_totals_vec - 3];
    let second_last = elf_totals_vec[length_of_totals_vec - 2];
    let last = elf_totals_vec[length_of_totals_vec - 1];

    let total_of_last_three = third_last + second_last + last;

    println!("Highest Elf total of all time: {highest_elf_total}");
    println!("{:?}", elf_totals_vec);
    println!("3rd last: {third_last}, 2nd last: {second_last}, last: {last}");
    println!("Total of top 3: {total_of_last_three}");
}
