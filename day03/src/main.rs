use std::fs;
mod part01;
mod part02;
mod rucksack;
mod group;


fn main() {
    let data = fs::read_to_string("data/input_test.txt").expect("File is manually given");

    let points: u32 = part02::part_02(&data);
    //println!("{data}");
    dbg!(points);
}
