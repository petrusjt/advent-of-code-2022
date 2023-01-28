use std::fs;

use crate::day2::rock_paper_scissors::RockPaperScissors;

mod rock_paper_scissors;

pub fn day2() {
    let content = fs::read_to_string("input-aoc-2022-2.txt")
        .expect("File should be readable");
    let result: Vec<i32> = content.split_terminator("\n")
        .map(|line: &str| {
            let mut iter = line.splitn(2, " ");
            return (RockPaperScissors::get_from_string(iter.next().unwrap()),
                    RockPaperScissors::get_from_string(iter.next().unwrap()));
        })
        .map(|(x, y)| get_score_from_x_and_y(match x {
            Ok(rps) => rps,
            Err(error) => panic!("{}", error),
        }, match y {
            Ok(rps) => rps,
            Err(error) => panic!("{}", error),
        })
        )
        .collect::<Vec<i32>>();
    println!("Advent of Code 2022/2: {}", result.iter().sum::<i32>())
}

fn get_score_from_x_and_y(x: RockPaperScissors, y: RockPaperScissors) -> i32 {
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