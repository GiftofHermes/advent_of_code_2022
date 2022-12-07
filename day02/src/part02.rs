
// X means you need to lose
// Y means you need to end the round in a draw 
// Z means you need to win
use crate::actions::Action;
use crate::points;


#[derive(Copy, Clone)]
enum State {
    Lose,
    Draw,
    Win
}

fn string_to_action(action: &str) -> Action { 
    match action { 
        "A" => Action::Rock,
        "B" => Action::Paper,
        "C" => Action::Scissors,
        _ => unreachable!(),
    }
}

fn string_to_state(state: &str) -> State { 
    match state { 
        "X" => State::Lose,
        "Y" => State::Draw,
        "Z" => State::Win,
        _ => unreachable!(),
    }
}

fn game_state_to_action(enemy_action: Action, state: State) -> Action { 
    match (enemy_action, state) { 
        (Action::Rock, State::Lose) => Action::Scissors,
        (Action::Rock, State::Draw) => Action::Rock,
        (Action::Rock, State::Win) => Action::Paper,
        (Action::Paper, State::Lose) => Action::Rock,
        (Action::Paper, State::Draw) => Action::Paper,
        (Action::Paper, State::Win) => Action::Scissors,
        (Action::Scissors, State::Lose) => Action::Paper,
        (Action::Scissors, State::Draw) => Action::Scissors,
        (Action::Scissors, State::Win) => Action::Rock,
    }
}

fn convert_to_point(line: &str) -> u32 {
    let action_state: Vec<&str> = line.split(' ').collect();
    let enemy_action = string_to_action(action_state[0]);
    let state = string_to_state(action_state[1]);
    let action = game_state_to_action(enemy_action, state);

    points::play_points(action) + points::game_points(action, enemy_action)
}

pub fn part_02(data: &str) -> u32 {
    data
        .split("\n")
        .map(|line| convert_to_point(line))
        .sum::<u32>()
}