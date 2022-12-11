use std::fs;
mod crane;
mod parser;
mod part01;
mod part02;

fn main() {
    let data = fs::read_to_string("data/input.txt").expect("File is manually given");
    let solution1: u32 = part01::part_01(&data);
    let solution2: u32 = part02::part_02(&data);
    //println!("{data}");
    dbg!(solution1, solution2);
}
