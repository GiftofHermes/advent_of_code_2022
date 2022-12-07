use crate::actions::Action;

pub fn game_points(action: Action, enemy_action: Action) -> u32 {
    match (action, enemy_action) {
        (action, enemy_action) if action == enemy_action => 3,
        (Action::Rock, Action::Paper)
        | (Action::Paper, Action::Scissors)
        | (Action::Scissors, Action::Rock) => 0,
        (Action::Rock, Action::Scissors)
        | (Action::Paper, Action::Rock)
        | (Action::Scissors, Action::Paper) => 6,
        (_, _) => unreachable!(),
    }
}

pub fn play_points(action: Action) -> u32 {
    match action {
        Action::Rock => 1,
        Action::Paper => 2,
        Action::Scissors => 3,
    }
}
