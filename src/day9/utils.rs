use std::collections::HashSet;

#[derive(Copy, Clone)]
#[derive(Eq, PartialEq, Hash)]
#[derive(Debug)]
pub(super) struct Position {
    x: i32,
    y: i32,
}
impl Position {
    pub fn new() -> Position {
        return Position { x: 0, y: 0 };
    }
    pub fn from(x: i32, y: i32) -> Position {
        return Position { x, y };
    }

    pub fn get_new_position(&self, direction: &str) -> Position {
        return match direction {
            "U" => Position::from(self.x, self.y + 1),
            "D" => Position::from(self.x, self.y - 1),
            "L" => Position::from(self.x - 1, self.y),
            "R" => Position::from(self.x + 1, self.y),
            _ => {panic!("Unknown direction: {}", direction)}
        }
    }
}

pub(super) struct Head {
    position: Position,
    pub(super) tail: Tail,
}

impl Head {
    pub fn new() -> Box<Head> {
        return Box::from(
            Head {
                position: Position { x: 0, y: 0},
                tail: Tail::new(),
            }
        )
    }

    pub fn change_position(&mut self, direction: &str, count: u32) {
        for _ in 0..count {
            self.position = self.position.get_new_position(direction);
            self.tail.on_head_position_change(self.position);
        }
    }
}

pub(super) struct Tail {
    position: Position,
    pub(super) visited_positions: HashSet<Position>
}
impl Tail {
    pub fn new() -> Tail {
        let starting_position = Position::new();
        return Tail {
            position: starting_position,
            visited_positions: HashSet::from([starting_position])
        };
    }

    fn on_head_position_change(&mut self, head_position: Position) {
        let current_position = self.position.clone();
        if current_position.x.abs_diff(head_position.x)
            + current_position.y.abs_diff(head_position.y) > 2 {
            let mut new_position = current_position;
            if current_position.x < head_position.x {
                new_position = new_position.get_new_position("R");
            } else {
                new_position = new_position.get_new_position("L");
            }
            if current_position.y < head_position.y {
                new_position = new_position.get_new_position("U");
            } else {
                new_position = new_position.get_new_position("D");
            }
            self.position = new_position;
        }
        else if current_position.x.abs_diff(head_position.x) > 1 {
            if current_position.x < head_position.x {
                self.position = current_position.get_new_position("R");
            } else {
                self.position = current_position.get_new_position("L");
            }
        } else if current_position.y.abs_diff(head_position.y) > 1 {
            if current_position.y < head_position.y {
                self.position = current_position.get_new_position("U");
            } else {
                self.position = current_position.get_new_position("D");
            }
        }

        self.visited_positions.insert(self.position.clone());
    }
}