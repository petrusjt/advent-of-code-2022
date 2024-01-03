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
            // what I wanted:
            // monkeys[i].inspect_and_throw(&mut monkeys, true);

            // what I had to do:
            //  reason: for a value only 1 mutable reference can be present at any given time
            let mut new_monkeys = monkeys.clone();
            monkeys[i].inspect_and_throw(&mut new_monkeys, true);
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

pub fn day11_part2() {
    let content: String = fs::read_to_string("input-aoc-2022-11.txt").unwrap();
    println!("Advent of Code 2022/11/1: {}", get_monkey_business_part2(content));
}

fn get_monkey_business_part2(content: String) -> i128 {
    if true {
        // The logic probably works, but I can't figure out a way to keep worry levels from overflowing i128
        //  without altering the result
        return -1;
    }
    let mut monkeys: Vec<Monkey> = content.as_str().split("\n\n")
        .map(Monkey::from)
        .collect();
    let monkeys_count = monkeys.len();
    for _ in 0..10000 {
        for i in 0..monkeys_count {
            let mut new_monkeys = monkeys.clone();
            monkeys[i].inspect_and_throw(&mut new_monkeys, false);
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
    return i128::from(*reverse_inspection_count_vec[0]) * i128::from(*reverse_inspection_count_vec[1]);
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

    #[test]
    fn test_get_monkey_business_part2() {
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

        assert_eq!(2713310158i128, get_monkey_business_part2(String::from(input)));
    }

}