mod monkey;
mod operation;
mod test;

use std::fs;
use crate::day11::monkey::Monkey;

pub fn day11() {
    let content: String = fs::read_to_string("input-aoc-2022-11.txt").unwrap();
    println!("Advent of Code 2022/11/1: {}", get_monkey_business(content));
}

fn get_monkey_business(content: String) -> i32 {
    let mut monkeys: Vec<Monkey> = content.as_str().split("\n\n")
        .map(Monkey::from)
        .collect();
    let monkeys_count = monkeys.len();
    for _ in 0..20 {
        for i in 0..monkeys_count {
            let mut new_monkeys = monkeys.clone();
            monkeys[i].inspect_and_throw(&mut new_monkeys);
            let current_inspections_count = monkeys[i].inspection_count;
            monkeys = new_monkeys;
            monkeys[i].items.clear();
            monkeys[i].inspection_count = current_inspections_count;
        }
    }

    let mut inspection_count_vec = monkeys.iter()
        .map(|x: &Monkey| x.inspection_count)
        .collect::<Vec<i32>>();
    inspection_count_vec.sort();
    let reverse_inspection_count_vec = inspection_count_vec.iter().rev().collect::<Vec<&i32>>();
    return reverse_inspection_count_vec[0] * reverse_inspection_count_vec[1];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_monkey_business() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

        assert_eq!(10605, get_monkey_business(String::from(input)));
    }

}