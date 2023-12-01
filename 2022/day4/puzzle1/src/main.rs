extern crate array_tool;
use array_tool::vec::Intersect;
use std::fs;

fn main() {
    let input_path = "./input.txt";
    let contents = fs::read_to_string(input_path).expect("FAILED TO READ");

    let range_pairs: Vec<String> = contents.split("\n").map(|str| String::from(str)).collect();

    let mut num_overlaps: u32 = 0;

    for range_pair in range_pairs {
        println!("Range pair: {range_pair}");
        let ranges: Vec<String> = range_pair.split(",").map(|str| String::from(str)).collect();

        let vec_1 = range_to_vec(&ranges[0]);
        let vec_2 = range_to_vec(&ranges[1]);

        let overlap_vec = vec_1.intersect(vec_2.clone());

        println!("Intersection: {:?}", overlap_vec);

        if overlap_vec.len() == vec_1.len() || overlap_vec.len() == vec_2.len() {
            println!("Found an overlap!");
            num_overlaps += 1;
        }
        println!("");
    }

    println!("Number of overlaps: {num_overlaps}");

}

fn range_to_vec(range: &String) -> Vec<u32> {
    let bounds: Vec<u32> = range.split("-").map(|x| x.parse().unwrap()).collect();
    let vec : Vec<u32> = (bounds[0]..(bounds[1] + 1)).collect();
    vec
}
