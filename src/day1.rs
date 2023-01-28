use std::fs;

pub fn day1() {
    let result = fs::read_to_string("input-aoc-2022-1.txt")
        .expect("File should be readable")
        .split("\n\n")
        .map(|x: &str| x.split("\n")
            .map(|s: &str| s.parse::<i32>().unwrap_or(0))
            .sum::<i32>())
        .collect::<Vec<i32>>();
    println!("Advent of Code 2022/1: {:?}", result.iter().max().unwrap())
}