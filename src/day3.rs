use std::collections::HashMap;
use std::fs;

pub fn day3() {
    let content = fs::read_to_string("input-aoc-2022-3.txt")
        .expect("File should be readable");
    let result = content.split_terminator("\n")
        .map(|line: &str| (&(line[..line.len() / 2]),
                           &(line[line.len() / 2..])))
        .map(get_priority_from_duplicate_item)
        .collect::<Vec<i32>>();
    println!("Advent of Code 2022/3: {}", result.iter().sum::<i32>())
}

fn get_priority_from_duplicate_item(tuple: (&str, &str)) -> i32 {
    let map = get_map();
    let (first_half, second_half) = tuple;
    let mut ans: i32 = 0;
    'outer: for letter_first in first_half.chars() {
        for letter_second in second_half.chars() {
            if letter_first == letter_second {
                ans += map[&letter_first];
                break 'outer;
            }
        }
    }
    return ans;
}

fn get_map() -> HashMap<char, i32> {
    let mut map: HashMap<char, i32> = HashMap::new();
    let lowercase_letters = "abcdefghijklmnopqrstuvwxyz";
    for i in 0..26 {
        map.insert(lowercase_letters.chars().nth(i).unwrap(), (i + 1).try_into().unwrap());
    }
    let uppercase_letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    for i in 0..26 {
        map.insert(uppercase_letters.chars().nth(i).unwrap(), (i + 27).try_into().unwrap());
    }
    return map;
}