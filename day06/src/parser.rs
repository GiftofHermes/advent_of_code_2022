pub fn parse_start_of_packet(data: &str) -> u32 {
    let inter = data.chars().collect::<Vec<char>>();
    let start_indicator_length = 4;

    inter
        .windows(start_indicator_length)
        .enumerate()
        .filter(|(_i, vars)| {
            vars.iter()
                .enumerate()
                .all(|(i, x)| vars[i + 1..].iter().all(|y| x != y))
        })
        .nth(0)
        .unwrap()
        .0 as u32
        + start_indicator_length as u32
}

pub fn parse_start_of_message(data: &str) -> u32 {
    let inter = data.chars().collect::<Vec<char>>();
    let message_length = 14;
    (inter
        .windows(message_length)
        .enumerate()
        .filter(|(_i, vars)| {
            vars.iter()
                .enumerate()
                .all(|(i, x)| vars[i + 1..].iter().all(|y| x != y))
        })
        .nth(0)
        .unwrap()
        .0 as u32)
        + message_length as u32
}
