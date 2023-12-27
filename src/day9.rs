mod toberenamed;

use std::fs;
use crate::day9::toberenamed::Head;

pub fn day9() {
    let content: String = fs::read_to_string("input-aoc-2022-9.txt").unwrap();
    let tmp: &str = content.as_str();
    let commands: Vec<(&str, u32)> = tmp.split("\n")
        .filter(|&x| !x.is_empty())
        .map(|x| {
            let command_raw = x.split(" ").collect::<Vec<&str>>();
            return (command_raw[0], command_raw[1].parse::<u32>().unwrap());
        })
        .collect();

    let mut head = Head::new();
    for (direction, count) in commands {
        head.change_position(direction, count);
    }
}