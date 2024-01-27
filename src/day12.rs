use std::fs;
use std::ptr::eq;

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
    path_length: i32,
}

impl Position {
    pub fn of_tuple(pos: (i32, i32)) -> Position {
        return Position { x: pos.0, y: pos.1, path_length: 0 };
    }

    pub fn up(&self) -> Position {
        return Position { x: self.x, y: self.y - 1, path_length: self.path_length + 1 };
    }

    pub fn down(&self) -> Position {
        return Position { x: self.x, y: self.y + 1, path_length: self.path_length + 1 };
    }

    pub fn left(&self) -> Position {
        return Position { x: self.x - 1, y: self.y, path_length: self.path_length + 1 };
    }

    pub fn right(&self) -> Position {
        return Position { x: self.x + 1, y: self.y, path_length: self.path_length + 1 };
    }
}
impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }

    fn ne(&self, other: &Self) -> bool {
        return !eq(self, other);
    }
}
impl Default for Position {
    fn default() -> Self {
        return Position { x: -1, y: -1, path_length: 0 };
    }
}

pub fn day12() {
    let content: String = fs::read_to_string("input-aoc-2022-12.txt").unwrap();
    let heightmap: Vec<Vec<char>> = content.split("\n")
        .map(|line| line.chars().collect())
        .collect();
    println!("Advent of Code 2022/12/1: {}", get_shortest_path(heightmap));
}

fn get_shortest_path(heightmap: Vec<Vec<char>>) -> i32 {
    let starting_position = get_start_position(&heightmap, 'S').expect("Starting point: ");
    let mut to_visit_positions: Vec<Position> = vec![];
    let mut visited_positions: Vec<Position> = vec![];
    to_visit_positions.push(starting_position);

    while !to_visit_positions.is_empty() {
        let current_pos = to_visit_positions.remove(0);

        if heightmap[current_pos.y as usize][current_pos.x as usize] == 'E' {
            return current_pos.path_length;
        }

        if !visited_positions.contains(&current_pos) {
            if can_move(&heightmap, &current_pos, &current_pos.up()) {
                to_visit_positions.push(current_pos.up());
            }
            if can_move(&heightmap, &current_pos, &current_pos.down()) {
                to_visit_positions.push(current_pos.down());
            }
            if can_move(&heightmap, &current_pos, &current_pos.left()) {
                to_visit_positions.push(current_pos.left());
            }
            if can_move(&heightmap, &current_pos, &current_pos.right()) {
                to_visit_positions.push(current_pos.right());
            }
        }

        visited_positions.push(current_pos);
    }
    return -1;
}

fn get_start_position(heightmap: &Vec<Vec<char>>, searched_char: char) -> Result<Position, &str> {
    let mut start_pos: Position = Position::default();
    for i in 0..heightmap.len() {
        for j in 0..heightmap[i].len() {
            if heightmap[i][j] == searched_char {
                start_pos = Position::of_tuple((j as i32, i as i32));
                break;
            }
        }
    }
    return if start_pos == Position::default() {
        Err("No starting point defined!")
    } else {
        Ok(start_pos)
    };
}

fn can_move(heightmap: &Vec<Vec<char>>, current_position: &Position, checked_position: &Position) -> bool {
    if !(checked_position.y > 0 && checked_position.y < heightmap.len() as i32 - 1
        && checked_position.x > 0 && checked_position.x < heightmap[checked_position.y as usize].len() as i32 - 1) {
        return false;
    }

    let mut checked_char = heightmap[checked_position.y as usize][checked_position.x as usize];
    if checked_char == 'E' {
        checked_char = 'z';
    }
    let mut current_char = heightmap[current_position.y as usize][current_position.x as usize];
    if current_char == 'S' {
        current_char = 'a';
    }
    if checked_char <= heightmap[current_position.y as usize][current_position.x as usize] {
        return true;
    }
    if checked_char as u8 == current_char as u8 + 1u8 {
        return true;
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_shortest_path_1() {
        let heightmap = vec![
            vec!['S', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'],
            vec!['w', 'x', 'x', 'x', 'y', 'z', 'z', 'E', 'z', 'z', 'z', 'l'],
            vec!['v', 'u', 'v', 'u', 't', 's', 'r', 'q', 'p', 'o', 'n', 'm'],
        ];

        assert_eq!(32, get_shortest_path(heightmap))
    }

    #[test]
    fn test_get_shortest_path_2() {
        let heightmap = vec![
            vec!['S', 'a', 'b', 'q', 'p', 'o', 'n', 'm'],
            vec!['a', 'b', 'c', 'r', 'y', 'x', 'x', 'l'],
            vec!['a', 'c', 'c', 's', 'z', 'E', 'x', 'k'],
        ];

        assert_eq!(31, get_shortest_path(heightmap))
    }
}