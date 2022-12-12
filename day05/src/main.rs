use std::fs;
mod crane;
mod parser;
mod part01;
mod part02;

fn main() {
    let data = fs::read_to_string("data/input_test.txt").expect("File is manually given");
    let solution1: String = part01::part_01(&data);
    let solution2: String = part02::part_02(&data);
    //println!("{data}");
    dbg!(solution1, solution2);
}
