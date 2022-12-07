use crate::parser;

// number of assignments that fully contain the other
pub fn part_01(data: &str) -> u32 {
    data.lines()
        .map(parser::parse)
        .map(|x| x.is_fully_encompassing() as u32)
        .sum()
}
