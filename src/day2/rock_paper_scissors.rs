use crate::day2::rock_paper_scissors::RockPaperScissors::{PAPER, ROCK, SCISSORS};

#[derive(PartialEq)]
pub(crate) enum RockPaperScissors {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}

impl RockPaperScissors {
    pub fn get_from_string(s: &str) -> Result<RockPaperScissors, &'static str> {
        if (*s).eq("A") || (*s).eq("X") {
            return Ok(ROCK);
        }
        if (*s).eq("B") || (*s).eq("Y") {
            return Ok(PAPER);
        }
        if (*s).eq("C") || (*s).eq("Z") {
            return Ok(SCISSORS);
        }
        return Err("Invalid type");
    }
}