use crate::parser;

// number of assignments that fully contain the other
pub fn part_02(data: &str) -> u32 {
    parser::parse_start_of_message(data)
}
