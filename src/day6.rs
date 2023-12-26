use std::collections::HashSet;
use std::fs;

pub fn day6() {
    let content: String = fs::read_to_string("input-aoc-2022-6.txt").unwrap();
    let alma: &str = content.as_str()
        .split("\n")
        .filter(|&x| !x.is_empty())
        .collect::<Vec<&str>>()[0];

    let chars = alma.chars().collect::<Vec<char>>();
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