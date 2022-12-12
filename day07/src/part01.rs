use crate::parser;

// number of assignments that fully contain the other
pub fn part_01(data: &str) -> u32 {
    parser::parse_directory(data)
}
