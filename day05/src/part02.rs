use crate::parser;

pub fn part_02(data: &str) -> String {
    let mut crane = parser::parse(data);
    crane.apply_instructions_upgraded();
    crane.highest_crates()
}
