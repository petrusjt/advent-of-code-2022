use crate::day11::operation::Operation;
use crate::day11::test::Test;

#[derive(Clone)]
pub(super) struct Monkey {
    pub(super) items: Vec<i128>,
    operation: Operation,
    pub(super) test: Test,
    pub(super) inspection_count: i32,
}
impl Monkey {
    pub fn from(unparsed: &str) -> Monkey {
        let split_unparsed: Vec<&str> = unparsed.split("\n").collect();
        return Monkey {
            items: split_unparsed[1].split(":")
                .collect::<Vec<&str>>()[1].trim().split(",")
                .map(|y| y.trim().parse::<i128>().unwrap())
                .collect(),
            operation: Operation::from(split_unparsed[2]),
            test: Test::from(split_unparsed[3..].to_vec()),
            inspection_count: 0
        };
    }

    pub fn inspect_and_throw(&mut self, monkeys: &mut Vec<Monkey>, is_part_1: bool, modulo: i32) {
        let items_owned = self.items.to_owned();
        for worry_level in items_owned {
            let new_worry_level = if is_part_1 {
                self.operation.apply(&worry_level) / 3
            } else {
                self.operation.apply(&worry_level) % i128::from(modulo)
            };
            self.inspection_count += 1;
            let throw_index = self.test.apply(&new_worry_level);
            monkeys[throw_index as usize].items.push(new_worry_level);
        }
    }
}