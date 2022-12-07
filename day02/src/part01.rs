// The winner of the whole tournament is the player with the highest score.
// Your total score is the sum of your scores for each round.
// The score for a single round is the score for the shape you selected
// (1 for Rock, 2 for Paper, and 3 for Scissors)
// plus the score for the outcome of the round
// (0 if you lost, 3 if the round was a draw, and 6 if you won).

// input.txt = 8 + 1 + 6 = 15 points

use crate::actions::Action;
use crate::points;

fn string_to_action(action: &str) -> Action {
    match action {
        "A" | "X" => Action::Rock,
        "B" | "Y" => Action::Paper,
        "C" | "Z" => Action::Scissors,
        _ => unreachable!(),
    }
}

fn convert_to_point(line: &str) -> u32 {
    let actions: Vec<Action> = line
        .split(' ')
        .map(|action| string_to_action(action))
        .collect();
    let enemy_action = actions[0];
    let action = actions[1];

    points::play_points(action) + points::game_points(action, enemy_action)
}

pub fn part_01(data: &str) -> u32 {
    data.split("\n")
        .map(|line| convert_to_point(line))
        .sum::<u32>()
}
