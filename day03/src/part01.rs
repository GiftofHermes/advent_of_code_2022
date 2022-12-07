// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.

// Find the item type that appears in both compartments of each rucksack.
// What is the sum of the priorities of those item types?
use crate::rucksack;
use crate::rucksack::Rucksack;

pub fn part_01(data: &str) -> u32 {
    let mut points: Vec<u32> = vec![];

    for line in data.lines() {
        let bag = Rucksack::new(line);

        let point = bag
            .shared_items()
            .iter()
            .map(|x| rucksack::char_to_point(*x))
            .sum::<u32>();
        dbg![bag.shared_items(), point];
        points.push(point);
    }

    points.iter().sum()
}
