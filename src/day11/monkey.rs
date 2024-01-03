use crate::day11::operation::Operation;
use crate::day11::test::Test;

#[derive(Clone)]
pub(super) struct Monkey {
    pub(super) items: Vec<i32>,
    operation: Operation,
    test: Test,
    pub(super) inspection_count: i32,
}
impl Monkey {
    pub fn from(unparsed: &str) -> Monkey {
        let split_unparsed: Vec<&str> = unparsed.split("\n").collect();
        return Monkey {
            items: split_unparsed[1].split(":")
                .collect::<Vec<&str>>()[1].trim().split(",")
                .map(|y| y.trim().parse::<i32>().unwrap())
                .collect(),
            operation: Operation::from(split_unparsed[2]),
            test: Test::from(split_unparsed[3..].to_vec()),
            inspection_count: 0
        };
    }

    pub fn inspect_and_throw(&mut self, monkeys: &mut Vec<Monkey>) {
        let items_owned = self.items.to_owned();
        for worry_level in items_owned {
            let new_worry_level = self.operation.apply(&worry_level) / 3;
            self.inspection_count += 1;
            let throw_index = self.test.apply(&new_worry_level);
            monkeys[throw_index as usize].items.push(new_worry_level);
        }
    }
}