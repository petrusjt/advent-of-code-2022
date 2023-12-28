mod utils;

use std::fs;
use crate::day9::utils::{Head, Head2};

pub fn day9() {
    let content: String = fs::read_to_string("input-aoc-2022-9.txt").unwrap();
    let tmp: &str = content.as_str();
    let commands: Vec<(&str, u32)> = tmp.split("\n")
        .filter(|&x| !x.is_empty())
        .map(|x| {
            let command_raw = x.split(" ").collect::<Vec<&str>>();
            return (command_raw[0], command_raw[1].trim().parse::<u32>().unwrap());
        })
        .collect();

    println!("Advent of Code 2022/9/1: {}", count_visited_positions(commands));
}

fn count_visited_positions(commands: Vec<(&str, u32)>) -> u32 {
    let mut head = Head::new();
    for (direction, count) in commands {
        head.change_position(direction, count);
    }
    return head.tail.visited_positions.len() as u32;
}

pub fn day9_part2() {
    let content: String = fs::read_to_string("input-aoc-2022-9.txt").unwrap();
    let tmp: &str = content.as_str();
    let commands: Vec<(&str, u32)> = tmp.split("\n")
        .filter(|&x| !x.is_empty())
        .map(|x| {
            let command_raw = x.split(" ").collect::<Vec<&str>>();
            return (command_raw[0], command_raw[1].trim().parse::<u32>().unwrap());
        })
        .collect();

    println!("Advent of Code 2022/9/2: {}", count_visited_positions2(commands));
}

fn count_visited_positions2(commands: Vec<(&str, u32)>) -> u32 {
    let mut head = Head2::new();
    for (direction, count) in commands {
        head.change_position(direction, count);
    }
    return head.tails.last().unwrap().visited_positions.len() as u32;
}

#[cfg(test)]
mod tests {
    use crate::day9::count_visited_positions;

    #[test]
    fn test_count_visited_positions() {
        let commands = vec![
            ("R", 4),
            ("U", 4),
            ("L", 3),
            ("D", 1),
            ("R", 4),
            ("D", 1),
            ("L", 5),
            ("R", 2),
        ];
        assert_eq!(13, count_visited_positions(commands))
    }
}