use crate::parser;

// number of pairs that overlap at all.
pub fn part_02(data: &str) -> u32 {
    data.lines()
        .map(parser::parse)
        .map(|x| x.is_overlapping() as u32)
        .sum()
}
