use crate::day2::rock_paper_scissors::RockPaperScissors;

pub(crate) fn get_score_from_x_and_y(x: RockPaperScissors, y: RockPaperScissors) -> i32 {
    if x == y {
        return (y as i32) + 3;
    }
    if does_x_beat_y(&x, &y) {
        return y as i32;
    }

    return (y as i32) + 6;
}

fn does_x_beat_y(x: &RockPaperScissors, y: &RockPaperScissors) -> bool {
    (*x == RockPaperScissors::SCISSORS && *y == RockPaperScissors::PAPER)
        || (*x == RockPaperScissors::PAPER && *y == RockPaperScissors::ROCK)
        || (*x == RockPaperScissors::ROCK && *y == RockPaperScissors::SCISSORS)
}