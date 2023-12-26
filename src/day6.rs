use std::collections::HashSet;
use std::fs;

pub fn day6() {
    let content: String = fs::read_to_string("input-aoc-2022-6.txt").unwrap();
    let datastream: &str = content.as_str()
        .split("\n")
        .filter(|&x| !x.is_empty())
        .collect::<Vec<&str>>()[0];

    let chars = datastream.chars().collect::<Vec<char>>();
    println!("Advent of Code 2022/6/1: {}", find_packet_start_index(chars))
}

fn find_packet_start_index(chars: Vec<char>) -> i32 {
    for i in 0..(chars.len() - 4) {
        let slice = &chars[i..(i+4)];
        let set: HashSet<&char> = HashSet::from_iter(slice);
        if set.len() == 4 {
            return (i + 4) as i32;
        }
    }
    return 0;
}

pub fn day6_part2() {
    let content: String = fs::read_to_string("input-aoc-2022-6.txt").unwrap();
    let datastream: &str = content.as_str()
        .split("\n")
        .filter(|&x| !x.is_empty())
        .collect::<Vec<&str>>()[0];

    let chars = datastream.chars().collect::<Vec<char>>();
    println!("Advent of Code 2022/6/2: {}", find_packet_start_index_part2(chars))
}

fn find_packet_start_index_part2(chars: Vec<char>) -> i32 {
    for i in 0..(chars.len() - 14) {
        let slice = &chars[i..(i+14)];
        let set: HashSet<&char> = HashSet::from_iter(slice);
        if set.len() == 14 {
            return (i + 14) as i32;
        }
    }
    return 0;
}