use std::fs;

use crate::day2::rock_paper_scissors::RockPaperScissors;
use crate::day2::util::get_score_from_x_and_y;

mod rock_paper_scissors;
mod util;

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