use crate::parser;

pub fn part_01(data: &str) -> String {
    let mut crane = parser::parse(data);
    crane.apply_instructions();
    crane.highest_crates()
}
