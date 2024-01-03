use crate::day11::operation::OperationType::{Add, Multiply};

#[derive(Clone)]
pub(super) struct Operation {
    operation_type: OperationType,
    operand2: Option<i32>,
}
impl Operation {
    pub fn from(operation_unparsed: &str) -> Operation {
        let tokens: Vec<&str> = operation_unparsed.trim().split(" ").collect();
        let operation_type = OperationType::from(tokens[4]);
        let operand2 = tokens[5];
        return if operand2 == "old" {
            Operation {
                operation_type,
                operand2: None,
            }
        } else {
            Operation {
                operation_type,
                operand2: Some(operand2.trim().parse::<i32>().unwrap()),
            }
        }
    }

    pub fn apply(&self, value: &i32) -> i32 {
        return if self.operand2.is_some() {
            let operand2_unwrapped = self.operand2.unwrap();
            match self.operation_type {
                Add => value + operand2_unwrapped,
                Multiply => value * operand2_unwrapped
            }
        } else {
            match self.operation_type {
                Add => value + value,
                Multiply => value * value
            }
        }
    }
}

#[derive(Clone)]
enum OperationType {
    Add,
    Multiply
}
impl OperationType {
    pub fn from(operation_type: &str) -> OperationType {
        return match operation_type {
            "+" => Add,
            "*" => Multiply,
            _ => panic!("Unknown OperationType: {}", operation_type)
        }
    }
}