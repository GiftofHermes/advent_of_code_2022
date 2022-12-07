use crate::group;
use crate::group::Group;
use crate::rucksack;


pub fn part_02(data: &str) -> u32 { 
    let mut points:Vec<u32> = vec![];

    for chunk in data.lines().collect::<Vec<&str>>().chunks(3) { 
        let line1: &str = chunk[0];
        let line2: &str = chunk[1]; 
        let line3: &str = chunk[2];
        
        dbg![line1, line2, line3];

        let elf_group = Group::new(line1, line2, line3);

        let badge = elf_group.find_badge().unwrap(); // puzzle requires to be at least one badge for every group 
        let point = rucksack::char_to_point(badge);
        points.push(point)
    }

    points.iter().sum()
}