

type Position = (i32, i32);

pub(crate) struct Head {
    position: Position,
    tail: Tail,
}

impl Head {
    pub fn new() -> Box<Head> {
        return Box::from(
            Head {
                position: (0, 0),
                tail: Tail::new(),
            }
        )
    }

    pub fn change_position(&mut self, direction: &str, count: u32) {
        for _ in 0..count {
            let curr_position = self.position;
            match direction {
                "U" => self.position = (curr_position.0, curr_position.1 + 1),
                "D" => self.position = (curr_position.0, curr_position.1 - 1),
                "L" => self.position = (curr_position.0 - 1, curr_position.1),
                "R" => self.position = (curr_position.0 + 1, curr_position.1),
                _ => {panic!("Unknown direction: {}", direction)}
            }
            self.tail.on_head_position_change(self.position);
        }
    }
}

struct Tail {
    position: Position,
    visited_positions: Vec<Position>
}
impl Tail {
    pub fn new() -> Tail {
        return Tail {
            position: (0, 0),
            visited_positions: vec![]
        };
    }

    fn on_head_position_change(&mut self, position: Position) {
        let curr_pos = self.position;
        if (curr_pos.0.abs_diff(position.0) > 1 && curr_pos.1.abs_diff(position.1) > 1) {

        } else if (curr_pos.0.abs_diff(position.0) > 1) {

        } else if (curr_pos.1.abs_diff(position.1) > 1) {

        } else {
            // do nothing
        }
        self.visited_positions.push(self.position);
    }
}